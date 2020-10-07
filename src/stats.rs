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


use super::ZapApiError;
use super::ZapService;
use serde_json::Value;
use std::collections::HashMap;


/**
 * This file was automatically generated.
 */
/**
 * Statistics
*/
pub async fn stats(service: &ZapService, keyprefix: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("keyPrefix".to_string(), keyprefix.to_string());
    super::call(service, "stats", "view", "stats", params).await
}

/**
 * Gets all of the site based statistics, optionally filtered by a key prefix
*/
pub async fn all_sites_stats(service: &ZapService, keyprefix: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("keyPrefix".to_string(), keyprefix.to_string());
    super::call(service, "stats", "view", "allSitesStats", params).await
}

/**
 * Gets all of the global statistics, optionally filtered by a key prefix
*/
pub async fn site_stats(service: &ZapService, site: &str, keyprefix: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site.to_string());
    params.insert("keyPrefix".to_string(), keyprefix.to_string());
    super::call(service, "stats", "view", "siteStats", params).await
}

/**
 * Gets the Statsd service hostname
*/
pub async fn option_statsd_host(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "stats", "view", "optionStatsdHost", params).await
}

/**
 * Gets the Statsd service port
*/
pub async fn option_statsd_port(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "stats", "view", "optionStatsdPort", params).await
}

/**
 * Gets the prefix to be applied to all stats sent to the configured Statsd service
*/
pub async fn option_statsd_prefix(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "stats", "view", "optionStatsdPrefix", params).await
}

/**
 * Returns 'true' if in memory statistics are enabled, otherwise returns 'false'
*/
pub async fn option_in_memory_enabled(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "stats", "view", "optionInMemoryEnabled", params).await
}

/**
 * Returns 'true' if a Statsd server has been correctly configured, otherwise returns 'false'
*/
pub async fn option_statsd_enabled(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "stats", "view", "optionStatsdEnabled", params).await
}

/**
 * Clears all of the statistics
*/
pub async fn clear_stats(service: &ZapService, keyprefix: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("keyPrefix".to_string(), keyprefix.to_string());
    super::call(service, "stats", "action", "clearStats", params).await
}

/**
 * Sets the Statsd service hostname, supply an empty string to stop using a Statsd service
*/
pub async fn set_option_statsd_host(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "stats", "action", "setOptionStatsdHost", params).await
}

/**
 * Sets the prefix to be applied to all stats sent to the configured Statsd service
*/
pub async fn set_option_statsd_prefix(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "stats", "action", "setOptionStatsdPrefix", params).await
}

/**
 * Sets whether in memory statistics are enabled
*/
pub async fn set_option_in_memory_enabled(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "stats", "action", "setOptionInMemoryEnabled", params).await
}

/**
 * Sets the Statsd service port
*/
pub async fn set_option_statsd_port(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "stats", "action", "setOptionStatsdPort", params).await
}

