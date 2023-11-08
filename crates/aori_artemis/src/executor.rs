use crate::types::Executor;
use anyhow::Result;
use aori_requests::aori_provider::AoriProvider;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};
/// An executor that sends requests to the Aori Provider.
pub struct AoriExecutor {
    aori_provider: Arc<Mutex<AoriProvider>>,
}

impl AoriExecutor {
    pub fn new(aori_provider: Arc<Mutex<AoriProvider>>) -> Self {
        Self { aori_provider }
    }
}

#[async_trait]
impl Executor<serde_json::Value> for AoriExecutor {
    /// Send requests to the Aori Provider.
    async fn execute(&mut self, action: serde_json::Value) -> Result<()> {
        info!("Received request: {:?}", action);
        let result = {
            let mut provider = self.aori_provider.lock().await;
            info!("Sending payload: {}", action.clone());
            provider.send(action.clone()).await
        };
        match result {
            Ok(_) => {
                info!("Request sent successfully");
                Ok(())
            }
            Err(e) => {
                error!("Request error: {}", e);
                Err(anyhow::anyhow!("Request error: {}", e))
            }
        }
    }
}
