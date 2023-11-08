use aori::artemis::collector::AoriCollector;
use aori::requests::aori_provider::AoriProvider;
use aori::types::events::AoriEvent;

use artemis_core::types::Collector;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

// use crate::collector::AoriCollector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = AoriProvider::new_from_env()
        .await
        .expect("failed to initialise aori provider");
    let provider = Arc::new(Mutex::new(provider));
    let collector = AoriCollector::new(provider);

    // Getting the event stream from the collector
    let mut stream = collector.get_event_stream().await?;

    // Iterate over the stream and print each event
    while let Some(event) = stream.next().await {
        match event {
            // can filter based on event types
            AoriEvent::Subscribed(message) => {
                println!("{}", message);
            }
            // Add other match arms here for other AoriEvent variants if necessary
            _ => {
                println!("{:?}", event);
            }
        }
    }

    Ok(())
}
