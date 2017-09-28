use super::decimal;

pub type SecurityId = String;
pub type Volume = u32;
pub type Price = decimal::d128;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExchangeId {
    SH, SZ, SHFE, ZCE, CFFEX, DCE
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Direction {
    Buy, Sell
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SecurityUuid {
    pub exchange_id: ExchangeId,
    pub security_id: SecurityId
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderReq {
    pub account: Account,
    pub security: SecurityUuid,
    pub direction: Direction,
    pub price: Price,
    pub volume: Volume,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnyRequest {
    Order(OrderReq),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestField {
    pub id: u32,
    pub request: AnyRequest
}