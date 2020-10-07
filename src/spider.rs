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
    super::call(service, "spider", "view", "status", params).await
}

/**
 * 
*/
pub async fn results(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "spider", "view", "results", params).await
}

/**
 * 
*/
pub async fn full_results(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "spider", "view", "fullResults", params).await
}

/**
 * 
*/
pub async fn scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "scans", params).await
}

/**
 * Gets the regexes of URLs excluded from the spider scans.
*/
pub async fn excluded_from_scan(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "excludedFromScan", params).await
}

/**
 * Returns a list of unique URLs from the history table based on HTTP messages added by the Spider.
*/
pub async fn all_urls(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "allUrls", params).await
}

/**
 * Returns a list of the names of the nodes added to the Sites tree by the specified scan.
*/
pub async fn added_nodes(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "spider", "view", "addedNodes", params).await
}

/**
 * Gets all the domains that are always in scope. For each domain the following are shown: the index, the value (domain), if enabled, and if specified as a regex.
*/
pub async fn domains_always_in_scope(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "domainsAlwaysInScope", params).await
}

/**
 * Use view domainsAlwaysInScope instead.
*/
#[deprecated]
pub async fn option_domains_always_in_scope(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionDomainsAlwaysInScope", params).await
}

/**
 * Use view domainsAlwaysInScope instead.
*/
#[deprecated]
pub async fn option_domains_always_in_scope_enabled(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionDomainsAlwaysInScopeEnabled", params).await
}

/**
 * 
*/
pub async fn option_handle_parameters(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionHandleParameters", params).await
}

/**
 * Gets the maximum number of child nodes (per node) that can be crawled, 0 means no limit.
*/
pub async fn option_max_children(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionMaxChildren", params).await
}

/**
 * Gets the maximum depth the spider can crawl, 0 if unlimited.
*/
pub async fn option_max_depth(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionMaxDepth", params).await
}

/**
 * 
*/
pub async fn option_max_duration(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionMaxDuration", params).await
}

/**
 * Gets the maximum size, in bytes, that a response might have to be parsed.
*/
pub async fn option_max_parse_size_bytes(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionMaxParseSizeBytes", params).await
}

/**
 * 
*/
pub async fn option_max_scans_in_ui(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionMaxScansInUI", params).await
}

/**
 * 
*/
pub async fn option_request_wait_time(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionRequestWaitTime", params).await
}

/**
 * 
*/
#[deprecated(note="Option no longer in effective use.")]
pub async fn option_scope(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionScope", params).await
}

/**
 * 
*/
#[deprecated(note="Option no longer in effective use.")]
pub async fn option_scope_text(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionScopeText", params).await
}

/**
 * 
*/
pub async fn option_skip_url_string(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionSkipURLString", params).await
}

/**
 * 
*/
pub async fn option_thread_count(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionThreadCount", params).await
}

/**
 * 
*/
pub async fn option_user_agent(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionUserAgent", params).await
}

/**
 * Gets whether or not a spider process should accept cookies while spidering.
*/
pub async fn option_accept_cookies(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionAcceptCookies", params).await
}

/**
 * 
*/
pub async fn option_handle_o_data_parameters_visited(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionHandleODataParametersVisited", params).await
}

/**
 * 
*/
pub async fn option_parse_comments(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionParseComments", params).await
}

/**
 * 
*/
pub async fn option_parse_git(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionParseGit", params).await
}

/**
 * 
*/
pub async fn option_parse_robots_txt(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionParseRobotsTxt", params).await
}

/**
 * 
*/
pub async fn option_parse_svn_entries(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionParseSVNEntries", params).await
}

/**
 * 
*/
pub async fn option_parse_sitemap_xml(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionParseSitemapXml", params).await
}

/**
 * 
*/
pub async fn option_post_form(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionPostForm", params).await
}

/**
 * 
*/
pub async fn option_process_form(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionProcessForm", params).await
}

/**
 * Gets whether or not the 'Referer' header should be sent while spidering.
*/
pub async fn option_send_referer_header(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionSendRefererHeader", params).await
}

