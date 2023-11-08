use aori_types::events::AoriEvent;
use serde_json::Value;

/// Core Event enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Event {
    AoriTransaction(AoriEvent),
}

/// Core Action enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Action {
    SendAoriPayload(Value),
    // SubmitTx(SubmitTxToMempool),
}
