use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountResponseObject {
    address: String,
    subaccount_number: f32,
    equity: String,
    free_collateral: String,
    open_perpetuals_positions: PerpetualsPositionsMap,
    asset_positions: AssetPositionMap,
    margin_enabled: bool,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerpetualsPositionsMap {
    additional_properties: Option<HashMap<String, PerpetualPositionResponseObject>>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerpetualPositionResponseObject {
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
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum PerpetualPositionStatus {
    OPEN,
    CLOSED,
    LIQUIDATED,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetPositionMap {
    additional_properties: Option<HashMap<String, AssetPositionResponseObject>>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetPositionResponseObject {
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