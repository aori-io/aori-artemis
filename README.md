# Aori Rust SDK

![H](assets/aori.svg)

Aori is a high-performance orderbook protocol for high-frequency trading on-chain and facilitating OTC settlement. This repository provides a Rust SDK for interacting with the Aori Websocket-based API to help developers integrate and build on top of the protocol as easily as possible.

This SDK is released under the [MIT License](LICENSE).

---

If you have any further questions, refer to [the technical documentation](https://www.aori.io/developers). Alternatively, please reach out to us [on Discord](https://discord.gg/K37wkh2ZfR) or [on Twitter](https://twitter.com/aori_io).

## Table of Contents
- [Installation](#installation)
  - [Initialization](#initialization)
  - [Aori Websockets API](#aori-websockets-api)
- [SDK Main Concepts](#sdk-main-concepts)
  - [Aori Provider](#aori-provider)
  - [Aori Events](#aori-events)
  - [Deserialising Aori Events](#deserialising-aori-events)
- [Examples](#examples)
  - [The Philosophy of Having Tons of Examples](#the-philosophy-of-having-tons-of-examples)
  - [How to run those examples](#how-to-run-those-examples)
  - [Simple Requests Examples](#requests-examples)
  - [Artemis Examples](#artemis-example)



# Installation

To install the SDK, add the following to the Cargo.toml:

```bash
[dependencies]
aori = { git = "https://github.com/aori-io/aori-sdk-rs/", branch = "main"}
```

## Initialization

After importing the SDK into your project, you can access the sdk in the following way (here as an example how to access the AoriProvider):

```rust
use aori::requests::aori_provider::AoriProvider;

let mut provider = AoriProvider::new_from_env().await.expect("Failed to create API provider.");

client.subscribe_orderbook().await.expect("Failed to subscribe orderbook.");

```
And to make a simple order: (/examples/make_order.rs)
```rust
use aori::requests::aori_provider::AoriProvider;

let mut provider = AoriProvider::new_from_env().await.expect("Failed to create API provider.");

let wallet = &provider.wallet_addr;
let sell_token = "0xD3664B5e72B46eaba722aB6f43c22dBF40181954";
let buy_token = "0x2715Ccea428F8c7694f7e78B2C89cb454c5F7294";
let sell_amount = "100000000"; // 100 usdc (6 decimals)
let buy_amount = "750000000000000000"; // 0.75 eth (18 decimals)

let order_params = OrderParameters::limit_order(wallet, sell_token, sell_amount, buy_token, buy_amount).to_order_components();


provider.make_order(order_params).await.expect("Failed to send make_order");

```

Please refer to the ./examples folder to see working examples with documentation.

## Aori Websockets API

As the Aori API is a Websocket-based API, requests and responses may come back in an asynchronous manner. 
AoriProvider uses the Websocket crate and is utilising a split into WebSocketReadHalf and WebSocketWriteHalf. These handle read and write operations, respectively.

Aori Websockets API is using two main endpoints:
- "wss://beta.feed.aori.io" for orderbook updates (aori 'tape')
- "wss://api.beta.order.aori.io" for everything else.

These are named MARKET_URL and REQUEST_URL in constants.rs.

In the AoriProvider, those are initiliased as feed_conn and request_conn, respectively.

# SDK Main Concepts

## Aori Provider

There are a number of key functionalities that can be performed using the SDK, perhaps one of the more important ones is the initialisation of AoriProvider.

AoriProvider requires having NODE_URL (websocket), WALLET_ADDRESS and PRIVATE_KEY as environmental variables.

It can be then initialised and called in the following way:

```rust
use aori::requests::aori_provider::AoriProvider;

let mut provider = AoriProvider::new_from_env().await.expect("Failed to create API provider.");

provider.subscribe_orderbook().await.expect("Failed to subscribe orderbook.");

```

## Aori Events

The SDK has been developed with multiple types of users in mind. Therefore it has alongside the AoriProvider also separate requests.rs file with functions for payload generation, as well as types.rs describing the various types and AoriEvents. For example. if you want to deserialise or just use the OrderCreated event, you can do so in the following manner:

```rust
use aori::types::events::OrderCreatedData;
```
This imports the struct OrderCreatedData that takes the following form:
```rust
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct OrderCreatedData {
    pub order: OrderCreationData,
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    #[serde(rename = "inputToken")]
    pub input_token: String,
    #[serde(rename = "outputToken")]
    pub ouptut_token: String,
    #[serde(rename = "inputAmount")]
    pub input_amount: u64,
    #[serde(rename = "outputAmount")]
    pub output_amount: u64,
    pub rate: Option<f64>,
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
```
You can then access or filter events based on characteristics very easily, such as filtering based on input/output token or chain_id.

## Deserialising Aori Events

You can use AoriEvents to deserialise events received from subscribe orderbook. For a good example on how to do it look into the process_payload function in the artemis example: (./examples/artemis-simple/main.rs)
```rust
async fn process_payload(payload: String, tx: &UnboundedSender<AoriEvent>) -> Result<()> {
    if payload.contains("Subscribed to orderbook updates") {
        tx.send(AoriEvent::Subscribed("[AORI.IO] Subscribed to orderbook updates".to_string()))
            .expect("Error sending subscription confirmation");
    } else {
        match serde_json::from_str::<AoriResponse>(&payload) {
            Ok(response) => {
                tx.send(response.result).expect("Error sending response event");
            }
            Err(e) => {
                eprintln!("Failed to deserialize the message: {}", e);
            }
        }
    }
    Ok(())
}
```

# Examples
The examples section should reflect the two-fold nature of this library. 

Some examples are focused on predominantly traders looking to take their on-chain trading to the next level, while others are more searchers-focused, showing how to use aori with the artemis framework to possibly fill and offer trades for the traders.



## The philosophy of having tons of examples
We firmly believe that every method should have a ready-to-run example that just literally after pressing "cargo run" - 
don't want you guys to be spending time on figuring out how to work with aori, instead I want you to be spending time on more useful acitvities (like which strategies to run and how o optimise them).

I've been personally confused by many codebases and really, if you think something is missing either create a new branch and send me a pull request or just post an issue! Aori is meant also for people who never coded before, so the examples section should hopefully reflect that.

Rust can be confusing enough for y'all to be confused about the codebase even more.

-> if there is something (can be any method, feature, anything) that doesn't have an example and you'd want to have it, post us an issue and we'll add it!


## How to run those examples
Navigate into the examples folder, then do the following:
```
export WALLET_ADDRESS=0x_your_wallet_address
export PRIVATE_KEY=your_private_key (without the 0x prefix)
export NODE_URL=(wss node url) // can use 'wss://ethereum-goerli.publicnode.com' for example //
```

```
cargo run -- bin EXAMPLE_NAME
```
for example:
```
cargo run --bin account_orders
```

Play around - adjust the example to fit your needs, try out new stuff, try to break things.

## Requests examples
These examples should cover all methods relevant for accessing the api, as well as multiple ways of accessing the endpoint - 
using the aori provider and pre-built methods, building the query yourself manually or using requests.rs and then sending using AoriProvider.send().

There are some benefits of utilising something like an aori collector from the artemis folder if you want to sync multiple data streams,
but for the basic bots, this should be sufficent.


## Artemis example
Artemis is a framework for writing MEV bots in Rust.

As of now, this part is work in progress, but soon, there will be a simple implementation of an aori artemis bot.

Currently there is a collector confirming to artemis standard by outputting a stream, you should be able to plug this in into any artemis strategy.

We will have AoriExecutor, as well as an example strategy soon.