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
pub fn status(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "ascan", "view", "status", params)
}

pub fn scan_progress(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "ascan", "view", "scanProgress", params)
}

/**
 * Gets the IDs of the messages sent during the scan with the given ID. A message can be obtained with 'message' core view.
*/
pub fn messages_ids(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "ascan", "view", "messagesIds", params)
}

/**
 * Gets the IDs of the alerts raised during the scan with the given ID. An alert can be obtained with 'alert' core view.
*/
pub fn alerts_ids(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "ascan", "view", "alertsIds", params)
}

pub fn scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "scans", params)
}

pub fn scan_policy_names(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "scanPolicyNames", params)
}

/**
 * Gets the regexes of URLs excluded from the active scans.
*/
pub fn excluded_from_scan(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "excludedFromScan", params)
}

/**
 * Gets the scanners, optionally, of the given scan policy and/or scanner policy/category ID.
*/
pub fn scanners(
    service: &ZapService,
    scanpolicyname: String,
    policyid: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    params.insert("policyId".to_string(), policyid);
    super::call(service, "ascan", "view", "scanners", params)
}

pub fn policies(
    service: &ZapService,
    scanpolicyname: String,
    policyid: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    params.insert("policyId".to_string(), policyid);
    super::call(service, "ascan", "view", "policies", params)
}

pub fn attack_mode_queue(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "attackModeQueue", params)
}

/**
 * Gets all the parameters that are excluded. For each parameter the following are shown: the name, the URL, and the parameter type.
*/
pub fn excluded_params(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "excludedParams", params)
}

/**
 * Use view excludedParams instead.
 * Deprecated
*/
pub fn option_excluded_param_list(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionExcludedParamList", params)
}

/**
 * Gets all the types of excluded parameters. For each type the following are shown: the ID and the name.
*/
pub fn excluded_param_types(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "excludedParamTypes", params)
}

pub fn option_attack_policy(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionAttackPolicy", params)
}

pub fn option_default_policy(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionDefaultPolicy", params)
}

pub fn option_delay_in_ms(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionDelayInMs", params)
}

pub fn option_handle_anti_csrf_tokens(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "ascan",
        "view",
        "optionHandleAntiCSRFTokens",
        params,
    )
}

pub fn option_host_per_scan(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionHostPerScan", params)
}

pub fn option_max_chart_time_in_mins(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionMaxChartTimeInMins", params)
}

pub fn option_max_results_to_list(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionMaxResultsToList", params)
}

pub fn option_max_rule_duration_in_mins(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "ascan",
        "view",
        "optionMaxRuleDurationInMins",
        params,
    )
}

pub fn option_max_scan_duration_in_mins(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "ascan",
        "view",
        "optionMaxScanDurationInMins",
        params,
    )
}

pub fn option_max_scans_in_ui(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionMaxScansInUI", params)
}

pub fn option_target_params_enabled_rpc(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "ascan",
        "view",
        "optionTargetParamsEnabledRPC",
        params,
    )
}

pub fn option_target_params_injectable(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "ascan",
        "view",
        "optionTargetParamsInjectable",
        params,
    )
}

pub fn option_thread_per_host(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionThreadPerHost", params)
}

/**
 * Tells whether or not the active scanner should add a query parameter to GET request that don't have parameters to start with.
*/
pub fn option_add_query_param(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionAddQueryParam", params)
}

pub fn option_allow_attack_on_start(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionAllowAttackOnStart", params)
}

/**
 * Tells whether or not the active scanner should inject the HTTP request header X-ZAP-Scan-ID, with the ID of the scanner that's sending the requests.
*/
pub fn option_inject_plugin_id_in_header(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "ascan",
        "view",
        "optionInjectPluginIdInHeader",
        params,
    )
}

pub fn option_prompt_in_attack_mode(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionPromptInAttackMode", params)
}

pub fn option_prompt_to_clear_finished_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "ascan",
        "view",
        "optionPromptToClearFinishedScans",
        params,
    )
}

pub fn option_rescan_in_attack_mode(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionRescanInAttackMode", params)
}

/**
 * Tells whether or not the HTTP Headers of all requests should be scanned. Not just requests that send parameters, through the query or request body.
*/
pub fn option_scan_headers_all_requests(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "ascan",
        "view",
        "optionScanHeadersAllRequests",
        params,
    )
}

pub fn option_show_advanced_dialog(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "view", "optionShowAdvancedDialog", params)
}

/**
 * Runs the active scanner against the given URL and/or Context. Optionally, the 'recurse' parameter can be used to scan URLs under the given URL, the parameter 'inScopeOnly' can be used to constrain the scan to URLs that are in scope (ignored if a Context is specified), the parameter 'scanPolicyName' allows to specify the scan policy (if none is given it uses the default scan policy), the parameters 'method' and 'postData' allow to select a given request in conjunction with the given URL.
*/
#[allow(clippy::too_many_arguments)]
pub fn scan(
    service: &ZapService,
    url: String,
    recurse: String,
    inscopeonly: String,
    scanpolicyname: String,
    method: String,
    postdata: String,
    contextid: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url);
    params.insert("recurse".to_string(), recurse);
    params.insert("inScopeOnly".to_string(), inscopeonly);
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    params.insert("method".to_string(), method);
    params.insert("postData".to_string(), postdata);
    params.insert("contextId".to_string(), contextid);
    super::call(service, "ascan", "action", "scan", params)
}

