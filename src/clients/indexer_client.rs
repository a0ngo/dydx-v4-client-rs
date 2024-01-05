use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap};

use super::{errors::RestError, ConstructorError};
use is_url::is_url;
use reqwest::blocking::{self, Client as BlockingClient};

pub struct IndexerClient {
    indexer_config: IndexerConfig,
    api_timeout: u32,
    markets: MarketsClient,
    accounts: AccountsClient,
    utiltiy: UtilityClient,
    req_handler: RestHandler,
}

impl IndexerClient {
    pub fn new(indexer_config: IndexerConfig, api_timeout: Option<u32>) -> Self {
        let req_handler = RestHandler::new(
            indexer_config.rest_endpoint.clone(),
            indexer_config.websocket_endpoint.clone(),
        )?;
        IndexerClient {
            indexer_config,
            api_timeout: api_timeout.unwrap_or(3000),
            markets: MarketsClient::new(indexer_config.clone(), req_handler.clone()),
            accounts: AccountsClient::new(indexer_config.clone(), req_handler.clone()),
            utiltiy: UtilityClient::new(indexer_config.clone(), req_handler.clone()),
            req_handler,
        }
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
            req_client: blocking::ClientBuilder::new()
                .timeout(api_timeout.unwrap_or(3))
                .build(),
        })
    }

    pub fn get<T>(
        &self,
        path: String,
        query_params: Option<HashMap<String, String>>,
    ) -> Result<T, RestError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let query_string = match query_params {
            Some(query_map) => query_map
                .into_iter()
                .map(|(key, value)| format!("{}={}", key, value))
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
            return Err(RestError::new(format!("String is not URL: {}", url)));
        }

        match self.req_client.get(url).send() {
            Ok(result) => Ok(result.json::<T>()),
            Err(e) => Err(RestError::new(e.into())),
        }
    }

    pub fn post<T, B>(&self, path: String, body_args: HashMap<String, B>) -> Result<T, RestError>
    where
        T: for<'a> Deserialize<'a>,
        B: for<'a> Serialize,
    {
        let body = serde_json::to_string(body_args.borrow())?;
        let url = format!("{}{}", self.host, path);

        if !is_url(url.as_str()) {
            return Err(RestError::new(format!("String is not URL: {}", url)));
        }

        match self.req_client.post(url).body(body).send() {
            Ok(result) => Ok(result.json::<T>()),
            Err(e) => Err(RestError::new(e.into())),
        }
    }
}
