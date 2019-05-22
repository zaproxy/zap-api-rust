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
 * Tells whether or not the passive scan should be performed only on messages that are in scope.
*/
pub fn scan_only_in_scope(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "view", "scanOnlyInScope", params)
}

/**
 * The number of records the passive scanner still has to scan
*/
pub fn records_to_scan(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "view", "recordsToScan", params)
}

/**
 * Lists all passive scanners with its ID, name, enabled state and alert threshold.
*/
pub fn scanners(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "view", "scanners", params)
}

/**
 * Show information about the passive scan rule currently being run (if any).
*/
pub fn current_rule(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "view", "currentRule", params)
}

/**
 * Gets the maximum number of alerts a passive scan rule should raise.
*/
pub fn max_alerts_per_rule(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "view", "maxAlertsPerRule", params)
}

/**
 * Sets whether or not the passive scanning is enabled (Note: the enabled state is not persisted).
*/
pub fn set_enabled(service: &ZapService, enabled: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("enabled".to_string(), enabled);
    super::call(service, "pscan", "action", "setEnabled", params)
}

/**
 * Sets whether or not the passive scan should be performed only on messages that are in scope.
*/
pub fn set_scan_only_in_scope(
    service: &ZapService,
    onlyinscope: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("onlyInScope".to_string(), onlyinscope);
    super::call(service, "pscan", "action", "setScanOnlyInScope", params)
}

/**
 * Enables all passive scanners
*/
pub fn enable_all_scanners(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "action", "enableAllScanners", params)
}

/**
 * Disables all passive scanners
*/
pub fn disable_all_scanners(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pscan", "action", "disableAllScanners", params)
}

/**
 * Enables all passive scanners with the given IDs (comma separated list of IDs)
*/
pub fn enable_scanners(service: &ZapService, ids: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids);
    super::call(service, "pscan", "action", "enableScanners", params)
}

/**
 * Disables all passive scanners with the given IDs (comma separated list of IDs)
*/
pub fn disable_scanners(service: &ZapService, ids: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids);
    super::call(service, "pscan", "action", "disableScanners", params)
}

/**
 * Sets the alert threshold of the passive scanner with the given ID, accepted values for alert threshold: OFF, DEFAULT, LOW, MEDIUM and HIGH
*/
pub fn set_scanner_alert_threshold(
    service: &ZapService,
    id: String,
    alertthreshold: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    params.insert("alertThreshold".to_string(), alertthreshold);
    super::call(
        service,
        "pscan",
        "action",
        "setScannerAlertThreshold",
        params,
    )
}

/**
 * Sets the maximum number of alerts a passive scan rule should raise.
*/
pub fn set_max_alerts_per_rule(
    service: &ZapService,
    maxalerts: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("maxAlerts".to_string(), maxalerts);
    super::call(service, "pscan", "action", "setMaxAlertsPerRule", params)
}
