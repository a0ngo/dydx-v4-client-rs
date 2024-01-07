use crate::arg_to_tuple;
use crate::clients::CamelCaseify;
use crate::option_t_to_string_option;
use crate::option_to_tuple;
use serde::{Deserialize, Serialize};

use std::{borrow::Borrow, collections::HashMap, time::Duration};

use crate::constants::{OrderSide, OrderStatus, OrderType, TickerType};

use super::indexer_client_types::FillResponse;
use super::indexer_client_types::HistoricalPnLResponse;
use super::{
    errors::{APIError, ConstructorError},
    indexer_client_types::{
        AssetPositionResponse, OrderResponseStruct, PerpetualPositionResponse,
        PositionDetailsRequest, SubAccountResponseObject, TransferResponse,
    },
};
use is_url::is_url;
use reqwest::blocking::{self, Client as BlockingClient};

pub struct IndexerClient {
    indexer_config: IndexerConfig,
    api_timeout: u32,
    req_handler: RestHandler,
}

impl IndexerClient {
    pub fn new(
        indexer_config: IndexerConfig,
        api_timeout: Option<u32>,
    ) -> Result<Self, ConstructorError> {
        let req_handler = RestHandler::new(indexer_config.rest_endpoint.clone(), None)?;
        Ok(IndexerClient {
            indexer_config,
            api_timeout: api_timeout.unwrap_or(3000),
            // markets: MarketsClient::new(indexer_config.clone(), req_handler.clone()),
            // accounts: AccountsClient::new(req_handler.clone()),
            // utiltiy: UtilityClient::new(indexer_config.clone(), req_handler.clone()),
            req_handler,
        })
    }
}

#[derive(Clone)]
pub struct IndexerConfig {
    rest_endpoint: String,
    websocket_endpoint: String,
}

impl IndexerConfig {
    pub fn new(rest_endpoint: String, websocket_endpoint: String) -> Self {
        IndexerConfig {
            rest_endpoint,
            websocket_endpoint,
        }
    }
}

#[derive(Clone)]
pub(crate) struct RestHandler {
    host: String,
    timeout: u32,
    req_client: BlockingClient,
}

impl RestHandler {
    pub fn new(api_host: String, api_timeout: Option<u32>) -> Result<Self, ConstructorError> {
        let host = if api_host.rfind('/').unwrap_or(0) == (api_host.len() - 1) {
            api_host
                .chars()
                .take(api_host.len() - 1)
                .collect::<String>()
        } else {
            api_host
        };

        if !is_url(&host) {
            return Err(ConstructorError::new(format!(
                "Provided api host is not a url: {}",
                host
            )));
        }
        Ok(RestHandler {
            host,
            timeout: api_timeout.unwrap_or(3000),
            req_client: match blocking::ClientBuilder::new()
                .timeout(Duration::from_secs(api_timeout.unwrap_or(3).into()))
                .build()
            {
                Ok(client) => client,
                Err(e) => return Err(ConstructorError::new(e.to_string())),
            },
        })
    }

    pub fn get<T>(
        &self,
        path: String,
        query_params: Option<Vec<(String, Option<String>)>>,
    ) -> Result<T, APIError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let query_string = match query_params {
            Some(query_map) => query_map
                .into_iter()
                .map(|(key, value)| {
                    if let Some(value) = value {
                        format!("{}={}", key, value)
                    } else {
                        "".to_string()
                    }
                })
                .filter(|v| !v.is_empty())
                .collect::<Vec<String>>()
                .join("&"),
            None => "".to_string(),
        };

        let url = format!(
            "{}{}{}",
            self.host,
            path,
            if query_string.len() != 0 {
                format!("?{}", query_string)
            } else {
                "".to_string()
            }
        );

        if !is_url(url.as_str()) {
            return Err(APIError::new(format!("String is not URL: {}", url)));
        }

        match self.req_client.get(url).send() {
            Ok(result) => Ok(match result.json::<T>() {
                Ok(json) => json,
                Err(e) => return Err(APIError::new(e.to_string())),
            }),
            Err(e) => Err(APIError::new(e.to_string())),
        }
    }

    pub fn post<T, B>(&self, path: String, body_args: HashMap<String, B>) -> Result<T, APIError>
    where
        T: for<'a> Deserialize<'a>,
        B: for<'a> Serialize,
    {
        let body = match serde_json::to_string(body_args.borrow()) {
            Ok(str) => str,
            Err(e) => return Err(APIError::new(e.to_string())),
        };
        let url = format!("{}{}", self.host, path);

        if !is_url(url.as_str()) {
            return Err(APIError::new(format!("String is not URL: {}", url)));
        }

        match self.req_client.post(url).body(body).send() {
            Ok(result) => Ok(match result.json::<T>() {
                Ok(json) => json,
                Err(e) => return Err(APIError::new(e.to_string())),
            }),
            Err(e) => Err(APIError::new(e.to_string())),
        }
    }
}

impl AccountsClient for IndexerClient {
    fn get_sub_accounts(
        &self,
        address: String,
        limit: Option<u32>,
    ) -> Result<Vec<SubAccountResponseObject>, APIError> {
        self.req_handler.get(
            format!("/v4/addresses/{address}"),
            Some(vec![option_to_tuple!(limit)]),
        )
    }

    fn get_sub_account(
        &self,
        address: String,
        sub_account_number: u32,
    ) -> Result<SubAccountResponseObject, APIError> {
        self.req_handler.get(
            format!("/v4/addresses/{address}/subaccountNumber/{sub_account_number}"),
            None,
        )
    }

    fn get_sub_account_perpetual_positions(
        &self,
        request: PositionDetailsRequest,
    ) -> Result<PerpetualPositionResponse, APIError> {
        self.req_handler
            .get("/v4/perpetualPositions".to_string(), Some(request.into()))
    }

