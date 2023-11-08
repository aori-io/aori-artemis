use aori_types::seaport::{OrderComponents, SEAPORT_DOMAIN};

use alloy_primitives::FixedBytes;
use alloy_sol_types::SolStruct;
use ethers::{
    prelude::{k256::ecdsa::SigningKey, Wallet},
    signers::Signer,
    types::{Signature, H256},
};

use serde_json::json;
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub fn create_ping_payload(last_id: &Arc<Mutex<u64>>) -> Value {
    let mut id = last_id.lock().unwrap();
    *id += 1;
    json!({
        "id": *id,
        "jsonrpc": "2.0",
        "method": "aori_ping",
        "params": []
    })
}

pub fn create_auth_wallet_payload(
    last_id: &Arc<Mutex<u64>>,
    wallet_addr: &Arc<&str>,
    wallet_sig: &Arc<&str>,
) -> Value {
    let mut id = last_id.lock().unwrap();
    *id += 1;
    json!({
        "id": *id,
        "jsonrpc": "2.0",
        "method": "aori_authWallet",
        "params": [{
            "address": (*wallet_addr).as_ref(),
            "signature": (*wallet_sig).as_ref()
        }]
    })
}

pub fn create_check_auth_payload(last_id: &Arc<Mutex<u64>>, jwt: &str) -> Value {
    let mut id = last_id.lock().unwrap();
    *id += 1;
    json!({
        "id": *id,
        "jsonrpc": "2.0",
        "method": "aori_checkAuth",
        "params": [{
            "auth": jwt
        }]
    })
}

pub fn create_view_orderbook_payload(
    last_id: &Arc<Mutex<u64>>,
    chain_id: u64,
    base: &str,
    quote: &str,
    side: &str,
) -> Value {
    let mut id = last_id.lock().unwrap();
    *id += 1;
    json!({
        "id": *id,
        "jsonrpc": "2.0",
        "method": "aori_viewOrderbook",
        "params": [{
            "chainId": chain_id,
            "query": {
                "base": base,
                "quote": quote,
            },
            "side": side // accepts values "BUY" or "SELL"
            // limit: 100 // this is the default, if your application requires it, you can limit the number of orders shown via this parameter
        }]
    })
}

pub fn create_make_order_payload(
    last_id: &Arc<Mutex<u64>>,
    wallet: &Wallet<SigningKey>,
    order_params: OrderComponents,
    chain_id: u64,
) -> eyre::Result<Value> {
    let new_id = {
        let mut id = last_id.lock().unwrap();
        *id += 1;
        *id
    };

    let id = new_id;
    let sig: FixedBytes<32> = order_params.eip712_signing_hash(&SEAPORT_DOMAIN);
    let signed_sig: Signature = wallet.sign_hash(H256::from_slice(sig.as_slice()))?;

    Ok(json!({
        "id": id,
        "jsonrpc": "2.0",
        "method": "aori_makeOrder",
        "params": [{
            "order": {
                "signature": format!("0x{}", signed_sig),
                "parameters": order_params.to_json()
            },
            "isPublic": true,
            "chainId": chain_id
        }]
    }))
}

pub fn create_take_order_payload(
    last_id: &Arc<Mutex<u64>>,
    wallet: &Wallet<SigningKey>,
    order_params: OrderComponents,
    order_id: &str,
    seat_id: &str,
    api_key: &str,
) -> eyre::Result<Value> {
    let mut id = last_id.lock().unwrap();
    *id += 1;

    let sig: FixedBytes<32> = order_params.eip712_signing_hash(&SEAPORT_DOMAIN);
    let signed_sig: Signature = wallet.sign_hash(H256::from_slice(sig.as_slice()))?;

    Ok(json!({
        "id": *id,
        "jsonrpc": "2.0",
        "method": "aori_takeOrder",
        "params": [{
            "order": {
                "signature": format!("0x{}", signed_sig),
                "parameters": order_params.to_json()
            },
            "orderId": order_id,
            "seatId": seat_id,
            "apiKey": api_key
        }]
    }))
}

pub async fn create_cancel_order_payload(
    last_id: &Arc<Mutex<u64>>,
    wallet: &Wallet<SigningKey>,
    order_id: &str,
    api_key: &str,
) -> eyre::Result<Value> {
    let new_id = {
        let mut id = last_id.lock().unwrap();
        *id += 1;
        *id
    };

    let id = new_id;

    let sig: Signature = wallet.sign_message(order_id).await?;
    // need to add 0x to the order signature!
    let sig_with_prefix = format!("0x{}", sig);

    Ok(json!({
        "id": id,
        "jsonrpc": "2.0",
        "method": "aori_cancelOrder",
        "params": [{
            "orderId": order_id,
            "signature": sig_with_prefix,
            "apiKey": api_key
        }]
    }))
}

pub fn create_subscribe_orderbook_payload(last_id: &Arc<Mutex<u64>>) -> Value {
    let mut id = last_id.lock().unwrap();
    *id += 1;

    json!({
        "id": *id,
        "jsonrpc": "2.0",
        "method": "aori_subscribeOrderbook",
        "params": []
    })
}

pub fn create_account_orders_payload(
    last_id: &Arc<Mutex<u64>>,
    wallet_addr: &Arc<&str>,
    wallet_sig: &Arc<&str>,
) -> Value {
    let mut id = last_id.lock().unwrap();
    *id += 1;

    json!({
        "id": *id,
        "jsonrpc": "2.0",
        "method": "aori_accountOrders",
        "params": [{
            "offerer": (*wallet_addr).as_ref(),
            "signature": (*wallet_sig).as_ref(),
        }]
    })
}

pub fn create_order_status_payload(last_id: &Arc<Mutex<u64>>, order_hash: &str) -> Value {
    let mut id = last_id.lock().unwrap();
    *id += 1;

    json!({
        "id": *id,
        "jsonrpc": "2.0",
        "method": "aori_orderStatus",
        "params": [{
            "orderHash": order_hash,
        }]
    })
}

// pub fn create_cancel_all_payload(last_id: &Arc<Mutex<u64>>, wallet_addr: &Arc<str>, wallet_sig: &Arc<str>, api_key: &str) -> Value {
//     let mut id = last_id.lock().unwrap();
//     *id += 1;

//     json!({
//         "id": *id,
//         "jsonrpc": "2.0",
//         "method": "aori_cancelAllOrders",
//         "params": [{
//             "offerer": (*wallet_addr).as_ref(),
//             "signature": (*wallet_sig).as_ref(),
//             "apiKey": api_key
//         }]
//     })
// }

// pub fn create_request_quote_payload(last_id: &Arc<Mutex<u64>>, input_token: &str, output_token: &str, input_amount: u64, chain_id: u64, api_key: &str) -> Value {
//     let mut id = last_id.lock().unwrap();
//     *id += 1;

//     json!({
//         "id": *id,
//         "jsonrpc": "2.0",
//         "method": "aori_requestQuote",
//         "params": [{
//             "inputToken": input_token,
//             "outputToken": output_token,
//             "inputAmount": input_amount,
//             "chainId": chain_id,
//             "apiKey": api_key
//         }]
//     })
// }