/**
 * Active Scans from the perspective of a User, obtained using the given Context ID and User ID. See 'scan' action for more details.
*/
#[allow(clippy::too_many_arguments)]
pub fn scan_as_user(
    service: &ZapService,
    url: String,
    contextid: String,
    userid: String,
    recurse: String,
    scanpolicyname: String,
    method: String,
    postdata: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url);
    params.insert("contextId".to_string(), contextid);
    params.insert("userId".to_string(), userid);
    params.insert("recurse".to_string(), recurse);
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    params.insert("method".to_string(), method);
    params.insert("postData".to_string(), postdata);
    super::call(service, "ascan", "action", "scanAsUser", params)
}

pub fn pause(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "ascan", "action", "pause", params)
}

pub fn resume(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "ascan", "action", "resume", params)
}

pub fn stop(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "ascan", "action", "stop", params)
}

pub fn remove_scan(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "ascan", "action", "removeScan", params)
}

pub fn pause_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "action", "pauseAllScans", params)
}

pub fn resume_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "action", "resumeAllScans", params)
}

pub fn stop_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "action", "stopAllScans", params)
}

pub fn remove_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "action", "removeAllScans", params)
}

/**
 * Clears the regexes of URLs excluded from the active scans.
*/
pub fn clear_excluded_from_scan(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ascan", "action", "clearExcludedFromScan", params)
}

/**
 * Adds a regex of URLs that should be excluded from the active scans.
*/
pub fn exclude_from_scan(service: &ZapService, regex: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    super::call(service, "ascan", "action", "excludeFromScan", params)
}

/**
 * Enables all scanners of the scan policy with the given name, or the default if none given.
*/
pub fn enable_all_scanners(
    service: &ZapService,
    scanpolicyname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    super::call(service, "ascan", "action", "enableAllScanners", params)
}

/**
 * Disables all scanners of the scan policy with the given name, or the default if none given.
*/
pub fn disable_all_scanners(
    service: &ZapService,
    scanpolicyname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    super::call(service, "ascan", "action", "disableAllScanners", params)
}

/**
 * Enables the scanners with the given IDs (comma separated list of IDs) of the scan policy with the given name, or the default if none given.
*/
pub fn enable_scanners(
    service: &ZapService,
    ids: String,
    scanpolicyname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids);
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    super::call(service, "ascan", "action", "enableScanners", params)
}

/**
 * Disables the scanners with the given IDs (comma separated list of IDs) of the scan policy with the given name, or the default if none given.
*/
pub fn disable_scanners(
    service: &ZapService,
    ids: String,
    scanpolicyname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids);
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    super::call(service, "ascan", "action", "disableScanners", params)
}

pub fn set_enabled_policies(
    service: &ZapService,
    ids: String,
    scanpolicyname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids);
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    super::call(service, "ascan", "action", "setEnabledPolicies", params)
}

pub fn set_policy_attack_strength(
    service: &ZapService,
    id: String,
    attackstrength: String,
    scanpolicyname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    params.insert("attackStrength".to_string(), attackstrength);
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    super::call(
        service,
        "ascan",
        "action",
        "setPolicyAttackStrength",
        params,
    )
}

pub fn set_policy_alert_threshold(
    service: &ZapService,
    id: String,
    alertthreshold: String,
    scanpolicyname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    params.insert("alertThreshold".to_string(), alertthreshold);
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    super::call(
        service,
        "ascan",
        "action",
        "setPolicyAlertThreshold",
        params,
    )
}

pub fn set_scanner_attack_strength(
    service: &ZapService,
    id: String,
    attackstrength: String,
    scanpolicyname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    params.insert("attackStrength".to_string(), attackstrength);
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    super::call(
        service,
        "ascan",
        "action",
        "setScannerAttackStrength",
        params,
    )
}

pub fn set_scanner_alert_threshold(
    service: &ZapService,
    id: String,
    alertthreshold: String,
    scanpolicyname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    params.insert("alertThreshold".to_string(), alertthreshold);
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    super::call(
        service,
        "ascan",
        "action",
        "setScannerAlertThreshold",
        params,
    )
}

pub fn add_scan_policy(
    service: &ZapService,
    scanpolicyname: String,
    alertthreshold: String,
    attackstrength: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    params.insert("alertThreshold".to_string(), alertthreshold);
    params.insert("attackStrength".to_string(), attackstrength);
    super::call(service, "ascan", "action", "addScanPolicy", params)
}

pub fn remove_scan_policy(
    service: &ZapService,
    scanpolicyname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    super::call(service, "ascan", "action", "removeScanPolicy", params)
}

