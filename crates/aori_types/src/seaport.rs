use alloy_sol_macro::sol;

use alloy_sol_types::{eip712_domain, Eip712Domain};

use once_cell::sync::Lazy;

use serde_json::{json, Value};

use crate::constants::{CURRENT_SEAPORT_ADDRESS, CURRENT_SEAPORT_VERSION};

pub static SEAPORT_DOMAIN: Lazy<Eip712Domain> = Lazy::new(|| {
    eip712_domain! {
        name: String::from("Seaport"),
        version: String::from(CURRENT_SEAPORT_VERSION),
        chain_id: 5,
        verifying_contract: CURRENT_SEAPORT_ADDRESS,
    }
});

sol! {
    #[derive(Debug)]
    enum OrderType {
        FULL_OPEN,
        PARTIAL_OPEN,
        FULL_RESTRICTED,
        PARTIAL_RESTRICTED,
        CONTRACT
    }

    #[derive(Debug)]
    enum ItemType {
        NATIVE,
        ERC20,
        ERC721,
        ERC1155,
        ERC721_WITH_CRITERIA,
        ERC1155_WITH_CRITERIA
    }

    #[derive(Debug)]
    struct OfferItem {
        uint8 itemType;
        address token;
        uint256 identifierOrCriteria;
        uint256 startAmount;
        uint256 endAmount;
    }

    #[derive(Debug)]
    struct OrderComponents {
        address offerer;
        address zone;
        OfferItem[] offer;
        ConsiderationItem[] consideration;
        uint8 orderType;
        uint256 startTime;
        uint256 endTime;
        bytes32 zoneHash;
        uint256 salt;
        bytes32 conduitKey;
        uint256 counter;
    }

    #[derive(Debug)]
    struct ConsiderationItem {
        uint8 itemType;
        address token;
        uint256 identifierOrCriteria;
        uint256 startAmount;
        uint256 endAmount;
        address payable recipient;
    }

    #[derive(Debug)]
    struct ReceivedItem {
        ItemType itemType;
        address token;
        uint256 identifier;
        uint256 amount;
        address payable recipient;
    }

    #[derive(Debug)]
    struct BasicOrderParameters {
        address considerationToken;
        uint256 considerationIdentifier;
        uint256 considerationAmount;
        address payable offerer;
        address zone;
        address offerToken;
        uint256 offerIdentifier;
        uint256 offerAmount;
        BasicOrderType basicOrderType;
        uint256 startTime;
        uint256 endTime;
        bytes32 zoneHash;
        uint256 salt;
        bytes32 offererConduitKey;
        bytes32 fulfillerConduitKey;
        uint256 totalOriginalAdditionalRecipients;
        AdditionalRecipient[] additionalRecipients;
        bytes signature;
    }

    #[derive(Debug)]
    struct OrderParameters {
        address offerer;
        address zone;
        OfferItem[] offer;
        ConsiderationItem[] consideration;
        OrderType orderType;
        uint256 startTime;
        uint256 endTime;
        bytes32 zoneHash;
        uint256 salt;
        bytes32 conduitKey;
        uint256 totalOriginalConsiderationItems;
    }

    #[derive(Debug)]
    struct Order {
        OrderParameters parameters;
        bytes signature;
    }

    #[derive(Debug)]
    struct AdvancedOrder {
        OrderParameters parameters;
        uint120 numerator;
        uint120 denominator;
        bytes signature;
        bytes extraData;
    }

    #[derive(Debug)]
    struct OrderStatus {
        bool isValidated;
        bool isCancelled;
        uint120 numerator;
        uint120 denominator;
    }

    #[derive(Debug)]
    struct ZoneParameters {
        bytes32 orderHash;
        address fulfiller;
        address offerer;
        SpentItem[] offer;
        ReceivedItem[] consideration;
        bytes extraData;
        bytes32[] orderHashes;
        uint256 startTime;
        uint256 endTime;
        bytes32 zoneHash;
    }

    #[derive(Debug)]
    struct SpentItem {
        ItemType itemType;
        address token;
        uint256 identifier;
        uint256 amount;
    }

    #[derive(Debug)]
    struct AdditionalRecipient {
        uint256 amount;
        address payable recipient;
    }

    #[derive(Debug)]
    enum BasicOrderType {
        ETH_TO_ERC721_FULL_OPEN,
        ETH_TO_ERC721_PARTIAL_OPEN,
        ETH_TO_ERC721_FULL_RESTRICTED,
        ETH_TO_ERC721_PARTIAL_RESTRICTED,
        ETH_TO_ERC1155_FULL_OPEN,
        ETH_TO_ERC1155_PARTIAL_OPEN,
        ETH_TO_ERC1155_FULL_RESTRICTED,
        ETH_TO_ERC1155_PARTIAL_RESTRICTED,
        ERC20_TO_ERC721_FULL_OPEN,
        ERC20_TO_ERC721_PARTIAL_OPEN,
        ERC20_TO_ERC721_FULL_RESTRICTED,
        ERC20_TO_ERC721_PARTIAL_RESTRICTED,
        ERC20_TO_ERC1155_FULL_OPEN,
        ERC20_TO_ERC1155_PARTIAL_OPEN,
        ERC20_TO_ERC1155_FULL_RESTRICTED,
        ERC20_TO_ERC1155_PARTIAL_RESTRICTED,
        ERC721_TO_ERC20_FULL_OPEN,
        ERC721_TO_ERC20_PARTIAL_OPEN,
        ERC721_TO_ERC20_FULL_RESTRICTED,
        ERC721_TO_ERC20_PARTIAL_RESTRICTED,
        ERC1155_TO_ERC20_FULL_OPEN,
        ERC1155_TO_ERC20_PARTIAL_OPEN,
        ERC1155_TO_ERC20_FULL_RESTRICTED,
        ERC1155_TO_ERC20_PARTIAL_RESTRICTED
    }
}

