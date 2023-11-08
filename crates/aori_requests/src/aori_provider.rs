use serde_json::Value;

use websockets::WebSocket;

use std::sync::{Arc, Mutex};

use eyre::Context;

use ethers::{
    prelude::{k256::ecdsa::SigningKey, LocalWallet, Wallet, Ws},
    providers::{Middleware, Provider},
    signers::Signer,
    types::Signature,
};

use aori_types::{
    constants::{MARKET_FEED_URL, REQUEST_URL},
    seaport::OrderComponents,
};

use crate::requests::*;

pub struct AoriProvider {
    pub request_conn: WebSocket,
    pub feed_conn: WebSocket,
    pub wallet: Wallet<SigningKey>,
    pub chain_id: u64,
    pub last_id: Arc<Mutex<u64>>,
    pub wallet_addr: Arc<str>,
    pub wallet_sig: Arc<str>,
}

impl AoriProvider {
    pub async fn new_from_env() -> eyre::Result<Self> {
        let key = std::env::var("PRIVATE_KEY").context("missing PRIVATE_KEY")?;
        let address = std::env::var("WALLET_ADDRESS").context("missing WALLET_ADDRESS")?;
        let node = std::env::var("NODE_URL").context("missing NODE_URL")?;

        let pv = Provider::<Ws>::connect(&node).await?;
        let chain_id = pv.get_chainid().await?.low_u64();
        // @dev adjust the provider to not have to initialise chain_id on boot,
        // instead operate with all chain ids on one provider and filter stream instead
        // - then add the ability to use different wallets per chain id

        let wallet = key.parse::<LocalWallet>()?.with_chain_id(chain_id);
        let sig: Signature = wallet.sign_message(address.as_str()).await?;
        let request_conn = WebSocket::connect(REQUEST_URL).await?;
        let feed_conn = WebSocket::connect(MARKET_FEED_URL).await?;

        Ok(Self {
            request_conn,
            feed_conn,
            wallet,
            chain_id,
            last_id: Arc::new(Mutex::new(0)),
            wallet_addr: address.into(),
            wallet_sig: format!("0x{}", sig).into(),
        })
    }
    ////////////////// GENERIC SEND //////////////////
    pub async fn send(&mut self, payload: Value) -> eyre::Result<()> {
        // Convert the JSON payload to a string and send it over the WebSocket connection.
        self.request_conn.send_text(payload.to_string()).await?;
        Ok(())
    }

    ////////////////// //////////////////

    //////////////////  SPECIFIC REQUESTS //////////////////

    pub async fn ping(&mut self) -> eyre::Result<()> {
        let ping_payload = create_ping_payload(&self.last_id);
        self.request_conn
            .send_text(ping_payload.to_string())
            .await?;
        Ok(())
    }

    pub async fn auth_wallet(&mut self) -> eyre::Result<()> {
        let auth_payload =
            create_auth_wallet_payload(&self.last_id, &self.wallet_addr, &self.wallet_sig);
        self.request_conn
            .send_text(auth_payload.to_string())
            .await?;
        Ok(())
    }

    pub async fn check_auth(&mut self, jwt: &str) -> eyre::Result<()> {
        let auth_payload = create_check_auth_payload(&self.last_id, jwt);
        self.request_conn
            .send_text(auth_payload.to_string())
            .await?;
        Ok(())
    }

    pub async fn view_orderbook(
        &mut self,
        base: &str,
        quote: &str,
        side: &str,
    ) -> eyre::Result<()> {
        let view_orderbook_payload =
            create_view_orderbook_payload(&self.last_id, self.chain_id, base, quote, side);
        self.request_conn
            .send_text(view_orderbook_payload.to_string())
            .await?;
        Ok(())
    }

    pub async fn make_order(&mut self, order_params: OrderComponents) -> eyre::Result<()> {
        let signed_order_payload =
            create_make_order_payload(&self.last_id, &self.wallet, order_params, self.chain_id)?;
        self.request_conn
            .send_text(signed_order_payload.to_string())
            .await?;
        Ok(())
    }

