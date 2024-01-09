use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::constants::{
    OrderSide, OrderStatus, OrderTimeInForce, OrderType, PerpetualPositionStatus,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountResponseObject {
    address: String,
    subaccount_number: u32,
    equity: String,
    free_collateral: String,
    open_perpetual_positions: Option<HashMap<String, PerpetualPositionResponseStruct>>,
    asset_positions: Option<HashMap<String, AssetPositionResponseStruct>>,
    margin_enabled: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PerpetualPositionResponseStruct {
    market: String,
    status: Option<PerpetualPositionStatus>,
    side: Option<PositionSide>,
    size: String,
    max_size: String,
    entry_price: String,
    realized_pnl: String,
    created_at: String,
    created_at_height: String,
    sum_open: String,
    sum_close: String,
    net_funding: String,
    unrealized_pnl: String,
    closed_at: Option<String>,
    exit_price: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetPositionResponseStruct {
    symbol: String,
    side: Option<PositionSide>,
    size: String,
    asset_id: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum PositionSide {
    LONG,
    SHORT,
}

// ========================================
// Transfer structs and enums
// ========================================

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferType {
    TransferIn,
    TransferOut,
    Deposit,
    Withdrawal,
}
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransferResponseSenderObject {
    sub_account_number: u32,
    address: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct TransferResponseStruct {
    id: String,
    sender: TransferResponseSenderObject,
    recipient: TransferResponseSenderObject,
    size: String,
    created_at: String,
    created_at_height: String,
    symbol: String,
    #[serde(rename = "type")]
    transfer_type: TransferType,
    transaction_hash: String,
}

// ========================================
// Order structs
// ========================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrderResponseStruct {
    id: String,
    subaccount_id: Option<String>,
    client_id: Option<String>,
    clob_pair_id: Option<String>,
    side: OrderSide,
    size: String,
    total_filled: String,
    price: String,
    #[serde(rename = "type")]
    order_type: OrderType,
    reduce_only: bool,
    order_flags: Option<String>,
    good_til_block: Option<String>,
    good_til_block_time: Option<String>,
    created_at_height: Option<String>,
    client_metadata: Option<String>,
    trigger_price: Option<String>,
    time_in_force: OrderTimeInForce,
    status: OrderStatus,
    post_only: bool,
    ticker: String,
}

// ========================================
// Fill structs
// ========================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FillResponseStruct {
    id: String,
    side: OrderSide,
    liquidity: Liquidity,
    #[serde(rename = "type")]
    fill_type: FillType,
    market: String,
    market_type: MarketType,
    price: String,
    size: String,
    fee: String,
    created_at: String,
    created_at_height: String,
    order_id: Option<String>,
    client_metadata: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Liquidity {
    Maker,
    Taker,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FillType {
    Maker,
    Limit,
    Liquidated,
    Liquidation,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketType {
    Perpetual,
    Spot,
}

// ========================================
// Historical PnL structs
// ========================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PnLTicksResponseStruct {
    id: String,
    subaccount_id: Option<String>,
    equity: String,
    total_pnl: String,
    net_transfers: String,
    created_at: String,
    block_height: String,
    block_time: String,
}

// ========================================
// Perpetual Market Structs
// ========================================

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PerpetualMarketResponseStruct {
    clob_pair_id: String,
    ticker: String,
    status: PerpetualMarketStatus,
    last_price: String,
    oracle_price: String,
    price_change_24h: String,
    volume_24h: String,
    trades_24h: f64,
    next_funding_rate: String,
    initial_margin_fraction: String,
    maintenance_margin_fraction: String,
    base_position_notional: String,
    open_interest: String,
    atomic_resolution: f64,
    quantum_conversion_exponent: f64,
    tick_size: String,
    step_size: String,
    step_base_quantums: f64,
    subticks_per_tick: f64,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]

pub enum PerpetualMarketStatus {
    Active,
    Paused,
    CancelOnly,
    PostOnly,
    Initializing,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookResponsePriceLevel {
    price: String,
    size: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeResponseStruct {
    id: String,
    side: OrderSide,
    size: String,
    price: String,
    create_at: String,
    create_at_height: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CandleResponseStruct {
    started_at: String,
    ticker: String,
    resolution: CandleResolution,
    low: String,
    high: String,
    open: String,
    close: String,
    base_token_volume: String,
    usd_volume: String,
    trades: u32,
    starting_open_interest: String,
    id: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]

pub enum CandleResolution {
    #[serde(rename = "1MIN")]
    OneMin,
    #[serde(rename = "5MINS")]
    FiveMin,
    #[serde(rename = "15MINS")]
    FifteenMin,
    #[serde(rename = "30MINS")]
    ThirtyMin,
    #[serde(rename = "1HOUR")]
    OneHour,
    #[serde(rename = "4HOURS")]
    FourHour,
    #[serde(rename = "1DAY")]
    OneDay,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalFundingResponseStruct {
    ticker: String,
    rate: String,
    price: String,
    effective_at: String,
    effective_at_height: String,
}

// ========================================
// Structs for vec of responses
// ========================================

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PerpetualPositionResponse {
    positions: Vec<PerpetualPositionResponseStruct>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AssetPositionResponse {
    positions: Vec<AssetPositionResponseStruct>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransferResponse {
    transfers: Vec<TransferResponseStruct>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FillResponse {
    fills: Vec<FillResponseStruct>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalPnLResponse {
    historical_pnl: Vec<FillResponseStruct>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PerpetualMarketsResponse {
    markets: HashMap<String, PerpetualMarketResponseStruct>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrderbookResponse {
    bids: Vec<OrderbookResponsePriceLevel>,
    asks: Vec<OrderbookResponsePriceLevel>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TradeResponse {
    trades: Vec<TradeResponseStruct>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CandleResponse {
    candles: Vec<CandleResponseStruct>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalFundingResponse {
    historical_funding: Vec<HistoricalFundingResponseStruct>,
}

pub type SparklineResponse = HashMap<String, Vec<String>>;

// ========================================
// Request structs
// ========================================

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PositionDetailsRequest {
    address: String,
    sub_account_number: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PerpetualPositionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_before_or_at_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_before_or_at: Option<String>,
}

impl Into<Vec<(String, Option<String>)>> for PositionDetailsRequest {
    fn into(self) -> Vec<(String, Option<String>)> {
        let mut v = vec![
            ("address".to_string(), Some(self.address)),
            (
                "subAccountNumber".to_string(),
                Some(format!("{}", self.sub_account_number)),
            ),
        ];
        if let Some(status) = self.status {
            v.push(("status".to_string(), Some(status.into())));
        };
        if let Some(limit) = self.limit {
            v.push(("limit".to_string(), Some(format!("{limit}"))));
        }
        if let Some(cboah) = self.create_before_or_at_height {
            v.push((
                "createdBeforeOrAtHeight".to_string(),
                Some(format!("{cboah}")),
            ));
        }
        if let Some(cboa) = self.create_before_or_at {
            v.push(("createdBeforeOrAt".to_string(), Some(cboa)));
        }
        v.to_owned()
    }
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subaccount_response_object_deserialization() {
        let json = r#"[
        {
          "address": "string",
          "subaccountNumber": 0,
          "equity": "string",
          "freeCollateral": "string",
          "openPerpetualPositions": {
            "property1": {
              "market": "string",
              "status": "OPEN",
              "side": "LONG",
              "size": "string",
              "maxSize": "string",
              "entryPrice": "string",
              "realizedPnl": "string",
              "createdAt": "string",
              "createdAtHeight": "string",
              "sumOpen": "string",
              "sumClose": "string",
              "netFunding": "string",
              "unrealizedPnl": "string",
              "closedAt": "string",
              "exitPrice": "string"
            },
            "property2": {
              "market": "string",
              "status": "OPEN",
              "side": "LONG",
              "size": "string",
              "maxSize": "string",
              "entryPrice": "string",
              "realizedPnl": "string",
              "createdAt": "string",
              "createdAtHeight": "string",
              "sumOpen": "string",
              "sumClose": "string",
              "netFunding": "string",
              "unrealizedPnl": "string",
              "closedAt": "string",
              "exitPrice": "string"
            }
          },
          "assetPositions": {
            "property1": {
              "symbol": "string",
              "side": "LONG",
              "size": "string",
              "assetId": "string"
            },
            "property2": {
              "symbol": "string",
              "side": "LONG",
              "size": "string",
              "assetId": "string"
            }
          },
          "marginEnabled": true
        }
      ]"#;

        let parsed: Vec<SubAccountResponseObject> = serde_json::from_str(json).unwrap();

        assert_eq!(parsed.len(), 1);

        let string = "string".to_string();
        let string_option = Some("string".to_string());

        let check_open_perpetual_position = |p: PerpetualPositionResponseStruct| {
            assert_eq!(p.market, string);
            assert_eq!(p.status, Some(PerpetualPositionStatus::OPEN));
            assert_eq!(p.side, Some(PositionSide::LONG));
            assert_eq!(p.size, string);
            assert_eq!(p.max_size, string);
            assert_eq!(p.entry_price, string);
            assert_eq!(p.realized_pnl, string);
            assert_eq!(p.created_at, string);
            assert_eq!(p.created_at_height, string);
            assert_eq!(p.sum_open, string);
            assert_eq!(p.sum_close, string);
            assert_eq!(p.net_funding, string);
            assert_eq!(p.unrealized_pnl, string);
            assert_eq!(p.closed_at, string_option);
            assert_eq!(p.exit_price, string_option);
        };

        let check_asset_position = |a: AssetPositionResponseStruct| {
            assert_eq!(a.symbol, string);
            assert_eq!(a.side, Some(PositionSide::LONG));
            assert_eq!(a.size, string);
            assert_eq!(a.asset_id, string_option);
        };

        let first = parsed.first().unwrap();
        assert_eq!(first.margin_enabled, true);
        assert_eq!(first.address, string);
        assert_eq!(first.subaccount_number, 0);
        assert_eq!(first.equity, string);

        println!("{:?}", first);
        let open_perpetual_positions_option = first.open_perpetual_positions.clone();

        assert_eq!(open_perpetual_positions_option.is_some(), true);

        let open_perpetual_position = open_perpetual_positions_option.unwrap();
        assert_eq!(open_perpetual_position.len(), 2);

        let first_opp = open_perpetual_position.values().next().unwrap();
        let second_opp = open_perpetual_position.values().skip(1).next().unwrap();
        check_open_perpetual_position(first_opp.to_owned());
        check_open_perpetual_position(second_opp.to_owned());

        let asset_positions_option = first.asset_positions.clone();
        assert_eq!(asset_positions_option.is_some(), true);

        let asset_positions = asset_positions_option.unwrap();
        assert_eq!(asset_positions.len(), 2);

        let first_ap = asset_positions.values().next().unwrap();
        let second_ap = asset_positions.values().skip(1).next().unwrap();
        check_asset_position(first_ap.to_owned());
        check_asset_position(second_ap.to_owned());
    }
}
