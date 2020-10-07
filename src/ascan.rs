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
 * 
*/
pub async fn status(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "ascan", "view", "status", params).await
}

/**
 * 
*/
pub async fn scan_progress(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "ascan", "view", "scanProgress", params).await
}

/**
 * Gets the IDs of the messages sent during the scan with the given ID. A message can be obtained with 'message' core view.
*/
pub async fn messages_ids(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "ascan", "view", "messagesIds", params).await
}

/**
 * Gets the IDs of the alerts raised during the scan with the given ID. An alert can be obtained with 'alert' core view.
*/
pub async fn alerts_ids(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "ascan", "view", "alertsIds", params).await
}

/**
 * 
*/
pub async fn scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "scans", params).await
}

/**
 * 
*/
pub async fn scan_policy_names(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "scanPolicyNames", params).await
}

/**
 * Gets the regexes of URLs excluded from the active scans.
*/
pub async fn excluded_from_scan(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "excludedFromScan", params).await
}

/**
 * Gets the scanners, optionally, of the given scan policy and/or scanner policy/category ID.
*/
pub async fn scanners(service: &ZapService, scanpolicyname: &str, policyid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    params.insert("policyId".to_string(), policyid.to_string());
    super::call(service, "ascan", "view", "scanners", params).await
}

/**
 * 
*/
pub async fn policies(service: &ZapService, scanpolicyname: &str, policyid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    params.insert("policyId".to_string(), policyid.to_string());
    super::call(service, "ascan", "view", "policies", params).await
}

/**
 * 
*/
pub async fn attack_mode_queue(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "attackModeQueue", params).await
}

/**
 * Gets all the parameters that are excluded. For each parameter the following are shown: the name, the URL, and the parameter type.
*/
pub async fn excluded_params(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "excludedParams", params).await
}

/**
 * Use view excludedParams instead.
*/
#[deprecated]
pub async fn option_excluded_param_list(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionExcludedParamList", params).await
}

/**
 * Gets all the types of excluded parameters. For each type the following are shown: the ID and the name.
*/
pub async fn excluded_param_types(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "excludedParamTypes", params).await
}

/**
 * 
*/
pub async fn option_attack_policy(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionAttackPolicy", params).await
}

/**
 * 
*/
pub async fn option_default_policy(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionDefaultPolicy", params).await
}

/**
 * 
*/
pub async fn option_delay_in_ms(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionDelayInMs", params).await
}

/**
 * 
*/
pub async fn option_handle_anti_csrf_tokens(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionHandleAntiCSRFTokens", params).await
}

/**
 * 
*/
pub async fn option_host_per_scan(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionHostPerScan", params).await
}

/**
 * 
*/
pub async fn option_max_chart_time_in_mins(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionMaxChartTimeInMins", params).await
}

/**
 * 
*/
pub async fn option_max_results_to_list(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionMaxResultsToList", params).await
}

/**
 * 
*/
pub async fn option_max_rule_duration_in_mins(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionMaxRuleDurationInMins", params).await
}

/**
 * 
*/
pub async fn option_max_scan_duration_in_mins(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionMaxScanDurationInMins", params).await
}

/**
 * 
*/
pub async fn option_max_scans_in_ui(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionMaxScansInUI", params).await
}

/**
 * 
*/
pub async fn option_target_params_enabled_rpc(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionTargetParamsEnabledRPC", params).await
}

/**
 * 
*/
pub async fn option_target_params_injectable(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionTargetParamsInjectable", params).await
}

/**
 * 
*/
pub async fn option_thread_per_host(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionThreadPerHost", params).await
}

/**
 * Tells whether or not the active scanner should add a query parameter to GET request that don't have parameters to start with.
*/
pub async fn option_add_query_param(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionAddQueryParam", params).await
}

/**
 * 
*/
pub async fn option_allow_attack_on_start(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionAllowAttackOnStart", params).await
}

/**
 * Tells whether or not the active scanner should inject the HTTP request header X-ZAP-Scan-ID, with the ID of the scanner that's sending the requests.
*/
pub async fn option_inject_plugin_id_in_header(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionInjectPluginIdInHeader", params).await
}

