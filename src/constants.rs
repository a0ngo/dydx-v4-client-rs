use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum TickerType {
    PERPETUAL,
}

impl Into<String> for TickerType {
    fn into(self) -> String {
        match self {
            TickerType::PERPETUAL => String::from("PERPETUAL"),
        }
    }
}

impl ToString for TickerType {
    fn to_string(&self) -> String {
        match self {
            TickerType::PERPETUAL => String::from("PERPETUAL"),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum PerpetualPositionStatus {
    OPEN,
    CLOSED,
    LIQUIDATED,
}

impl Into<String> for PerpetualPositionStatus {
    fn into(self) -> String {
        match self {
            PerpetualPositionStatus::OPEN => String::from("OPEN"),
            PerpetualPositionStatus::CLOSED => String::from("CLOSED"),
            PerpetualPositionStatus::LIQUIDATED => String::from("LIQUIDATED"),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    BestEffortOpened,
    Open,
    Filled,
    BestEffortCanceled,
    Canceled,
    Untriggered,
}

impl Into<String> for OrderStatus {
    fn into(self) -> String {
        match self {
            OrderStatus::BestEffortOpened => String::from("BEST_EFFORT_OPENED"),
            OrderStatus::Open => String::from("OPEN"),
            OrderStatus::Filled => String::from("FILLED"),
            OrderStatus::BestEffortCanceled => String::from("BEST_EFFORT_CANCELED"),
            OrderStatus::Canceled => String::from("CANCELED"),
            OrderStatus::Untriggered => String::from("UNTRIGGERED"),
        }
    }
}

impl ToString for OrderStatus {
    fn to_string(&self) -> String {
        match self {
            OrderStatus::BestEffortOpened => String::from("BEST_EFFORT_OPENED"),
            OrderStatus::Open => String::from("OPEN"),
            OrderStatus::Filled => String::from("FILLED"),
            OrderStatus::BestEffortCanceled => String::from("BEST_EFFORT_CANCELED"),
            OrderStatus::Canceled => String::from("CANCELED"),
            OrderStatus::Untriggered => String::from("UNTRIGGERED"),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    BUY,
    SELL,
}

impl Into<String> for OrderSide {
    fn into(self) -> String {
        match self {
            OrderSide::BUY => String::from("BUY"),
            OrderSide::SELL => String::from("SELL"),
        }
    }
}

impl ToString for OrderSide {
    fn to_string(&self) -> String {
        match self {
            OrderSide::BUY => String::from("BUY"),
            OrderSide::SELL => String::from("SELL"),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    TakeProfitLimit,
    StopMarket,
    TakeProfitMarket,
}

impl Into<String> for OrderType {
    fn into(self) -> String {
        match self {
            OrderType::Limit => String::from("LIMIT"),
            OrderType::Market => String::from("MARKET"),
            OrderType::StopLimit => String::from("STOP_LIMIT"),
            OrderType::TakeProfitLimit => String::from("TAKE_PROFIT_LIMIT"),
            OrderType::StopMarket => String::from("STOP_MARKET"),
            OrderType::TakeProfitMarket => String::from("TAKE_PROFIT_MARKET"),
        }
    }
}

impl ToString for OrderType {
    fn to_string(&self) -> String {
        match self {
            OrderType::Limit => String::from("LIMIT"),
            OrderType::Market => String::from("MARKET"),
            OrderType::StopLimit => String::from("STOP_LIMIT"),
            OrderType::TakeProfitLimit => String::from("TAKE_PROFIT_LIMIT"),
            OrderType::StopMarket => String::from("STOP_MARKET"),
            OrderType::TakeProfitMarket => String::from("TAKE_PROFIT_MARKET"),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum OrderTimeInForce {
    GIT,
    IOC,
    FOK,
}

impl Into<String> for OrderTimeInForce {
    fn into(self) -> String {
        match self {
            OrderTimeInForce::GIT => String::from("GIT"),
            OrderTimeInForce::IOC => String::from("IOC"),
            OrderTimeInForce::FOK => String::from("FOK"),
        }
    }
}
