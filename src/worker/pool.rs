use std::{error::Error, sync::Arc};

use tokio::{sync::{mpsc, Mutex}, task::JoinHandle};
use tracing::{error, info};

use crate::{broker::traits::Broker, core::TaskError, storage::Storage};

use super::{executor::Executor, registry::TaskRegistry};

pub struct WorkerPool {
    workers: Vec<JoinHandle<()>>,
    broker: Arc<dyn Broker>,
    storage: Arc<dyn Storage>,
    registry: Arc<TaskRegistry>,
    concurrency: usize,
    shutdown_tx: mpsc::Sender<()>,
    shutdown_rx: Arc<Mutex<mpsc::Receiver<()>>>,
}

impl WorkerPool {
    pub fn new(
        broker: Arc<dyn Broker>, 
        storage: Arc<dyn Storage>,
        registry: Arc<TaskRegistry>,
        concurrency: usize
    ) -> Self {
        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

        Self {
            workers: Vec::new(),
            broker,
            storage,
            registry,
            concurrency,
            shutdown_tx,
            shutdown_rx: Arc::new(Mutex::new(shutdown_rx)),
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
        let shutdown_rx = Arc::clone(&self.shutdown_rx);

        tokio::spawn(async move {
            loop {
                let mut shutdown_rx = shutdown_rx.lock().await;
                tokio::select! {
                    _ =  shutdown_rx.recv() => {
                        println!("Worker is shutting down");
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
        self.shutdown_tx.send(()).await?;

        // Wait for all workers to complete
        for worker in self.workers.drain(..) {
            worker.await?;
        }

        Ok(())
    }
}