/**
 * 
*/
pub async fn option_prompt_in_attack_mode(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionPromptInAttackMode", params).await
}

/**
 * 
*/
pub async fn option_prompt_to_clear_finished_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionPromptToClearFinishedScans", params).await
}

/**
 * 
*/
pub async fn option_rescan_in_attack_mode(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionRescanInAttackMode", params).await
}

/**
 * Tells whether or not the HTTP Headers of all requests should be scanned. Not just requests that send parameters, through the query or request body.
*/
pub async fn option_scan_headers_all_requests(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionScanHeadersAllRequests", params).await
}

/**
 * 
*/
pub async fn option_show_advanced_dialog(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionShowAdvancedDialog", params).await
}

/**
 * Runs the active scanner against the given URL and/or Context. Optionally, the 'recurse' parameter can be used to scan URLs under the given URL, the parameter 'inScopeOnly' can be used to constrain the scan to URLs that are in scope (ignored if a Context is specified), the parameter 'scanPolicyName' allows to specify the scan policy (if none is given it uses the default scan policy), the parameters 'method' and 'postData' allow to select a given request in conjunction with the given URL.
*/
#[allow(clippy::too_many_arguments)]
pub async fn scan(service: &ZapService, url: &str, recurse: &str, inscopeonly: &str, scanpolicyname: &str, method: &str, postdata: &str, contextid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url.to_string());
    params.insert("recurse".to_string(), recurse.to_string());
    params.insert("inScopeOnly".to_string(), inscopeonly.to_string());
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    params.insert("method".to_string(), method.to_string());
    params.insert("postData".to_string(), postdata.to_string());
    params.insert("contextId".to_string(), contextid.to_string());
    super::call(service, "ascan", "action", "scan", params).await
}

/**
 * Active Scans from the perspective of a User, obtained using the given Context ID and User ID. See 'scan' action for more details.
*/
#[allow(clippy::too_many_arguments)]
pub async fn scan_as_user(service: &ZapService, url: &str, contextid: &str, userid: &str, recurse: &str, scanpolicyname: &str, method: &str, postdata: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url.to_string());
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("userId".to_string(), userid.to_string());
    params.insert("recurse".to_string(), recurse.to_string());
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    params.insert("method".to_string(), method.to_string());
    params.insert("postData".to_string(), postdata.to_string());
    super::call(service, "ascan", "action", "scanAsUser", params).await
}

/**
 * 
*/
pub async fn pause(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "ascan", "action", "pause", params).await
}

/**
 * 
*/
pub async fn resume(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "ascan", "action", "resume", params).await
}

/**
 * 
*/
pub async fn stop(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "ascan", "action", "stop", params).await
}

/**
 * 
*/
pub async fn remove_scan(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "ascan", "action", "removeScan", params).await
}

/**
 * 
*/
pub async fn pause_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "action", "pauseAllScans", params).await
}

/**
 * 
*/
pub async fn resume_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "action", "resumeAllScans", params).await
}

/**
 * 
*/
pub async fn stop_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "action", "stopAllScans", params).await
}

/**
 * 
*/
pub async fn remove_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "action", "removeAllScans", params).await
}

/**
 * Clears the regexes of URLs excluded from the active scans.
*/
pub async fn clear_excluded_from_scan(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "action", "clearExcludedFromScan", params).await
}

/**
 * Adds a regex of URLs that should be excluded from the active scans.
*/
pub async fn exclude_from_scan(service: &ZapService, regex: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex.to_string());
    super::call(service, "ascan", "action", "excludeFromScan", params).await
}

/**
 * Enables all scanners of the scan policy with the given name, or the default if none given.
*/
pub async fn enable_all_scanners(service: &ZapService, scanpolicyname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    super::call(service, "ascan", "action", "enableAllScanners", params).await
}

/**
 * Disables all scanners of the scan policy with the given name, or the default if none given.
*/
pub async fn disable_all_scanners(service: &ZapService, scanpolicyname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    super::call(service, "ascan", "action", "disableAllScanners", params).await
}

