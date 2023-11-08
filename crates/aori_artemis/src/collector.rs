use anyhow::Result;
use aori_requests::aori_provider::AoriProvider;
use aori_types::events::{AoriEvent, AoriResponse};
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use websockets::Frame;

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
        let mut provider = self.provider.lock().await;
        provider
            .subscribe_orderbook()
            .await
            .expect("Failed to subscribe to orderbook");

        let (tx, rx) = mpsc::unbounded_channel();

        // Spawning a task to manage the responses
        tokio::spawn({
            let provider = Arc::clone(&self.provider);
            let tx = tx.clone();
            async move {
                loop {
                    let mut provider = provider.lock().await;
                    match provider.feed_conn.receive().await {
                        Ok(response) => {
                            if let Frame::Text { payload, .. } = response {
                                // Process the payload here and send the events to the channel
                                // This logic might depend on your specific needs
                                if let Err(e) = process_payload(payload, &tx).await {
                                    eprintln!("Error processing payload: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error receiving response: {}", e);
                        }
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
            "[AORI.IO] Subscribed to orderbook updates".to_string(),
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