pub fn update_scan_policy(
    service: &ZapService,
    scanpolicyname: String,
    alertthreshold: String,
    attackstrength: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanPolicyName".to_string(), scanpolicyname);
    params.insert("alertThreshold".to_string(), alertthreshold);
    params.insert("attackStrength".to_string(), attackstrength);
    super::call(service, "ascan", "action", "updateScanPolicy", params)
}

/**
 * Imports a Scan Policy using the given file system path.
*/
pub fn import_scan_policy(service: &ZapService, path: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("path".to_string(), path);
    super::call(service, "ascan", "action", "importScanPolicy", params)
}

/**
 * Adds a new parameter excluded from the scan, using the specified name. Optionally sets if the new entry applies to a specific URL (default, all URLs) and sets the ID of the type of the parameter (default, ID of any type). The type IDs can be obtained with the view excludedParamTypes.
*/
pub fn add_excluded_param(
    service: &ZapService,
    name: String,
    typ: String,
    url: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("name".to_string(), name);
    params.insert("type".to_string(), typ);
    params.insert("url".to_string(), url);
    super::call(service, "ascan", "action", "addExcludedParam", params)
}

/**
 * Modifies a parameter excluded from the scan. Allows to modify the name, the URL and the type of parameter. The parameter is selected with its index, which can be obtained with the view excludedParams.
*/
pub fn modify_excluded_param(
    service: &ZapService,
    idx: String,
    name: String,
    typ: String,
    url: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("idx".to_string(), idx);
    params.insert("name".to_string(), name);
    params.insert("type".to_string(), typ);
    params.insert("url".to_string(), url);
    super::call(service, "ascan", "action", "modifyExcludedParam", params)
}

/**
 * Removes a parameter excluded from the scan, with the given index. The index can be obtained with the view excludedParams.
*/
pub fn remove_excluded_param(service: &ZapService, idx: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("idx".to_string(), idx);
    super::call(service, "ascan", "action", "removeExcludedParam", params)
}

/**
 * Skips the scanner using the given IDs of the scan and the scanner.
*/
pub fn skip_scanner(
    service: &ZapService,
    scanid: String,
    scannerid: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    params.insert("scannerId".to_string(), scannerid);
    super::call(service, "ascan", "action", "skipScanner", params)
}

pub fn set_option_attack_policy(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(service, "ascan", "action", "setOptionAttackPolicy", params)
}

pub fn set_option_default_policy(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(service, "ascan", "action", "setOptionDefaultPolicy", params)
}

/**
 * Sets whether or not the active scanner should add a query param to GET requests which do not have parameters to start with.
*/
pub fn set_option_add_query_param(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(service, "ascan", "action", "setOptionAddQueryParam", params)
}

pub fn set_option_allow_attack_on_start(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionAllowAttackOnStart",
        params,
    )
}

pub fn set_option_delay_in_ms(service: &ZapService, integer: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(service, "ascan", "action", "setOptionDelayInMs", params)
}

pub fn set_option_handle_anti_csrf_tokens(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionHandleAntiCSRFTokens",
        params,
    )
}

pub fn set_option_host_per_scan(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(service, "ascan", "action", "setOptionHostPerScan", params)
}

/**
 * Sets whether or not the active scanner should inject the HTTP request header X-ZAP-Scan-ID, with the ID of the scanner that's sending the requests.
*/
pub fn set_option_inject_plugin_id_in_header(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionInjectPluginIdInHeader",
        params,
    )
}

pub fn set_option_max_chart_time_in_mins(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionMaxChartTimeInMins",
        params,
    )
}

pub fn set_option_max_results_to_list(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionMaxResultsToList",
        params,
    )
}

pub fn set_option_max_rule_duration_in_mins(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionMaxRuleDurationInMins",
        params,
    )
}

pub fn set_option_max_scan_duration_in_mins(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionMaxScanDurationInMins",
        params,
    )
}

pub fn set_option_max_scans_in_ui(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(service, "ascan", "action", "setOptionMaxScansInUI", params)
}

pub fn set_option_prompt_in_attack_mode(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionPromptInAttackMode",
        params,
    )
}

pub fn set_option_prompt_to_clear_finished_scans(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionPromptToClearFinishedScans",
        params,
    )
}

pub fn set_option_rescan_in_attack_mode(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionRescanInAttackMode",
        params,
    )
}

/**
 * Sets whether or not the HTTP Headers of all requests should be scanned. Not just requests that send parameters, through the query or request body.
*/
pub fn set_option_scan_headers_all_requests(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionScanHeadersAllRequests",
        params,
    )
}

pub fn set_option_show_advanced_dialog(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionShowAdvancedDialog",
        params,
    )
}

pub fn set_option_target_params_enabled_rpc(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionTargetParamsEnabledRPC",
        params,
    )
}

pub fn set_option_target_params_injectable(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "ascan",
        "action",
        "setOptionTargetParamsInjectable",
        params,
    )
}

pub fn set_option_thread_per_host(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(service, "ascan", "action", "setOptionThreadPerHost", params)
}