/**
 * Enables the scanners with the given IDs (comma separated list of IDs) of the scan policy with the given name, or the default if none given.
*/
pub async fn enable_scanners(service: &ZapService, ids: &str, scanpolicyname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids.to_string());
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    super::call(service, "ascan", "action", "enableScanners", params).await
}

/**
 * Disables the scanners with the given IDs (comma separated list of IDs) of the scan policy with the given name, or the default if none given.
*/
pub async fn disable_scanners(service: &ZapService, ids: &str, scanpolicyname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids.to_string());
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    super::call(service, "ascan", "action", "disableScanners", params).await
}

/**
 * 
*/
pub async fn set_enabled_policies(service: &ZapService, ids: &str, scanpolicyname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids.to_string());
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    super::call(service, "ascan", "action", "setEnabledPolicies", params).await
}

/**
 * 
*/
pub async fn set_policy_attack_strength(service: &ZapService, id: &str, attackstrength: &str, scanpolicyname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    params.insert("attackStrength".to_string(), attackstrength.to_string());
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    super::call(service, "ascan", "action", "setPolicyAttackStrength", params).await
}

/**
 * 
*/
pub async fn set_policy_alert_threshold(service: &ZapService, id: &str, alertthreshold: &str, scanpolicyname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    params.insert("alertThreshold".to_string(), alertthreshold.to_string());
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    super::call(service, "ascan", "action", "setPolicyAlertThreshold", params).await
}

/**
 * 
*/
pub async fn set_scanner_attack_strength(service: &ZapService, id: &str, attackstrength: &str, scanpolicyname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    params.insert("attackStrength".to_string(), attackstrength.to_string());
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    super::call(service, "ascan", "action", "setScannerAttackStrength", params).await
}

/**
 * 
*/
pub async fn set_scanner_alert_threshold(service: &ZapService, id: &str, alertthreshold: &str, scanpolicyname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    params.insert("alertThreshold".to_string(), alertthreshold.to_string());
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    super::call(service, "ascan", "action", "setScannerAlertThreshold", params).await
}

/**
 * 
*/
pub async fn add_scan_policy(service: &ZapService, scanpolicyname: &str, alertthreshold: &str, attackstrength: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    params.insert("alertThreshold".to_string(), alertthreshold.to_string());
    params.insert("attackStrength".to_string(), attackstrength.to_string());
    super::call(service, "ascan", "action", "addScanPolicy", params).await
}

/**
 * 
*/
pub async fn remove_scan_policy(service: &ZapService, scanpolicyname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    super::call(service, "ascan", "action", "removeScanPolicy", params).await
}

/**
 * 
*/
pub async fn update_scan_policy(service: &ZapService, scanpolicyname: &str, alertthreshold: &str, attackstrength: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname.to_string());
    params.insert("alertThreshold".to_string(), alertthreshold.to_string());
    params.insert("attackStrength".to_string(), attackstrength.to_string());
    super::call(service, "ascan", "action", "updateScanPolicy", params).await
}

/**
 * Imports a Scan Policy using the given file system path.
*/
pub async fn import_scan_policy(service: &ZapService, path: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("path".to_string(), path.to_string());
    super::call(service, "ascan", "action", "importScanPolicy", params).await
}

/**
 * Adds a new parameter excluded from the scan, using the specified name. Optionally sets if the new entry applies to a specific URL (default, all URLs) and sets the ID of the type of the parameter (default, ID of any type). The type IDs can be obtained with the view excludedParamTypes. 
*/
pub async fn add_excluded_param(service: &ZapService, name: &str, typ: &str, url: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("name".to_string(), name.to_string());
    params.insert("type".to_string(), typ.to_string());
    params.insert("url".to_string(), url.to_string());
    super::call(service, "ascan", "action", "addExcludedParam", params).await
}

/**
 * Modifies a parameter excluded from the scan. Allows to modify the name, the URL and the type of parameter. The parameter is selected with its index, which can be obtained with the view excludedParams.
*/
pub async fn modify_excluded_param(service: &ZapService, idx: &str, name: &str, typ: &str, url: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("idx".to_string(), idx.to_string());
    params.insert("name".to_string(), name.to_string());
    params.insert("type".to_string(), typ.to_string());
    params.insert("url".to_string(), url.to_string());
    super::call(service, "ascan", "action", "modifyExcludedParam", params).await
}

