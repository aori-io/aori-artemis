use aori_artemis::collector::AoriCollector;
use aori_requests::aori_provider::AoriProvider;

use aori_artemis::engine::Engine;
use aori_artemis::executor::AoriExecutor;
use aori_artemis::types::ExecutorMap;

use artemis_core::types::CollectorMap;
use simple_arb::{
    strategy::SimpleArb,
    types::{Action, Event},
};
use std::sync::Arc;
use tokio::sync::Mutex;
// use crate::collector::AoriCollector;

use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = filter::Targets::new()
        .with_target("aori_artemis", Level::INFO)
        .with_target("simple_arb", Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let provider = AoriProvider::new_from_env()
        .await
        .expect("failed to initialise aori provider");
    let provider2 = AoriProvider::new_from_env()
        .await
        .expect("failed to initialise aori provider");
    let wallet_is = provider.wallet.clone().expect("msg");
    let last_id_is = provider.last_id.clone();
    let api_key_is = std::env::var("API_KEY").expect("API_KEY not found in environment");

    let provider = Arc::new(Mutex::new(provider));
    let provider2 = Arc::new(Mutex::new(provider2));

    let mut engine: Engine<Event, Action> = Engine::default();

    let collector = Box::new(AoriCollector::new(provider.clone()));

    let collector = CollectorMap::new(collector, Event::AoriTransaction);

    engine.add_collector(Box::new(collector));

    // Set up strategy.
    let strategy = SimpleArb::new(wallet_is, last_id_is, api_key_is);
    engine.add_strategy(Box::new(strategy));

    let executor = Box::new(AoriExecutor::new(provider2));
    let executor = ExecutorMap::new(executor, |action| match action {
        Action::SendAoriPayload(payload) => Some(payload),
    });
    engine.add_executor(Box::new(executor));

    // Start engine.
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("res: {:?}", res);
        }
    }

    Ok(())
}
