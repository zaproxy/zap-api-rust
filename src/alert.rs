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
 * Gets the alert with the given ID, the corresponding HTTP message can be obtained with the 'messageId' field and 'message' API method
*/
pub async fn alert(service: &ZapService, id: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    super::call(service, "alert", "view", "alert", params).await
}

/**
 * Gets the alerts raised by ZAP, optionally filtering by URL or riskId, and paginating with 'start' position and 'count' of alerts
*/
pub async fn alerts(service: &ZapService, baseurl: &str, start: &str, count: &str, riskid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl.to_string());
    params.insert("start".to_string(), start.to_string());
    params.insert("count".to_string(), count.to_string());
    params.insert("riskId".to_string(), riskid.to_string());
    super::call(service, "alert", "view", "alerts", params).await
}

/**
 * Gets number of alerts grouped by each risk level, optionally filtering by URL
*/
pub async fn alerts_summary(service: &ZapService, baseurl: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl.to_string());
    super::call(service, "alert", "view", "alertsSummary", params).await
}

/**
 * Gets the number of alerts, optionally filtering by URL or riskId
*/
pub async fn number_of_alerts(service: &ZapService, baseurl: &str, riskid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl.to_string());
    params.insert("riskId".to_string(), riskid.to_string());
    super::call(service, "alert", "view", "numberOfAlerts", params).await
}

/**
 * Gets a summary of the alerts, optionally filtered by a 'url'. If 'recurse' is true then all alerts that apply to urls that start with the specified 'url' will be returned, otherwise only those on exactly the same 'url' (ignoring url parameters)
*/
pub async fn alerts_by_risk(service: &ZapService, url: &str, recurse: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url.to_string());
    params.insert("recurse".to_string(), recurse.to_string());
    super::call(service, "alert", "view", "alertsByRisk", params).await
}

/**
 * Gets a count of the alerts, optionally filtered as per alertsPerRisk
*/
pub async fn alert_counts_by_risk(service: &ZapService, url: &str, recurse: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url.to_string());
    params.insert("recurse".to_string(), recurse.to_string());
    super::call(service, "alert", "view", "alertCountsByRisk", params).await
}

/**
 * Deletes all alerts of the current session.
*/
pub async fn delete_all_alerts(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "alert", "action", "deleteAllAlerts", params).await
}

/**
 * Deletes the alert with the given ID. 
*/
pub async fn delete_alert(service: &ZapService, id: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    super::call(service, "alert", "action", "deleteAlert", params).await
}

/**
 * Update the confidence of the alerts.
*/
pub async fn update_alerts_confidence(service: &ZapService, ids: &str, confidenceid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids.to_string());
    params.insert("confidenceId".to_string(), confidenceid.to_string());
    super::call(service, "alert", "action", "updateAlertsConfidence", params).await
}

/**
 * Update the risk of the alerts.
*/
pub async fn update_alerts_risk(service: &ZapService, ids: &str, riskid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids.to_string());
    params.insert("riskId".to_string(), riskid.to_string());
    super::call(service, "alert", "action", "updateAlertsRisk", params).await
}

/**
 * Update the alert with the given ID, with the provided details.
*/
#[allow(clippy::too_many_arguments)]
pub async fn update_alert(service: &ZapService, id: &str, name: &str, riskid: &str, confidenceid: &str, description: &str, param: &str, attack: &str, otherinfo: &str, solution: &str, references: &str, evidence: &str, cweid: &str, wascid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    params.insert("name".to_string(), name.to_string());
    params.insert("riskId".to_string(), riskid.to_string());
    params.insert("confidenceId".to_string(), confidenceid.to_string());
    params.insert("description".to_string(), description.to_string());
    params.insert("param".to_string(), param.to_string());
    params.insert("attack".to_string(), attack.to_string());
    params.insert("otherInfo".to_string(), otherinfo.to_string());
    params.insert("solution".to_string(), solution.to_string());
    params.insert("references".to_string(), references.to_string());
    params.insert("evidence".to_string(), evidence.to_string());
    params.insert("cweId".to_string(), cweid.to_string());
    params.insert("wascId".to_string(), wascid.to_string());
    super::call(service, "alert", "action", "updateAlert", params).await
}

/**
 * Add an alert associated with the given message ID, with the provided details. (The ID of the created alert is returned.)
*/
#[allow(clippy::too_many_arguments)]
pub async fn add_alert(service: &ZapService, messageid: &str, name: &str, riskid: &str, confidenceid: &str, description: &str, param: &str, attack: &str, otherinfo: &str, solution: &str, references: &str, evidence: &str, cweid: &str, wascid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("messageId".to_string(), messageid.to_string());
    params.insert("name".to_string(), name.to_string());
    params.insert("riskId".to_string(), riskid.to_string());
    params.insert("confidenceId".to_string(), confidenceid.to_string());
    params.insert("description".to_string(), description.to_string());
    params.insert("param".to_string(), param.to_string());
    params.insert("attack".to_string(), attack.to_string());
    params.insert("otherInfo".to_string(), otherinfo.to_string());
    params.insert("solution".to_string(), solution.to_string());
    params.insert("references".to_string(), references.to_string());
    params.insert("evidence".to_string(), evidence.to_string());
    params.insert("cweId".to_string(), cweid.to_string());
    params.insert("wascId".to_string(), wascid.to_string());
    super::call(service, "alert", "action", "addAlert", params).await
}

