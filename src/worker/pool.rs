use std::{error::Error, sync::Arc, time::Duration};

use tokio::{sync::broadcast, task::JoinHandle};
use tracing::{error, info};

use crate::{broker::traits::Broker, core::TaskError, storage::Storage};

use super::{executor::Executor, registry::TaskRegistry};

pub struct WorkerPool {
    workers: Vec<JoinHandle<()>>,
    broker: Arc<dyn Broker>,
    storage: Arc<dyn Storage>,
    registry: Arc<TaskRegistry>,
    concurrency: usize,
    shutdown_tx: broadcast::Sender<()>,
}

impl WorkerPool {
    pub fn new(
        broker: Arc<dyn Broker>, 
        storage: Arc<dyn Storage>,
        registry: Arc<TaskRegistry>,
        concurrency: usize
    ) -> Self {
        let (shutdown_tx, _) = broadcast::channel(concurrency);

        Self {
            workers: Vec::new(),
            broker,
            storage,
            registry,
            concurrency,
            shutdown_tx,
        }
    }

    pub async fn start(&mut self) -> Result<(), TaskError> {

        for _ in 0..self.concurrency {
            let worker = self.spawn_worker(
                Arc::clone(&self.broker), 
                Arc::clone(&self.storage),
                Arc::clone(&self.registry),
            );
            self.workers.push(worker);
        }

        Ok(())
    }

    fn spawn_worker(
        &self,
        broker: Arc<dyn Broker>,
        storage: Arc<dyn Storage>,
        registry: Arc<TaskRegistry>,
    ) -> JoinHandle<()> {
        let mut shutdown_rx = self.shutdown_tx.subscribe();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok(_) = shutdown_rx.recv() => {
                        info!("Worker is shutting down");
                        break;
                    }
                    task = broker.pop() => {
                        match task {
                            Ok(Some(task)) => {
                                let executor = Executor::new(
                                    Arc::clone(&broker),
                                    Arc::clone(&storage),
                                    Arc::clone(&registry),
                                );

                                let result = executor.execute_task(task).await;
                                if let Err(e) = result {
                                    error!("Failed to execute task: {:?}", e);
                                }
                            }
                            Ok(None) => {
                                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            }
                            Err(e) => {
                                error!("Failed to pop task from broker: {:?}", e);
                            }
                        }
                    }
                }
            }
        })
    }

    /// Graceful shutdown of worker pool
    pub async fn shutdown(&mut self) -> Result<(), Box<dyn Error>>{
        // Send shutdown signal to all workers
        let _ = self.shutdown_tx.send(());

        let timeout = Duration::from_secs(10);

        // Wait for all workers to complete
        let shutdown_future = async {
            for worker in self.workers.drain(..) {
                worker.await?;
            }
            Ok::<_, Box<dyn Error>>(())
        };

        tokio::select! {
            result = shutdown_future => result,
            _ = tokio::time::sleep(timeout) => {
                error!("Worker pool shutdown timed out");
                Err("Shutdown timeout".into())
            }
        }
    }
}