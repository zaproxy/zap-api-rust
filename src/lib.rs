/* Zed Attack Proxy (ZAP) and its related class files.
 *
 * ZAP is an HTTP/HTTPS proxy for assessing web application security.
 *
 * Copyright 2020 the ZAP development team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! The Rust implementation to access the OWASP ZAP [API](https://www.zaproxy.org/docs/api/).
//!
//! For more information about OWASP ZAP, please visit [zaproxy.org](https://www.zaproxy.org)

#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![warn(missing_docs)]

use serde_json::Value;
use std::collections::HashMap;

/// anti-CSRF tokens
pub mod acsrf;

/// AJAX Spider
pub mod ajax_spider;

/// Alert
pub mod alert;

/// Alert Filter
pub mod alert_filter;

/// Active Scan
pub mod ascan;

/// Authentication
pub mod authentication;

/// Authorization
pub mod authorization;

/// Auto Update
pub mod autoupdate;

/// Break
pub mod brk;

/// Context
pub mod context;

/// Core
pub mod core;

/// Forced User
pub mod forced_user;

/// HTTP Sessions
pub mod http_sessions;

/// Import Log Files
pub mod import_log_files;

/// Import URLs
pub mod importurls;

/// Local Proxies
pub mod local_proxies;

/// OpenAPI
pub mod openapi;

/// Parameters
pub mod params;

/// Plug-n-Hack
pub mod pnh;

/// Passive Scan
pub mod pscan;

///  Replacer
pub mod replacer;

/// Reveal
pub mod reveal;

/// Rule Config
pub mod rule_config;

/// Script
pub mod script;

/// Search
pub mod search;

/// Selenium
pub mod selenium;

/// Session Management
pub mod session_management;

/// SOAP
pub mod soap;

/// Spider
pub mod spider;

/// Stats
pub mod stats;

/// Users
pub mod users;

/// WebSockets
pub mod websocket;

/// ZAP API Service.
///
/// # Example
///
/// ```
/// let zap_url = "http://localhost:8080".to_string();
/// let zap_api_key = "ChangeMe".to_string();
///
/// let service = ZapService {
///     url: zap_url,
///     api_key: zap_api_key,
/// };
/// ```
#[derive(Debug)]
pub struct ZapService {
    /// Base url of the ZAP API, eg http://localhost:8080
    pub url: String,

    /// API key used for connecting securely to the API
    pub api_key: String,
}

impl ZapService {
    /// Create a new instance of `ZapService`.
    pub fn new(url: String, api_key: String) -> Self {
        Self { url, api_key }
    }
}

/// ZAP API Error.
#[derive(Debug)]
pub struct ZapApiError {
    // Type of the error
    kind: String,

    // Error message
    message: String,
}

impl From<reqwest::Error> for ZapApiError {
    fn from(error: reqwest::Error) -> Self {
        ZapApiError {
            kind: String::from("comms"),
            message: error.to_string(),
        }
    }
}

impl From<serde_json::error::Error> for ZapApiError {
    fn from(error: serde_json::error::Error) -> Self {
        ZapApiError {
            kind: String::from("serde"),
            message: error.to_string(),
        }
    }
}

async fn call(
    service: &ZapService,
    component: &str,
    calltype: &str,
    method: &str,
    params: HashMap<String, String>,
) -> Result<Value, ZapApiError> {
    let mut url = [&service.url, "JSON", component, calltype, method, ""].join("/");
    url.push_str("?");
    let url_params = params
        .into_iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&");
    url.push_str(&url_params);

    let client = reqwest::Client::new()
        .get(&url)
        .header("X-ZAP-API-Key", &*service.api_key)
        .send()
        .await?;

    let text = client.text().await?;
    let json = serde_json::from_str(&text)?;
    Ok(json)
}
