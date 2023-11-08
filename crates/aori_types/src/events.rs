use chrono::Utc;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{
    constants::{DEFAULT_CONDUIT_KEY, DEFAULT_ORDER_ADDRESS, DEFAULT_ZONE_HASH},
    seaport,
};

use alloy_primitives::{Address, U256};

// Struct representing the outermost layer of JSON
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct AoriResponse {
    pub id: Option<u64>,
    pub result: AoriEvent,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct OrderCreationData {
    pub parameters: OrderParameters,
    pub signature: String,
}

// structs

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct OrderParameters {
    pub offerer: String,
    pub zone: String,
    pub offer: Vec<OfferItem>,
    pub consideration: Vec<ConsiderationItem>,
    #[serde(rename = "orderType")]
    pub order_type: u8,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "zoneHash")]
    pub zone_hash: String,
    pub salt: String,
    #[serde(rename = "conduitKey")]
    pub conduit_key: String,
    #[serde(rename = "totalOriginalConsiderationItems")]
    pub total_original_consideration_items: i16,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct OfferItem {
    #[serde(rename = "itemType")]
    pub item_type: u8,
    pub token: String,
    #[serde(rename = "identifierOrCriteria")]
    pub identifier_or_criteria: String,
    #[serde(rename = "startAmount")]
    pub start_amount: String,
    #[serde(rename = "endAmount")]
    pub end_amount: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ConsiderationItem {
    #[serde(rename = "itemType")]
    pub item_type: u8,
    pub token: String,
    #[serde(rename = "identifierOrCriteria")]
    pub identifier_or_criteria: String,
    #[serde(rename = "startAmount")]
    pub start_amount: String,
    #[serde(rename = "endAmount")]
    pub end_amount: String,
    pub recipient: String,
}

// impl

impl OrderParameters {
    // initialises the struct with default values
    pub fn load_default_order_parameters(wallet: String) -> Self {
        Self {
            offerer: wallet.clone(),
            zone: DEFAULT_ORDER_ADDRESS.to_string(),
            offer: vec![OfferItem::new(
                1,
                "".to_string(),
                "0".to_string(),
                "".to_string(),
                "".to_string(),
            )],
            consideration: vec![ConsiderationItem::new(
                1,
                "".to_string(),
                "0".to_string(),
                "".to_string(),
                "".to_string(),
                wallet,
            )],
            order_type: 3,
            start_time: "".to_string(),
            end_time: "".to_string(),
            zone_hash: "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            salt: "0".to_string(),
            conduit_key: "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            total_original_consideration_items: 1,
        }
    }

    pub fn to_order_components(self) -> seaport::OrderComponents {
        seaport::OrderComponents {
            // offerer: Address::parse_checksummed(&self.offerer, None).unwrap(),
            offerer: self.offerer.parse::<Address>().unwrap(),
            zone: self.zone.parse::<Address>().unwrap(),
            // zone: Address::parse_checksummed(&self.zone, None).unwrap(),
            offer: self
                .offer
                .iter()
                .map(|item| seaport::OfferItem {
                    itemType: item.item_type,
                    token: item.token.parse::<Address>().unwrap(),
                    // token: Address::parse_checksummed(&item.token, None).unwrap(),
                    identifierOrCriteria: U256::from(0),
                    startAmount: U256::from(item.start_amount.parse::<U256>().unwrap()),
                    endAmount: U256::from(item.end_amount.parse::<U256>().unwrap()),
                })
                .collect(),
            consideration: self
                .consideration
                .iter()
                .map(|item| seaport::ConsiderationItem {
                    itemType: item.item_type,
                    // token: Address::parse_checksummed(&item.token, None).unwrap(),
                    token: item.token.parse::<Address>().unwrap(),
                    identifierOrCriteria: U256::from(0),
                    startAmount: U256::from(item.start_amount.parse::<u64>().unwrap()),
                    endAmount: U256::from(item.end_amount.parse::<u64>().unwrap()),
                    recipient: item.recipient.parse::<Address>().unwrap(),
                    // recipient: Address::parse_checksummed(&item.recipient, None).unwrap(),
                })
                .collect(),
            orderType: self.order_type,
            startTime: U256::from(self.start_time.parse::<U256>().unwrap()),
            endTime: U256::from(self.end_time.parse::<U256>().unwrap()),
            zoneHash: DEFAULT_ZONE_HASH.into(),
            salt: U256::from_str_radix(self.salt.trim_start_matches("0x"), 16).unwrap(),
            conduitKey: DEFAULT_CONDUIT_KEY.into(),
            counter: U256::from(0), // @dev: to-do: query seaport for counter here
        }
    }

    // creates a limit order for erc20 to erc20 trade
    pub fn limit_order(
        wallet: &str,
        sell_token: &str,
        sell_amount: &str,
        buy_token: &str,
        buy_amount: &str,
    ) -> Self {
        let mut order = Self::load_default_order_parameters(wallet.to_string());

        let start_time = Utc::now().timestamp_millis();
        let end_time = start_time + 1000 * 60 * 60 * 24; // 24 hours later

        order.start_time = start_time.to_string();
        order.end_time = end_time.to_string();

        order.offer = vec![OfferItem {
            item_type: 1,
            token: sell_token.to_string(),
            identifier_or_criteria: "0".to_string(), // Replace "criteria" with your criteria
            start_amount: sell_amount.to_string(),
            end_amount: sell_amount.to_string(),
        }];

        order.consideration = vec![ConsiderationItem {
            item_type: 1,
            token: buy_token.to_string(),
            identifier_or_criteria: "0".to_string(), // Replace "criteria" with your criteria
            start_amount: buy_amount.to_string(),
            end_amount: buy_amount.to_string(),
            recipient: wallet.to_string(),
        }];

        order
    }
}
impl OfferItem {
    pub fn new(
        item_type: u8,
        token: String,
        identifier_or_criteria: String,
        start_amount: String,
        end_amount: String,
    ) -> Self {
        Self {
            item_type,
            token,
            identifier_or_criteria,
            start_amount,
            end_amount,
        }
    }
}

impl ConsiderationItem {
    pub fn new(
        item_type: u8,
        token: String,
        identifier_or_criteria: String,
        start_amount: String,
        end_amount: String,
        recipient: String,
    ) -> Self {
        Self {
            item_type,
            token,
            identifier_or_criteria,
            start_amount,
            end_amount,
            recipient,
        }
    }
}

#[derive(Clone, Serialize, PartialEq, Debug)]
pub enum AoriEvent {
    #[serde(rename = "Subscribed")]
    Subscribed(String),

    #[serde(rename = "OrderCancelled")]
    OrderCancelled(Box<OrderCancelledData>),

    #[serde(rename = "OrderCreated")]
    OrderCreated(Box<OrderCreatedData>),

    #[serde(rename = "OrderTaken")]
    OrderTaken(Box<OrderTakenData>),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct OrderCreatedData {
    pub order: OrderCreationData,
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    #[serde(rename = "inputToken")]
    pub input_token: String,
    #[serde(rename = "outputToken")]
    pub output_token: String,
    #[serde(rename = "inputAmount")]
    pub input_amount: u64,
    #[serde(rename = "outputAmount")]
    pub output_amount: u64,
    #[serde(rename = "chainId")]
    pub chain_id: i64,
    pub active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: u64,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub rate: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct OrderCancelledData {
    pub order: OrderCreationData,
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    #[serde(rename = "inputToken")]
    pub input_token: String,
    #[serde(rename = "outputToken")]
    pub output_token: String,
    #[serde(rename = "inputAmount")]
    pub input_amount: u64,
    #[serde(rename = "outputAmount")]
    pub output_amount: u64,
    #[serde(rename = "chainId")]
    pub chain_id: i64,
    pub active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: u64,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct OrderTakenData {
    pub order: OrderCreationData,
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    #[serde(rename = "inputToken")]
    pub input_token: String,
    #[serde(rename = "outputToken")]
    pub output_token: String,
    #[serde(rename = "inputAmount")]
    pub input_amount: u64,
    #[serde(rename = "outputAmount")]
    pub output_amount: u64,
    #[serde(rename = "chainId")]
    pub chain_id: i64,
    pub active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: u64,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "takenAt")]
    pub taken_at: u64,
}

// Implementing Deserialize trait manually for AoriEvent
impl<'de> Deserialize<'de> for AoriEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;

        // Getting the type value
        let type_ = map
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| serde::de::Error::missing_field("type"))?;

        // Handling deserialization based on the type value
        match type_ {
            "Subscribed to orderbook updates" => {
                Ok(AoriEvent::Subscribed("Subscribed.".to_string()))
            }
            "OrderCancelled" => {
                let data: OrderCancelledData = serde_json::from_value(map.remove("data").unwrap())
                    .map_err(serde::de::Error::custom)?;
                Ok(AoriEvent::OrderCancelled(Box::new(data)))
            }
            "OrderCreated" => {
                let data: OrderCreatedData = serde_json::from_value(map.remove("data").unwrap())
                    .map_err(serde::de::Error::custom)?;
                Ok(AoriEvent::OrderCreated(Box::new(data)))
            }
            "OrderTaken" => {
                let data: OrderTakenData = serde_json::from_value(map.remove("data").unwrap())
                    .map_err(serde::de::Error::custom)?;
                Ok(AoriEvent::OrderTaken(Box::new(data)))
            }
            _ => Err(serde::de::Error::unknown_variant(
                type_,
                &[
                    "OrderCancelled",
                    "OrderCreated",
                    "OrderTaken",
                    "Subscribed to orderbook updates",
                ],
            )),
        }
    }
}
