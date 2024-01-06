use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountResponseObject {
    address: String,
    subaccount_number: u32,
    equity: String,
    free_collateral: String,
    open_perpetual_positions: Option<HashMap<String, PerpetualPositionResponseObject>>,
    asset_positions: Option<HashMap<String, AssetPositionResponseObject>>,
    margin_enabled: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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

#[derive(Deserialize, Serialize, Clone, Debug)]
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

        let check_open_perpetual_position = |p: PerpetualPositionResponseObject| {
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

        let check_asset_position = |a: AssetPositionResponseObject| {
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