/**
 * Removes a parameter excluded from the scan, with the given index. The index can be obtained with the view excludedParams.
*/
pub async fn remove_excluded_param(service: &ZapService, idx: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("idx".to_string(), idx.to_string());
    super::call(service, "ascan", "action", "removeExcludedParam", params).await
}

/**
 * Skips the scanner using the given IDs of the scan and the scanner.
*/
pub async fn skip_scanner(service: &ZapService, scanid: &str, scannerid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    params.insert("scannerId".to_string(), scannerid.to_string());
    super::call(service, "ascan", "action", "skipScanner", params).await
}

/**
 * 
*/
pub async fn set_option_attack_policy(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "ascan", "action", "setOptionAttackPolicy", params).await
}

/**
 * 
*/
pub async fn set_option_default_policy(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "ascan", "action", "setOptionDefaultPolicy", params).await
}

/**
 * Sets whether or not the active scanner should add a query param to GET requests which do not have parameters to start with.
*/
pub async fn set_option_add_query_param(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "ascan", "action", "setOptionAddQueryParam", params).await
}

/**
 * 
*/
pub async fn set_option_allow_attack_on_start(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "ascan", "action", "setOptionAllowAttackOnStart", params).await
}

/**
 * 
*/
pub async fn set_option_delay_in_ms(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "ascan", "action", "setOptionDelayInMs", params).await
}

/**
 * 
*/
pub async fn set_option_handle_anti_csrf_tokens(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "ascan", "action", "setOptionHandleAntiCSRFTokens", params).await
}

/**
 * 
*/
pub async fn set_option_host_per_scan(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "ascan", "action", "setOptionHostPerScan", params).await
}

/**
 * Sets whether or not the active scanner should inject the HTTP request header X-ZAP-Scan-ID, with the ID of the scanner that's sending the requests.
*/
pub async fn set_option_inject_plugin_id_in_header(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "ascan", "action", "setOptionInjectPluginIdInHeader", params).await
}

/**
 * 
*/
pub async fn set_option_max_chart_time_in_mins(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "ascan", "action", "setOptionMaxChartTimeInMins", params).await
}

/**
 * 
*/
pub async fn set_option_max_results_to_list(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "ascan", "action", "setOptionMaxResultsToList", params).await
}

/**
 * 
*/
pub async fn set_option_max_rule_duration_in_mins(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "ascan", "action", "setOptionMaxRuleDurationInMins", params).await
}

/**
 * 
*/
pub async fn set_option_max_scan_duration_in_mins(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "ascan", "action", "setOptionMaxScanDurationInMins", params).await
}

/**
 * 
*/
pub async fn set_option_max_scans_in_ui(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "ascan", "action", "setOptionMaxScansInUI", params).await
}

/**
 * 
*/
pub async fn set_option_prompt_in_attack_mode(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "ascan", "action", "setOptionPromptInAttackMode", params).await
}

/**
 * 
*/
pub async fn set_option_prompt_to_clear_finished_scans(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "ascan", "action", "setOptionPromptToClearFinishedScans", params).await
}

/**
 * 
*/
pub async fn set_option_rescan_in_attack_mode(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "ascan", "action", "setOptionRescanInAttackMode", params).await
}

/**
 * Sets whether or not the HTTP Headers of all requests should be scanned. Not just requests that send parameters, through the query or request body.
*/
pub async fn set_option_scan_headers_all_requests(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "ascan", "action", "setOptionScanHeadersAllRequests", params).await
}

/**
 * 
*/
pub async fn set_option_show_advanced_dialog(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "ascan", "action", "setOptionShowAdvancedDialog", params).await
}

/**
 * 
*/
pub async fn set_option_target_params_enabled_rpc(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "ascan", "action", "setOptionTargetParamsEnabledRPC", params).await
}

/**
 * 
*/
pub async fn set_option_target_params_injectable(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "ascan", "action", "setOptionTargetParamsInjectable", params).await
}

/**
 * 
*/
pub async fn set_option_thread_per_host(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "ascan", "action", "setOptionThreadPerHost", params).await
}

