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
    super::call(service, "spider", "view", "status", params)
}

pub fn results(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "spider", "view", "results", params)
}

pub fn full_results(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "spider", "view", "fullResults", params)
}

pub fn scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "scans", params)
}

/**
 * Gets the regexes of URLs excluded from the spider scans.
*/
pub fn excluded_from_scan(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "excludedFromScan", params)
}

/**
 * Returns a list of unique URLs from the history table based on HTTP messages added by the Spider.
*/
pub fn all_urls(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "allUrls", params)
}

/**
 * Returns a list of the names of the nodes added to the Sites tree by the specified scan.
*/
pub fn added_nodes(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "spider", "view", "addedNodes", params)
}

/**
 * Gets all the domains that are always in scope. For each domain the following are shown: the index, the value (domain), if enabled, and if specified as a regex.
*/
pub fn domains_always_in_scope(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "domainsAlwaysInScope", params)
}

/**
 * Use view domainsAlwaysInScope instead.
 * Deprecated
*/
pub fn option_domains_always_in_scope(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "spider",
        "view",
        "optionDomainsAlwaysInScope",
        params,
    )
}

/**
 * Use view domainsAlwaysInScope instead.
 * Deprecated
*/
pub fn option_domains_always_in_scope_enabled(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "spider",
        "view",
        "optionDomainsAlwaysInScopeEnabled",
        params,
    )
}

pub fn option_handle_parameters(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionHandleParameters", params)
}

/**
 * Gets the maximum number of child nodes (per node) that can be crawled, 0 means no limit.
*/
pub fn option_max_children(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionMaxChildren", params)
}

/**
 * Gets the maximum depth the spider can crawl, 0 if unlimited.
*/
pub fn option_max_depth(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionMaxDepth", params)
}

pub fn option_max_duration(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionMaxDuration", params)
}

/**
 * Gets the maximum size, in bytes, that a response might have to be parsed.
*/
pub fn option_max_parse_size_bytes(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionMaxParseSizeBytes", params)
}

pub fn option_max_scans_in_ui(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionMaxScansInUI", params)
}

pub fn option_request_wait_time(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionRequestWaitTime", params)
}

pub fn option_scope(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionScope", params)
}

pub fn option_scope_text(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionScopeText", params)
}

pub fn option_skip_url_string(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionSkipURLString", params)
}

pub fn option_thread_count(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionThreadCount", params)
}

pub fn option_user_agent(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionUserAgent", params)
}

/**
 * Gets whether or not a spider process should accept cookies while spidering.
*/
pub fn option_accept_cookies(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionAcceptCookies", params)
}

pub fn option_handle_o_data_parameters_visited(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "spider",
        "view",
        "optionHandleODataParametersVisited",
        params,
    )
}

pub fn option_parse_comments(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionParseComments", params)
}

pub fn option_parse_git(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionParseGit", params)
}

pub fn option_parse_robots_txt(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionParseRobotsTxt", params)
}

pub fn option_parse_svn_entries(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionParseSVNEntries", params)
}

pub fn option_parse_sitemap_xml(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionParseSitemapXml", params)
}

pub fn option_post_form(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionPostForm", params)
}

pub fn option_process_form(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionProcessForm", params)
}

/**
 * Gets whether or not the 'Referer' header should be sent while spidering.
*/
pub fn option_send_referer_header(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionSendRefererHeader", params)
}

pub fn option_show_advanced_dialog(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "spider",
        "view",
        "optionShowAdvancedDialog",
        params,
    )
}

/**
 * Runs the spider against the given URL (or context). Optionally, the 'maxChildren' parameter can be set to limit the number of children scanned, the 'recurse' parameter can be used to prevent the spider from seeding recursively, the parameter 'contextName' can be used to constrain the scan to a Context and the parameter 'subtreeOnly' allows to restrict the spider under a site's subtree (using the specified 'url').
*/
pub fn scan(
    service: &ZapService,
    url: String,
    maxchildren: String,
    recurse: String,
    contextname: String,
    subtreeonly: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url);
    params.insert("maxChildren".to_string(), maxchildren);
    params.insert("recurse".to_string(), recurse);
    params.insert("contextName".to_string(), contextname);
    params.insert("subtreeOnly".to_string(), subtreeonly);
    super::call(service, "spider", "action", "scan", params)
}

/**
 * Runs the spider from the perspective of a User, obtained using the given Context ID and User ID. See 'scan' action for more details.
*/
pub fn scan_as_user(
    service: &ZapService,
    contextid: String,
    userid: String,
    url: String,
    maxchildren: String,
    recurse: String,
    subtreeonly: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid);
    params.insert("userId".to_string(), userid);
    params.insert("url".to_string(), url);
    params.insert("maxChildren".to_string(), maxchildren);
    params.insert("recurse".to_string(), recurse);
    params.insert("subtreeOnly".to_string(), subtreeonly);
    super::call(service, "spider", "action", "scanAsUser", params)
}

pub fn pause(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "spider", "action", "pause", params)
}

pub fn resume(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "spider", "action", "resume", params)
}

pub fn stop(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "spider", "action", "stop", params)
}

pub fn remove_scan(service: &ZapService, scanid: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid);
    super::call(service, "spider", "action", "removeScan", params)
}

pub fn pause_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "action", "pauseAllScans", params)
}

pub fn resume_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "action", "resumeAllScans", params)
}

pub fn stop_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "action", "stopAllScans", params)
}

pub fn remove_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "action", "removeAllScans", params)
}

