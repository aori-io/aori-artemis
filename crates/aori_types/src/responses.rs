use serde::{Deserialize, Serialize};

use crate::events::OrderCreatedData;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct AoriViewOrderbookResponse {
    pub id: Option<u64>,
    pub result: AoriOrderbookData,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct AoriOrderbookData {
    pub orders: Vec<OrderCreatedData>,
}
