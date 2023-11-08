// strategy that is:
// - listening to new orders on Aori Orderbook
// - for every new order, check whether it is possible to arb the order through aori orderbook
// (until partial fills implemented, check whether there is an exact same amount on the other side of the trade)
// - if there is an arb opportunity, match the two legs -> buy order 1 and sell order 2 simultaneously
// - wait for confirmation, if prompted with a OrderToSettle event on the collector, settle.

// first implementation only implements weth and usdc test tokens on goerli

use super::types::{Action, Event};
use anyhow::Error;
use aori_requests::requests::create_take_order_payload;
use aori_types::events::{AoriEvent, OrderCreatedData};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use ethers::prelude::{k256::ecdsa::SigningKey, Wallet};

use std::sync::Arc;
use std::sync::Mutex;
use tracing::info;

#[derive(Debug, Clone)]
pub struct TokenEntry<'a> {
    /// Address of the v2 pool.
    pub address: &'a str,
    /// Whether the pool has weth as token0.
    pub ticker: &'a str,
    pub chain_id: u64,
}
impl<'a> PartialEq for TokenEntry<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.chain_id == other.chain_id
    }
}

#[derive(Debug, Clone)]
pub struct SimpleArb {
    pub token_list: Vec<TokenEntry<'static>>,
    pub orderbook_entries: Vec<OrderCreatedData>,
    pub wallet: Wallet<SigningKey>,
    pub last_id: Arc<Mutex<u64>>,
    pub api_key: String,
}

impl SimpleArb {
    /// Create a new instance of the strategy.
    pub fn new(wallet: Wallet<SigningKey>, last_id: Arc<Mutex<u64>>, api_key: String) -> Self {
        let _token_list: Vec<TokenEntry> = vec![
            TokenEntry {
                address: "0xD3664B5e72B46eaba722aB6f43c22dBF40181954",
                ticker: "usdc",
                chain_id: 5,
            },
            TokenEntry {
                address: "0x2715Ccea428F8c7694f7e78B2C89cb454c5F7294",
                ticker: "weth",
                chain_id: 5,
            },
        ];
        info!(
            "relevant token: {} wiht chain id {}",
            _token_list[0].address, _token_list[0].chain_id
        );
        info!(
            "relevant token: {} with chain id {}",
            _token_list[1].address, _token_list[1].chain_id
        );
        Self {
            token_list: _token_list,
            orderbook_entries: Vec::new(),
            wallet,
            last_id,
            api_key,
        }
    }
    fn is_token_relevant(&self, token_address: &str, chain_id: u64) -> bool {
        self.token_list.iter().any(|token_entry| {
            token_entry.address == token_address && token_entry.chain_id == chain_id
        })
    }
}
// don't do anything at startup
#[async_trait]
impl Strategy<Event, Action> for SimpleArb {
    async fn sync_state(&mut self) -> Result<(), Error> {
        // TODO: view orderbook for all the combination of all tokens on both sides and store the orders in the structure as defined above
        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::AoriTransaction(aori_event) => {
                info!("Received a new aori event: {:?}", aori_event);

                match aori_event {
                    AoriEvent::OrderCreated(order_data) => {
                        let order_data = *order_data;
                        info!(
                            "OrderCreated event with input address: {:?} and chain id {:?}",
                            order_data.input_token, order_data.chain_id
                        );

                        // check if token relevant
                        if self
                            .is_token_relevant(&order_data.input_token, order_data.chain_id as u64)
                            || self.is_token_relevant(
                                &order_data.output_token,
                                order_data.chain_id as u64,
                            )
                        // if it is, look whether there is a trade to be done -> i.e. look for whether there is a matching order
                        {
                            info!(
                                "new order stored into the memory: {}",
                                order_data.order_hash
                            );
                            self.orderbook_entries.push(order_data.clone());

                            // Look for matching entries with the same input and output token pair
                            let mut matching_orders = self
                                .orderbook_entries
                                .iter()
                                .filter(|entry| {
                                    entry.input_token == order_data.output_token
                                        && entry.output_token == order_data.input_token
                                        && entry.input_amount > order_data.output_amount
                                })
                                .collect::<Vec<_>>();

                            matching_orders.sort_by(|a, b| {
                                let profit_a = a.input_amount - order_data.output_amount;
                                let profit_b = b.input_amount - order_data.output_amount;
                                profit_b.cmp(&profit_a)
                            });

                            let highest_profit_pair = matching_orders.first().cloned();

                            if let Some(highest_profit_pair) = highest_profit_pair {
                                info!("Arbitrage opportunity found! Sending orders for hashes {} and {}, generating payloads.", highest_profit_pair.order_hash, order_data.order_hash);

                                // Generate orders
                                let orders = vec![highest_profit_pair.clone(), order_data.clone()];
                                info!("vector of the highest profit: {:?}", orders);
                                self.generate_take_orders(orders).await
                            } else {
                                vec![]
                            }
                        } else {
                            vec![]
                        }
                    }
                    // on order cancelled, remove from orderbook hashmap
                    _ => vec![], // here handle other aori events
                }
            }
        }
    }
}

impl SimpleArb {
    pub async fn generate_take_orders(&self, orders: Vec<OrderCreatedData>) -> Vec<Action> {
        info!("Generating take orders for: {:?}", orders);
        let mut actions = Vec::new();

        for order in orders {
            info!("Processing order: {:?}", order);

            info!(
                "Creating payload using these order params {:?}",
                order.order.parameters.clone().to_order_components()
            );
            let payload = create_take_order_payload(
                &self.last_id,
                &self.wallet,
                order.order.parameters.to_order_components(),
                &order.order_hash,
                "0",
                &self.api_key,
            );

            info!("Payload created: {:?}", payload);

            let action = Action::SendAoriPayload(payload.unwrap());

            actions.push(action);
        }
        info!("Payloads generated: {:?}", actions);
        actions
    }
}
