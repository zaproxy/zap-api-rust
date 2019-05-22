/* Zed Attack Proxy (ZAP) and its related class files.
 *
 * ZAP is an HTTP/HTTPS proxy for assessing web application security.
 *
 * Copyright 2019 the ZAP development team
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
 * Gets the alert with the given ID, the corresponding HTTP message can be obtained with the 'messageId' field and 'message' API method
*/
pub fn alert(service: &ZapService, id: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    super::call(service, "alert", "view", "alert", params)
}

/**
 * Gets the alerts raised by ZAP, optionally filtering by URL or riskId, and paginating with 'start' position and 'count' of alerts
*/
pub fn alerts(
    service: &ZapService,
    baseurl: String,
    start: String,
    count: String,
    riskid: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    params.insert("riskId".to_string(), riskid);
    super::call(service, "alert", "view", "alerts", params)
}

/**
 * Gets number of alerts grouped by each risk level, optionally filtering by URL
*/
pub fn alerts_summary(service: &ZapService, baseurl: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl);
    super::call(service, "alert", "view", "alertsSummary", params)
}

/**
 * Gets the number of alerts, optionally filtering by URL or riskId
*/
pub fn number_of_alerts(
    service: &ZapService,
    baseurl: String,
    riskid: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl);
    params.insert("riskId".to_string(), riskid);
    super::call(service, "alert", "view", "numberOfAlerts", params)
}

/**
 * Gets a summary of the alerts, optionally filtered by a 'url'. If 'recurse' is true then all alerts that apply to urls that start with the specified 'url' will be returned, otherwise only those on exactly the same 'url' (ignoring url parameters)
*/
pub fn alerts_by_risk(
    service: &ZapService,
    url: String,
    recurse: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url);
    params.insert("recurse".to_string(), recurse);
    super::call(service, "alert", "view", "alertsByRisk", params)
}

/**
 * Gets a count of the alerts, optionally filtered as per alertsPerRisk
*/
pub fn alert_counts_by_risk(
    service: &ZapService,
    url: String,
    recurse: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url);
    params.insert("recurse".to_string(), recurse);
    super::call(service, "alert", "view", "alertCountsByRisk", params)
}

/**
 * Deletes all alerts of the current session.
*/
pub fn delete_all_alerts(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "alert", "action", "deleteAllAlerts", params)
}

/**
 * Deletes the alert with the given ID.
*/
pub fn delete_alert(service: &ZapService, id: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    super::call(service, "alert", "action", "deleteAlert", params)
}