    fn get_sub_account_asset_positions(
        &self,
        request: PositionDetailsRequest,
    ) -> Result<AssetPositionResponse, APIError> {
        self.req_handler
            .get("/v4/assetPositions".to_string(), Some(request.into()))
    }

    fn get_sub_account_transfers(
        &self,
        address: String,
        sub_account_number: u32,
        limit: Option<u32>,
        created_before_or_at_height: Option<u32>,
        created_before_or_at: Option<String>,
    ) -> Result<TransferResponse, APIError> {
        self.req_handler.get(
            "/v4/transfers".to_string(),
            Some(vec![
                arg_to_tuple!(address),
                arg_to_tuple!(sub_account_number),
                option_to_tuple!(limit),
                option_to_tuple!(created_before_or_at_height),
                option_to_tuple!(created_before_or_at),
            ]),
        )
    }

    fn get_sub_account_orders(
        &self,
        address: String,
        sub_account_number: u32,
        ticker: Option<String>,
        ticker_type: TickerType,
        side: Option<OrderSide>,
        status: Option<OrderStatus>,
        order_type: Option<OrderType>,
        limit: Option<u32>,
        good_til_block_before_or_at: Option<u64>,
        good_til_block_time_before_or_at: Option<String>,
        return_latest_orders: Option<bool>,
    ) -> Result<Vec<OrderResponseStruct>, APIError> {
        self.req_handler.get(
            "/v4/orders".to_string(),
            Some(vec![
                arg_to_tuple!(address),
                arg_to_tuple!(sub_account_number),
                option_to_tuple!(ticker),
                arg_to_tuple!(ticker_type),
                option_to_tuple!(side),
                option_to_tuple!(status),
                option_to_tuple!(order_type),
                option_to_tuple!(limit),
                option_to_tuple!(good_til_block_before_or_at),
                option_to_tuple!(good_til_block_time_before_or_at),
                option_to_tuple!(return_latest_orders),
            ]),
        )
    }

    fn get_order(&self, order_id: String) -> Result<OrderResponseStruct, APIError> {
        self.req_handler.get(format!("/v4/orders/{order_id}"), None)
    }

    fn get_sub_account_fills(
        &self,
        address: String,
        sub_account_number: u32,
        ticker: Option<String>,
        ticker_type: TickerType,
        limit: Option<u32>,
        created_before_or_at_height: Option<u32>,
        created_before_or_at: Option<String>,
    ) -> Result<FillResponse, APIError> {
        self.req_handler.get(
            "/v4/fills".to_string(),
            Some(vec![
                arg_to_tuple!(address),
                arg_to_tuple!(sub_account_number),
                option_to_tuple!(ticker),
                arg_to_tuple!(ticker_type),
                option_to_tuple!(limit),
                option_to_tuple!(created_before_or_at_height),
                option_to_tuple!(created_before_or_at),
            ]),
        )
    }

    fn get_sub_account_historical_pnls(
        &self,
        address: String,
        sub_account_number: u32,
        effective_before_or_at: Option<String>,
        effective_at_or_after: Option<String>,
    ) -> Result<HistoricalPnLResponse, APIError> {
        self.req_handler.get(
            "/v4/historical-pnl".to_string(),
            Some(vec![
                arg_to_tuple!(address),
                arg_to_tuple!(sub_account_number),
                option_to_tuple!(effective_before_or_at),
                option_to_tuple!(effective_at_or_after),
            ]),
        )
    }
}

// ========================================================
// Client traits
// ========================================================

trait AccountsClient {
    fn get_sub_accounts(
        &self,
        address: String,
        limit: Option<u32>,
    ) -> Result<Vec<SubAccountResponseObject>, APIError>;

    fn get_sub_account(
        &self,
        address: String,
        sub_account_number: u32,
    ) -> Result<SubAccountResponseObject, APIError>;

    fn get_sub_account_perpetual_positions(
        &self,
        request: PositionDetailsRequest,
    ) -> Result<PerpetualPositionResponse, APIError>;

    fn get_sub_account_asset_positions(
        &self,
        request: PositionDetailsRequest,
    ) -> Result<AssetPositionResponse, APIError>;

    fn get_sub_account_transfers(
        &self,
        address: String,
        sub_account_number: u32,
        limit: Option<u32>,
        created_before_or_at_height: Option<u32>,
        created_before_or_at: Option<String>,
    ) -> Result<TransferResponse, APIError>;

    fn get_sub_account_orders(
        &self,
        address: String,
        sub_account_number: u32,
        ticker: Option<String>,
        ticker_type: TickerType,
        side: Option<OrderSide>,
        status: Option<OrderStatus>,
        order_type: Option<OrderType>,
        limit: Option<u32>,
        good_til_block_before_or_at: Option<u64>,
        good_til_block_time_before_or_at: Option<String>,
        return_latest_orders: Option<bool>,
    ) -> Result<Vec<OrderResponseStruct>, APIError>;

    fn get_order(&self, order_id: String) -> Result<OrderResponseStruct, APIError>;

    fn get_sub_account_fills(
        &self,
        address: String,
        sub_account_number: u32,
        ticker: Option<String>,
        ticker_type: TickerType,
        limit: Option<u32>,
        created_before_or_at_height: Option<u32>,
        created_before_or_at: Option<String>,
    ) -> Result<FillResponse, APIError>;

    fn get_sub_account_historical_pnls(
        &self,
        address: String,
        sub_account_number: u32,
        effective_before_or_at: Option<String>,
        effective_at_or_after: Option<String>,
    ) -> Result<HistoricalPnLResponse, APIError>;
}