/**
 * 
*/
pub async fn option_show_advanced_dialog(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "view", "optionShowAdvancedDialog", params).await
}

/**
 * Runs the spider against the given URL (or context). Optionally, the 'maxChildren' parameter can be set to limit the number of children scanned, the 'recurse' parameter can be used to prevent the spider from seeding recursively, the parameter 'contextName' can be used to constrain the scan to a Context and the parameter 'subtreeOnly' allows to restrict the spider under a site's subtree (using the specified 'url').
*/
pub async fn scan(service: &ZapService, url: &str, maxchildren: &str, recurse: &str, contextname: &str, subtreeonly: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url.to_string());
    params.insert("maxChildren".to_string(), maxchildren.to_string());
    params.insert("recurse".to_string(), recurse.to_string());
    params.insert("contextName".to_string(), contextname.to_string());
    params.insert("subtreeOnly".to_string(), subtreeonly.to_string());
    super::call(service, "spider", "action", "scan", params).await
}

/**
 * Runs the spider from the perspective of a User, obtained using the given Context ID and User ID. See 'scan' action for more details.
*/
pub async fn scan_as_user(service: &ZapService, contextid: &str, userid: &str, url: &str, maxchildren: &str, recurse: &str, subtreeonly: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("userId".to_string(), userid.to_string());
    params.insert("url".to_string(), url.to_string());
    params.insert("maxChildren".to_string(), maxchildren.to_string());
    params.insert("recurse".to_string(), recurse.to_string());
    params.insert("subtreeOnly".to_string(), subtreeonly.to_string());
    super::call(service, "spider", "action", "scanAsUser", params).await
}

/**
 * 
*/
pub async fn pause(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "spider", "action", "pause", params).await
}

/**
 * 
*/
pub async fn resume(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "spider", "action", "resume", params).await
}

/**
 * 
*/
pub async fn stop(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "spider", "action", "stop", params).await
}

/**
 * 
*/
pub async fn remove_scan(service: &ZapService, scanid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scanId".to_string(), scanid.to_string());
    super::call(service, "spider", "action", "removeScan", params).await
}

/**
 * 
*/
pub async fn pause_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "action", "pauseAllScans", params).await
}

/**
 * 
*/
pub async fn resume_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "action", "resumeAllScans", params).await
}

/**
 * 
*/
pub async fn stop_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "action", "stopAllScans", params).await
}

/**
 * 
*/
pub async fn remove_all_scans(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "action", "removeAllScans", params).await
}

/**
 * Clears the regexes of URLs excluded from the spider scans.
*/
pub async fn clear_excluded_from_scan(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "action", "clearExcludedFromScan", params).await
}

/**
 * Adds a regex of URLs that should be excluded from the spider scans.
*/
pub async fn exclude_from_scan(service: &ZapService, regex: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex.to_string());
    super::call(service, "spider", "action", "excludeFromScan", params).await
}

/**
 * Adds a new domain that's always in scope, using the specified value. Optionally sets if the new entry is enabled (default, true) and whether or not the new value is specified as a regex (default, false).
*/
pub async fn add_domain_always_in_scope(service: &ZapService, value: &str, isregex: &str, isenabled: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("value".to_string(), value.to_string());
    params.insert("isRegex".to_string(), isregex.to_string());
    params.insert("isEnabled".to_string(), isenabled.to_string());
    super::call(service, "spider", "action", "addDomainAlwaysInScope", params).await
}

/**
 * Modifies a domain that's always in scope. Allows to modify the value, if enabled or if a regex. The domain is selected with its index, which can be obtained with the view domainsAlwaysInScope.
*/
pub async fn modify_domain_always_in_scope(service: &ZapService, idx: &str, value: &str, isregex: &str, isenabled: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("idx".to_string(), idx.to_string());
    params.insert("value".to_string(), value.to_string());
    params.insert("isRegex".to_string(), isregex.to_string());
    params.insert("isEnabled".to_string(), isenabled.to_string());
    super::call(service, "spider", "action", "modifyDomainAlwaysInScope", params).await
}

