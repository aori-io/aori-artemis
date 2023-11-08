use anyhow::Result;
use aori_requests::aori_provider::AoriProvider;
use aori_types::events::{AoriEvent, AoriResponse};
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::StreamExt;
pub struct AoriCollector {
    provider: Arc<Mutex<AoriProvider>>,
}

impl AoriCollector {
    pub fn new(provider: Arc<Mutex<AoriProvider>>) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl Collector<AoriEvent> for AoriCollector {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, AoriEvent>> {
        let provider = Arc::clone(&self.provider);
        let mut locked_provider = provider.lock().await;
        locked_provider
            .subscribe_orderbook()
            .await
            .map_err(|_| anyhow::anyhow!("Failed to subscribe orderbook."))?;

        let (tx, rx) = mpsc::unbounded_channel();

        // Spawning a task to manage the responses
        tokio::spawn({
            let provider = Arc::clone(&provider);
            let tx = tx.clone();
            async move {
                while let Some(result) = provider.lock().await.request_conn.next().await {
                    match result {
                        Ok(message) => {
                            let message = message.into_text().expect("msg");
                            if let Err(e) = process_payload(message, &tx).await {
                                eprintln!("Error processing payload {}", e);
                            }
                        }
                        Err(e) => eprintln!("Error receiving message: {}", e),
                    }
                }
                while let Some(result) = provider.lock().await.feed_conn.next().await {
                    match result {
                        Ok(message) => {
                            let message = message.into_text().expect("msg");
                            if let Err(e) = process_payload(message, &tx).await {
                                eprintln!("Error processing payload {}", e);
                            }
                        }
                        Err(e) => eprintln!("Error receiving message: {}", e),
                    }
                }
            }
        });

        // Convert the receiver into a Stream
        let stream = UnboundedReceiverStream::new(rx);

        Ok(Box::pin(stream))
    }
}

async fn process_payload(payload: String, tx: &UnboundedSender<AoriEvent>) -> Result<()> {
    if payload.contains("Subscribed to orderbook updates") {
        tx.send(AoriEvent::Subscribed(
            "if you're reading this you have subscribed to aori thx".to_string(),
        ))
        .expect("Error sending subscription confirmation");
    } else {
        match serde_json::from_str::<AoriResponse>(&payload) {
            Ok(response) => {
                tx.send(response.result)
                    .expect("Error sending response event");
            }
            Err(e) => {
                eprintln!("Failed to deserialize the message: {}", e);
            }
        }
    }
    Ok(())
}