    pub async fn take_order(
        &mut self,
        order_params: OrderComponents,
        order_id: &str,
        seat_id: &str,
        api_key: &str,
    ) -> eyre::Result<()> {
        let take_order_payload = create_take_order_payload(
            &self.last_id,
            &self.wallet,
            order_params,
            order_id,
            seat_id,
            api_key,
        )?;
        self.request_conn
            .send_text(take_order_payload.to_string())
            .await?;
        Ok(())
    }

    pub async fn cancel_order(&mut self, order_id: &str, api_key: &str) -> eyre::Result<()> {
        let cancel_order_payload =
            create_cancel_order_payload(&self.last_id, &self.wallet, order_id, api_key).await?;
        self.request_conn
            .send_text(cancel_order_payload.to_string())
            .await?;
        Ok(())
    }

    pub async fn subscribe_orderbook(&mut self) -> eyre::Result<()> {
        let subscribe_orderbook_payload = create_subscribe_orderbook_payload(&self.last_id);
        self.feed_conn
            .send_text(subscribe_orderbook_payload.to_string())
            .await?;
        Ok(())
    }

    pub async fn account_orders(&mut self) -> eyre::Result<()> {
        let account_orders_payload =
            create_account_orders_payload(&self.last_id, &self.wallet_addr, &self.wallet_sig);
        self.request_conn
            .send_text(account_orders_payload.to_string())
            .await?;
        Ok(())
    }

    pub async fn order_status(&mut self, order_hash: &str) -> eyre::Result<()> {
        let order_status_payload = create_order_status_payload(&self.last_id, order_hash);
        self.request_conn
            .send_text(order_status_payload.to_string())
            .await?;
        Ok(())
    }

    // pub async fn cancel_all_orders(&mut self, api_key: &str) -> eyre::Result<()> {
    //     let cancel_all_order_payload = create_cancel_all_payload(&self.last_id, &self.wallet_addr, &self.wallet_sig, api_key);
    //     self.request_conn.send_text(cancel_all_order_payload.to_string()).await?;
    //     Ok(())
    // }

    // pub async fn request_quote(&mut self, input_token: &str, output_token: &str, input_amount: u64, api_key: &str) -> eyre::Result<()> {
    //     let quote_payload = create_request_quote_payload(&self.last_id, input_token, output_token, input_amount, self.chain_id, api_key);
    //     self.request_conn.send_text(quote_payload.to_string()).await?;
    //     Ok(())
    // }