/**
 * Removes a domain that's always in scope, with the given index. The index can be obtained with the view domainsAlwaysInScope.
*/
pub async fn remove_domain_always_in_scope(service: &ZapService, idx: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("idx".to_string(), idx.to_string());
    super::call(service, "spider", "action", "removeDomainAlwaysInScope", params).await
}

/**
 * Enables all domains that are always in scope.
*/
pub async fn enable_all_domains_always_in_scope(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "action", "enableAllDomainsAlwaysInScope", params).await
}

/**
 * Disables all domains that are always in scope.
*/
pub async fn disable_all_domains_always_in_scope(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "spider", "action", "disableAllDomainsAlwaysInScope", params).await
}

/**
 * 
*/
pub async fn set_option_handle_parameters(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "spider", "action", "setOptionHandleParameters", params).await
}

/**
 * Use actions [add|modify|remove]DomainAlwaysInScope instead.
*/
#[deprecated(note="Option no longer in effective use.")]
pub async fn set_option_scope_string(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "spider", "action", "setOptionScopeString", params).await
}

/**
 * 
*/
pub async fn set_option_skip_url_string(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "spider", "action", "setOptionSkipURLString", params).await
}

/**
 * 
*/
pub async fn set_option_user_agent(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "spider", "action", "setOptionUserAgent", params).await
}

/**
 * Sets whether or not a spider process should accept cookies while spidering.
*/
pub async fn set_option_accept_cookies(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "spider", "action", "setOptionAcceptCookies", params).await
}

/**
 * 
*/
pub async fn set_option_handle_o_data_parameters_visited(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "spider", "action", "setOptionHandleODataParametersVisited", params).await
}

/**
 * Sets the maximum number of child nodes (per node) that can be crawled, 0 means no limit.
*/
pub async fn set_option_max_children(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "spider", "action", "setOptionMaxChildren", params).await
}

/**
 * Sets the maximum depth the spider can crawl, 0 for unlimited depth.
*/
pub async fn set_option_max_depth(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "spider", "action", "setOptionMaxDepth", params).await
}

/**
 * 
*/
pub async fn set_option_max_duration(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "spider", "action", "setOptionMaxDuration", params).await
}

/**
 * Sets the maximum size, in bytes, that a response might have to be parsed. This allows the spider to skip big responses/files.
*/
pub async fn set_option_max_parse_size_bytes(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "spider", "action", "setOptionMaxParseSizeBytes", params).await
}

/**
 * 
*/
pub async fn set_option_max_scans_in_ui(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "spider", "action", "setOptionMaxScansInUI", params).await
}

/**
 * 
*/
pub async fn set_option_parse_comments(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "spider", "action", "setOptionParseComments", params).await
}

/**
 * 
*/
pub async fn set_option_parse_git(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "spider", "action", "setOptionParseGit", params).await
}

/**
 * 
*/
pub async fn set_option_parse_robots_txt(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "spider", "action", "setOptionParseRobotsTxt", params).await
}

/**
 * 
*/
pub async fn set_option_parse_svn_entries(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "spider", "action", "setOptionParseSVNEntries", params).await
}

/**
 * 
*/
pub async fn set_option_parse_sitemap_xml(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "spider", "action", "setOptionParseSitemapXml", params).await
}

/**
 * 
*/
pub async fn set_option_post_form(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "spider", "action", "setOptionPostForm", params).await
}

/**
 * 
*/
pub async fn set_option_process_form(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "spider", "action", "setOptionProcessForm", params).await
}

/**
 * 
*/
pub async fn set_option_request_wait_time(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "spider", "action", "setOptionRequestWaitTime", params).await
}

/**
 * Sets whether or not the 'Referer' header should be sent while spidering.
*/
pub async fn set_option_send_referer_header(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "spider", "action", "setOptionSendRefererHeader", params).await
}

/**
 * 
*/
pub async fn set_option_show_advanced_dialog(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "spider", "action", "setOptionShowAdvancedDialog", params).await
}

/**
 * 
*/
pub async fn set_option_thread_count(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "spider", "action", "setOptionThreadCount", params).await
}

