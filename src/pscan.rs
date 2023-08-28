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
 * Tells whether or not the passive scan should be performed only on messages that are in scope.
*/
pub async fn scan_only_in_scope(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "view", "scanOnlyInScope", params).await
}

/**
 * The number of records the passive scanner still has to scan
*/
pub async fn records_to_scan(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "view", "recordsToScan", params).await
}

/**
 * Lists all passive scanners with its ID, name, enabled state and alert threshold.
*/
pub async fn scanners(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "view", "scanners", params).await
}

/**
 * Show information about the passive scan rule currently being run (if any).
*/
pub async fn current_rule(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "view", "currentRule", params).await
}

/**
 * Gets the maximum number of alerts a passive scan rule should raise.
*/
pub async fn max_alerts_per_rule(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "view", "maxAlertsPerRule", params).await
}

/**
 * Sets whether or not the passive scanning is enabled (Note: the enabled state is not persisted).
*/
pub async fn set_enabled(service: &ZapService, enabled: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("enabled".to_string(), enabled.to_string());
    super::call(service, "pscan", "action", "setEnabled", params).await
}

/**
 * Sets whether or not the passive scan should be performed only on messages that are in scope.
*/
pub async fn set_scan_only_in_scope(service: &ZapService, onlyinscope: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("onlyInScope".to_string(), onlyinscope.to_string());
    super::call(service, "pscan", "action", "setScanOnlyInScope", params).await
}

/**
 * Enables all passive scanners
*/
pub async fn enable_all_scanners(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "action", "enableAllScanners", params).await
}

/**
 * Disables all passive scanners
*/
pub async fn disable_all_scanners(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "action", "disableAllScanners", params).await
}

/**
 * Enables all passive scanners with the given IDs (comma separated list of IDs)
*/
pub async fn enable_scanners(service: &ZapService, ids: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids.to_string());
    super::call(service, "pscan", "action", "enableScanners", params).await
}

/**
 * Disables all passive scanners with the given IDs (comma separated list of IDs)
*/
pub async fn disable_scanners(service: &ZapService, ids: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids.to_string());
    super::call(service, "pscan", "action", "disableScanners", params).await
}

/**
 * Sets the alert threshold of the passive scanner with the given ID, accepted values for alert threshold: OFF, DEFAULT, LOW, MEDIUM and HIGH
*/
pub async fn set_scanner_alert_threshold(service: &ZapService, id: &str, alertthreshold: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    params.insert("alertThreshold".to_string(), alertthreshold.to_string());
    super::call(service, "pscan", "action", "setScannerAlertThreshold", params).await
}

/**
 * Sets the maximum number of alerts a passive scan rule should raise.
*/
pub async fn set_max_alerts_per_rule(service: &ZapService, maxalerts: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("maxAlerts".to_string(), maxalerts.to_string());
    super::call(service, "pscan", "action", "setMaxAlertsPerRule", params).await
}

/**
 * Disables all passive scan tags.
*/
pub async fn disable_all_tags(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "action", "disableAllTags", params).await
}

/**
 * Enables all passive scan tags.
*/
pub async fn enable_all_tags(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "action", "enableAllTags", params).await
}