    //////////////////  //////////////////
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, Address, U256};
    use alloy_sol_types::SolStruct;
    use aori_types::constants::{DEFAULT_CONDUIT_KEY, DEFAULT_ORDER_ADDRESS, DEFAULT_ZONE_HASH};
    use aori_types::seaport::{
        ConsiderationItem, ItemType, OfferItem, OrderComponents, OrderType, SEAPORT_DOMAIN,
    };
    use ethers::types::H256;
    use tokio::time::{sleep, Duration};
    use websockets::Frame;

    #[tokio::test]
    async fn generate_order_sig() {
        dotenv::dotenv().ok();
        let apv = AoriProvider::new_from_env()
            .await
            .expect("Failed to create Aori Provider");
        let offer_item = OfferItem {
            itemType: ItemType::ERC20 as u8,
            token: Address::ZERO,
            identifierOrCriteria: U256::from(0),
            startAmount: U256::from(0),
            endAmount: U256::from(0),
        };
        let consider_item = ConsiderationItem {
            itemType: ItemType::ERC20 as u8,
            token: Address::ZERO,
            identifierOrCriteria: U256::from(0),
            startAmount: U256::from(0),
            endAmount: U256::from(0),
            recipient: Address::ZERO,
        };
        let order_components = OrderComponents {
            offerer: Address::ZERO,
            zone: DEFAULT_ORDER_ADDRESS,
            offer: vec![offer_item],
            consideration: vec![consider_item],
            orderType: OrderType::PARTIAL_RESTRICTED as u8,
            startTime: U256::from(1697240202),
            endTime: U256::from(1697240202),
            zoneHash: DEFAULT_ZONE_HASH.into(),
            salt: U256::from(0),
            conduitKey: DEFAULT_CONDUIT_KEY.into(),
            counter: U256::from(0),
        };

        let params_sig = order_components.eip712_signing_hash(&*SEAPORT_DOMAIN);

        /*
        https://docs.rs/ethers/latest/ethers/signers/struct.Wallet.html#method.sign_typed_data
            async fn sign_typed_data<T: Eip712 + Send + Sync>(
                &self,
                payload: &T,
            ) -> Result<Signature, Self::Error> {
                let encoded =
                    payload.encode_eip712().map_err(|e| Self::Error::Eip712Error(e.to_string()))?;

                self.sign_hash(H256::from(encoded))
            }
        https://github.com/ProjectOpenSea/seaport-js/blob/c7552e1f77528f648b1208f04d4eac910171d48c/src/constants.ts#L10
        for the type you're signing
        */

        let signed_bytes: Signature = apv.wallet.sign_message(params_sig).await.unwrap();
        let signed_slice: Signature = apv
            .wallet
            .sign_hash(H256::from_slice(params_sig.as_slice()))
            .unwrap();
        println!("0x{}", signed_bytes);
        println!("0x{}", signed_slice);
    }

    #[tokio::test]
    async fn test_connection() {
        dotenv::dotenv().ok();
        let mut apv = AoriProvider::new_from_env()
            .await
            .expect("Failed to create Aori Provider");
        apv.ping().await.unwrap();
        let response = format!("{:#?}", apv.request_conn.receive().await.unwrap());
        println!("{response:}");
    }

    #[tokio::test]
    async fn test_auth() {
        dotenv::dotenv().ok();
        let mut apv = AoriProvider::new_from_env()
            .await
            .expect("Failed to create Aori Provider");
        apv.auth_wallet().await.unwrap();
        let frame: Frame = apv.request_conn.receive().await.unwrap();

        let payload: String = match frame {
            Frame::Text { payload, .. } => Some(payload),
            _ => None,
        }
        .unwrap();
        let resp_value: serde_json::Value = serde_json::from_str(&payload).unwrap();
        println!("{:#?}", resp_value);
        let jwt = resp_value.pointer("/result/auth").unwrap().to_string();
        apv.check_auth(jwt.as_str()).await.unwrap();
        sleep(Duration::from_millis(100)).await;
        let check = format!("{:#?}", apv.request_conn.receive().await.unwrap());
        println!("jwt > {}", jwt);
        println!(" check > {check:}");
    }

    #[tokio::test]
    async fn test_make_order() {
        dotenv::dotenv().ok();
        let wallet = std::env::var("WALLET_ADDRESS")
            .context("missing WALLET_ADDRESS")
            .unwrap();
        let start_time = chrono::Utc::now().timestamp_millis();
        let end_time = chrono::Utc::now().timestamp_millis() + 1000 * 60 * 60 * 24;
        let mut apv = AoriProvider::new_from_env()
            .await
            .expect("Failed to create Aori Provider");
        let offer_item = OfferItem {
            itemType: ItemType::ERC20 as u8,
            token: address!("2715Ccea428F8c7694f7e78B2C89cb454c5F7294"),
            identifierOrCriteria: U256::from(0),
            startAmount: U256::from(1000000000000000_u128),
            endAmount: U256::from(1000000000000000_u128),
        };
        let consider_item = ConsiderationItem {
            itemType: ItemType::ERC20 as u8,
            token: address!("D3664B5e72B46eaba722aB6f43c22dBF40181954"),
            identifierOrCriteria: U256::from(0),
            startAmount: U256::from(1500000),
            endAmount: U256::from(1500000),
            recipient: Address::parse_checksummed(&wallet, None).unwrap(),
        };
        let order_params = OrderComponents {
            offerer: Address::parse_checksummed(&wallet, None).unwrap(),
            zone: DEFAULT_ORDER_ADDRESS,
            offer: vec![offer_item.clone()],
            consideration: vec![consider_item.clone()],
            orderType: OrderType::PARTIAL_RESTRICTED as u8,
            startTime: U256::from(start_time),
            endTime: U256::from(end_time),
            zoneHash: DEFAULT_ZONE_HASH.into(),
            salt: U256::from(0),
            conduitKey: DEFAULT_CONDUIT_KEY.into(),
            // totalOriginalConsiderationItems: U256::from(1),
            counter: U256::from(0),
        };

        apv.make_order(order_params).await.unwrap();

        let response = format!("{:#?}", apv.request_conn.receive().await.unwrap());
        println!("{response:}");
    }
}
