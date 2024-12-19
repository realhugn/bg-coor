use std::collections::HashMap;
use std::path::PathBuf;
use async_trait::async_trait;
use bg_coor::{core::TaskError, task_manager::TaskManager, worker::registry::TaskHandler};
use serde_json::{json, Value};
use yara::Compiler;

pub struct YaraScanTaskHandler;

#[async_trait]
impl TaskHandler for YaraScanTaskHandler {
    async fn handle(
        &self,
        args: Vec<Value>,
        _kwargs: HashMap<String, Value>,
    ) -> Result<Vec<u8>, TaskError> {
        // Get file path and rules from arguments
        let file_path = args.get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| TaskError::InvalidArgument("File path missing".into()))?;
        
        let yara_rule = args.get(1)
            .and_then(|v| v.as_str())
            .ok_or_else(|| TaskError::InvalidArgument("YARA rule missing".into()))?;

        // Compile YARA rule
        let mut compiler = Compiler::new().map_err(|e| TaskError::Other(e.to_string()))?;
        compiler = compiler.add_rules_str(yara_rule).map_err(|e| TaskError::Other(e.to_string()))?;
        let rules = compiler.compile_rules().map_err(|e| TaskError::Other(e.to_string()))?;

        // Scan file
        let path = PathBuf::from(file_path);
        let matches = rules.scan_file(path, 3).map_err(|e| TaskError::Other(e.to_string()))?;

        // Format results
        let result = matches
            .iter()
            .map(|m| m.identifier.to_string())
            .collect::<Vec<_>>()
            .join(",");

        Ok(result.into_bytes())
    }
}

// Usage example
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut task_manager = TaskManager::builder(2).build();
    task_manager.register_handler("yara_scan", YaraScanTaskHandler)?;
    task_manager.start().await?;

    let yara_rule = r#"
        rule contains_password {
            strings:
                $pwd = "password" nocase
            condition:
                $pwd
        }
    "#;

    let args = vec![
        json!("/home/realhugn/PlayGround/rust-learning/bg-coor/examples/yara_scan/example_scan.txt"),
        json!(yara_rule),
    ];

    let signature = bg_coor::core::TaskSignature {
        name: "yara_scan".to_string(),
        args,
        kwargs: Default::default(),
    };
    
    let task_id = task_manager.enqueue_task(signature, 3).await?;


    println!("Task ID: {}", task_id);

    loop {
        let task = task_manager.get_task(task_id).await?;
        if let Some(task) = task {
            if task.is_finished() {
                println!("Task finished");
                let result = task.get_result().unwrap();
                println!("Task result: {}", String::from_utf8(result.to_vec())?);
                break;
            }
        }
    }

    Ok(())
}