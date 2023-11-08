use alloy_primitives::{address, hex, Address};

pub static DEFAULT_ORDER_ADDRESS: Address = address!("eA2b4e7F02b859305093f9F4778a19D66CA176d5");
pub static DEFAULT_ZONE_HASH: [u8; 32] =
    hex!("0000000000000000000000000000000000000000000000000000000000000000");
pub static DEFAULT_DURATION: i32 = 86400000_i32;
pub static CURRENT_SEAPORT_ADDRESS: Address = address!("00000000000000adc04c56bf30ac9d3c0aaf14dc");
pub static CURRENT_SEAPORT_VERSION: &str = "1.5";
pub static DEFAULT_CONDUIT_KEY: [u8; 32] =
    hex!("0000000000000000000000000000000000000000000000000000000000000000");

pub static REQUEST_URL: &str = "wss://api.beta.order.aori.io";
pub static MARKET_FEED_URL: &str = "wss://beta.feed.aori.io";