impl OfferItem {
    pub fn to_json(&self) -> Value {
        json!({
            "itemType": self.itemType,
            "token": format!("{}", self.token),
            "identifierOrCriteria": format!("{}", self.identifierOrCriteria),
            "startAmount": format!("{}", self.startAmount),
            "endAmount": format!("{}", self.endAmount)
        })
    }
}

impl ConsiderationItem {
    pub fn to_json(&self) -> Value {
        json!({
            "itemType": self.itemType,
            "token": format!("{}", self.token),
            "identifierOrCriteria": format!("{}", self.identifierOrCriteria),
            "startAmount": format!("{}", self.startAmount),
            "endAmount": format!("{}", self.endAmount),
            "recipient": format!("{}", self.recipient)
        })
    }
}

impl OrderParameters {
    pub fn to_json(&self) -> Value {
        json!({
            "offerer": format!("{}", self.offerer),
            "zone": format!("{}", self.zone),
            "offer": self.offer.iter().map(|item| item.to_json()).collect::<Vec<Value>>(),
            "consideration": self.consideration.iter().map(|item| item.to_json()).collect::<Vec<Value>>(),
            "orderType": self.orderType as u8,
            "startTime": format!("{}", self.startTime),
            "endTime": format!("{}", self.endTime),
            "zoneHash": format!("{}", self.zoneHash),
            "salt": format!("{}", self.salt),
            "conduitKey": format!("{}", self.conduitKey),
            "totalOriginalConsiderationItems": self.totalOriginalConsiderationItems.to::<i16>(),
        })
    }
}

impl OrderComponents {
    pub fn to_json(&self) -> Value {
        json!({
            "offerer": format!("{}", self.offerer),
            "zone": format!("{}", self.zone),
            "offer": self.offer.iter().map(|item| item.to_json()).collect::<Vec<Value>>(),
            "consideration": self.consideration.iter().map(|item| item.to_json()).collect::<Vec<Value>>(),
            "orderType": self.orderType,
            "startTime": format!("{}", self.startTime),
            "endTime": format!("{}", self.endTime),
            "zoneHash": format!("{}", self.zoneHash),
            "salt": format!("{}", self.salt),
            "conduitKey": format!("{}", self.conduitKey),
            "totalOriginalConsiderationItems": 1,
            "counter": format!("{}", self.counter),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{DEFAULT_CONDUIT_KEY, DEFAULT_ORDER_ADDRESS, DEFAULT_ZONE_HASH};
    use alloy_primitives::{Address, U256};

    #[test]
    fn load_lazy() {
        let dom = &*SEAPORT_DOMAIN;
        println!("{:?}", dom);
    }

    #[test]
    fn parse_to_json() {
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
        let order_comps = OrderComponents {
            offerer: Address::ZERO,
            zone: DEFAULT_ORDER_ADDRESS,
            offer: vec![offer_item.clone(), offer_item.clone()],
            consideration: vec![consider_item.clone(), consider_item.clone()],
            orderType: OrderType::PARTIAL_RESTRICTED as u8,
            startTime: U256::from(1697240202),
            endTime: U256::from(1697240202),
            zoneHash: DEFAULT_ZONE_HASH.into(),
            salt: U256::from(0),
            conduitKey: DEFAULT_CONDUIT_KEY.into(),
            counter: U256::from(0),
        };
        let order_params = OrderParameters {
            offerer: Address::ZERO,
            zone: DEFAULT_ORDER_ADDRESS,
            offer: vec![offer_item.clone(), offer_item.clone()],
            consideration: vec![consider_item.clone(), consider_item.clone()],
            orderType: OrderType::PARTIAL_RESTRICTED,
            startTime: U256::from(1697240202),
            endTime: U256::from(1697240202),
            zoneHash: DEFAULT_ZONE_HASH.into(),
            salt: U256::from(0),
            conduitKey: DEFAULT_CONDUIT_KEY.into(),
            totalOriginalConsiderationItems: U256::from(2),
        };
        let comps_json = order_comps.to_json();
        let params_json = order_params.to_json();
        println!("{:#?} {:#?}", order_comps, comps_json);
        assert!(comps_json.pointer("/offer").unwrap().is_array());
        assert!(params_json.pointer("/offer").unwrap().is_array());
        assert_eq!(comps_json["offer"].as_array().unwrap().len(), 2);
        assert_eq!(params_json["offer"].as_array().unwrap().len(), 2);
        assert_eq!(comps_json["consideration"].as_array().unwrap().len(), 2);
        assert_eq!(params_json["consideration"].as_array().unwrap().len(), 2);
    }
}