/**
 * Clears the regexes of URLs excluded from the spider scans.
*/
pub fn clear_excluded_from_scan(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "action", "clearExcludedFromScan", params)
}

/**
 * Adds a regex of URLs that should be excluded from the spider scans.
*/
pub fn exclude_from_scan(service: &ZapService, regex: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    super::call(service, "spider", "action", "excludeFromScan", params)
}

/**
 * Adds a new domain that's always in scope, using the specified value. Optionally sets if the new entry is enabled (default, true) and whether or not the new value is specified as a regex (default, false).
*/
pub fn add_domain_always_in_scope(
    service: &ZapService,
    value: String,
    isregex: String,
    isenabled: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("value".to_string(), value);
    params.insert("isRegex".to_string(), isregex);
    params.insert("isEnabled".to_string(), isenabled);
    super::call(
        service,
        "spider",
        "action",
        "addDomainAlwaysInScope",
        params,
    )
}

/**
 * Modifies a domain that's always in scope. Allows to modify the value, if enabled or if a regex. The domain is selected with its index, which can be obtained with the view domainsAlwaysInScope.
*/
pub fn modify_domain_always_in_scope(
    service: &ZapService,
    idx: String,
    value: String,
    isregex: String,
    isenabled: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("idx".to_string(), idx);
    params.insert("value".to_string(), value);
    params.insert("isRegex".to_string(), isregex);
    params.insert("isEnabled".to_string(), isenabled);
    super::call(
        service,
        "spider",
        "action",
        "modifyDomainAlwaysInScope",
        params,
    )
}

/**
 * Removes a domain that's always in scope, with the given index. The index can be obtained with the view domainsAlwaysInScope.
*/
pub fn remove_domain_always_in_scope(
    service: &ZapService,
    idx: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("idx".to_string(), idx);
    super::call(
        service,
        "spider",
        "action",
        "removeDomainAlwaysInScope",
        params,
    )
}

/**
 * Enables all domains that are always in scope.
*/
pub fn enable_all_domains_always_in_scope(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "spider",
        "action",
        "enableAllDomainsAlwaysInScope",
        params,
    )
}

/**
 * Disables all domains that are always in scope.
*/
pub fn disable_all_domains_always_in_scope(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "spider",
        "action",
        "disableAllDomainsAlwaysInScope",
        params,
    )
}

pub fn set_option_handle_parameters(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "spider",
        "action",
        "setOptionHandleParameters",
        params,
    )
}

/**
 * Use actions [add|modify|remove]DomainAlwaysInScope instead.
 * Deprecated Option no longer in effective use.
*/
pub fn set_option_scope_string(service: &ZapService, string: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(service, "spider", "action", "setOptionScopeString", params)
}

pub fn set_option_skip_url_string(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "spider",
        "action",
        "setOptionSkipURLString",
        params,
    )
}

pub fn set_option_user_agent(service: &ZapService, string: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(service, "spider", "action", "setOptionUserAgent", params)
}

/**
 * Sets whether or not a spider process should accept cookies while spidering.
*/
pub fn set_option_accept_cookies(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "spider",
        "action",
        "setOptionAcceptCookies",
        params,
    )
}

pub fn set_option_handle_o_data_parameters_visited(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "spider",
        "action",
        "setOptionHandleODataParametersVisited",
        params,
    )
}

/**
 * Sets the maximum number of child nodes (per node) that can be crawled, 0 means no limit.
*/
pub fn set_option_max_children(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(service, "spider", "action", "setOptionMaxChildren", params)
}

/**
 * Sets the maximum depth the spider can crawl, 0 for unlimited depth.
*/
pub fn set_option_max_depth(service: &ZapService, integer: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(service, "spider", "action", "setOptionMaxDepth", params)
}

pub fn set_option_max_duration(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(service, "spider", "action", "setOptionMaxDuration", params)
}

/**
 * Sets the maximum size, in bytes, that a response might have to be parsed. This allows the spider to skip big responses/files.
*/
pub fn set_option_max_parse_size_bytes(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "spider",
        "action",
        "setOptionMaxParseSizeBytes",
        params,
    )
}

pub fn set_option_max_scans_in_ui(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(service, "spider", "action", "setOptionMaxScansInUI", params)
}

pub fn set_option_parse_comments(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "spider",
        "action",
        "setOptionParseComments",
        params,
    )
}

pub fn set_option_parse_git(service: &ZapService, boolean: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(service, "spider", "action", "setOptionParseGit", params)
}

pub fn set_option_parse_robots_txt(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "spider",
        "action",
        "setOptionParseRobotsTxt",
        params,
    )
}

pub fn set_option_parse_svn_entries(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "spider",
        "action",
        "setOptionParseSVNEntries",
        params,
    )
}

pub fn set_option_parse_sitemap_xml(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "spider",
        "action",
        "setOptionParseSitemapXml",
        params,
    )
}

pub fn set_option_post_form(service: &ZapService, boolean: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(service, "spider", "action", "setOptionPostForm", params)
}

pub fn set_option_process_form(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(service, "spider", "action", "setOptionProcessForm", params)
}

pub fn set_option_request_wait_time(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "spider",
        "action",
        "setOptionRequestWaitTime",
        params,
    )
}

/**
 * Sets whether or not the 'Referer' header should be sent while spidering.
*/
pub fn set_option_send_referer_header(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "spider",
        "action",
        "setOptionSendRefererHeader",
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
        "spider",
        "action",
        "setOptionShowAdvancedDialog",
        params,
    )
}

pub fn set_option_thread_count(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(service, "spider", "action", "setOptionThreadCount", params)
}
