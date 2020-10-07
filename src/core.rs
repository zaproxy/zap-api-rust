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
 * Gets the name of the hosts accessed through/by ZAP
*/
pub async fn hosts(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "hosts", params).await
}

/**
 * Gets the sites accessed through/by ZAP (scheme and domain)
*/
pub async fn sites(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "sites", params).await
}

/**
 * Gets the URLs accessed through/by ZAP, optionally filtering by (base) URL.
*/
pub async fn urls(service: &ZapService, baseurl: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl.to_string());
    super::call(service, "core", "view", "urls", params).await
}

/**
 * Gets the child nodes underneath the specified URL in the Sites tree
*/
pub async fn child_nodes(service: &ZapService, url: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url.to_string());
    super::call(service, "core", "view", "childNodes", params).await
}

/**
 * Gets the HTTP message with the given ID. Returns the ID, request/response headers and bodies, cookies, note, type, RTT, and timestamp.
*/
pub async fn message(service: &ZapService, id: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    super::call(service, "core", "view", "message", params).await
}

/**
 * Gets the HTTP messages sent by ZAP, request and response, optionally filtered by URL and paginated with 'start' position and 'count' of messages
*/
pub async fn messages(service: &ZapService, baseurl: &str, start: &str, count: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl.to_string());
    params.insert("start".to_string(), start.to_string());
    params.insert("count".to_string(), count.to_string());
    super::call(service, "core", "view", "messages", params).await
}

/**
 * Gets the HTTP messages with the given IDs.
*/
pub async fn messages_by_id(service: &ZapService, ids: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids.to_string());
    super::call(service, "core", "view", "messagesById", params).await
}

/**
 * Gets the number of messages, optionally filtering by URL
*/
pub async fn number_of_messages(service: &ZapService, baseurl: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl.to_string());
    super::call(service, "core", "view", "numberOfMessages", params).await
}

/**
 * Gets the mode
*/
pub async fn mode(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "mode", params).await
}

/**
 * Gets ZAP version
*/
pub async fn version(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "version", params).await
}

/**
 * Gets the regular expressions, applied to URLs, to exclude from the local proxies.
*/
pub async fn excluded_from_proxy(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "excludedFromProxy", params).await
}

/**
 * 
*/
pub async fn home_directory(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "homeDirectory", params).await
}

/**
 * Gets the location of the current session file
*/
pub async fn session_location(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "sessionLocation", params).await
}

/**
 * Gets all the domains that are excluded from the outgoing proxy. For each domain the following are shown: the index, the value (domain), if enabled, and if specified as a regex.
*/
pub async fn proxy_chain_excluded_domains(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "proxyChainExcludedDomains", params).await
}

/**
 * Use view proxyChainExcludedDomains instead.
*/
#[deprecated]
pub async fn option_proxy_chain_skip_name(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainSkipName", params).await
}

/**
 * Use view proxyChainExcludedDomains instead.
*/
#[deprecated]
pub async fn option_proxy_excluded_domains(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyExcludedDomains", params).await
}

/**
 * Use view proxyChainExcludedDomains instead.
*/
#[deprecated]
pub async fn option_proxy_excluded_domains_enabled(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyExcludedDomainsEnabled", params).await
}

/**
 * Gets the path to ZAP's home directory.
*/
pub async fn zap_home_path(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "zapHomePath", params).await
}

/**
 * Gets the maximum number of alert instances to include in a report.
*/
pub async fn option_maximum_alert_instances(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionMaximumAlertInstances", params).await
}

/**
 * Gets whether or not related alerts will be merged in any reports generated.
*/
pub async fn option_merge_related_alerts(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionMergeRelatedAlerts", params).await
}

/**
 * Gets the path to the file with alert overrides.
*/
pub async fn option_alert_overrides_file_path(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionAlertOverridesFilePath", params).await
}

/**
 * Gets the alert with the given ID, the corresponding HTTP message can be obtained with the 'messageId' field and 'message' API method
*/
#[deprecated(note="Use the API endpoint with the same name in the 'alert' component instead.")]
pub async fn alert(service: &ZapService, id: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    super::call(service, "core", "view", "alert", params).await
}

/**
 * Gets the alerts raised by ZAP, optionally filtering by URL or riskId, and paginating with 'start' position and 'count' of alerts
*/
#[deprecated(note="Use the API endpoint with the same name in the 'alert' component instead.")]
pub async fn alerts(service: &ZapService, baseurl: &str, start: &str, count: &str, riskid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl.to_string());
    params.insert("start".to_string(), start.to_string());
    params.insert("count".to_string(), count.to_string());
    params.insert("riskId".to_string(), riskid.to_string());
    super::call(service, "core", "view", "alerts", params).await
}

/**
 * Gets number of alerts grouped by each risk level, optionally filtering by URL
*/
#[deprecated(note="Use the API endpoint with the same name in the 'alert' component instead.")]
pub async fn alerts_summary(service: &ZapService, baseurl: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl.to_string());
    super::call(service, "core", "view", "alertsSummary", params).await
}

/**
 * Gets the number of alerts, optionally filtering by URL or riskId
*/
#[deprecated(note="Use the API endpoint with the same name in the 'alert' component instead.")]
pub async fn number_of_alerts(service: &ZapService, baseurl: &str, riskid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl.to_string());
    params.insert("riskId".to_string(), riskid.to_string());
    super::call(service, "core", "view", "numberOfAlerts", params).await
}

/**
 * Gets the user agent that ZAP should use when creating HTTP messages (for example, spider messages or CONNECT requests to outgoing proxy).
*/
pub async fn option_default_user_agent(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionDefaultUserAgent", params).await
}

/**
 * Gets the TTL (in seconds) of successful DNS queries.
*/
pub async fn option_dns_ttl_successful_queries(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionDnsTtlSuccessfulQueries", params).await
}

/**
 * 
*/
pub async fn option_http_state(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionHttpState", params).await
}

/**
 * 
*/
pub async fn option_proxy_chain_name(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainName", params).await
}

/**
 * 
*/
pub async fn option_proxy_chain_password(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainPassword", params).await
}

/**
 * 
*/
pub async fn option_proxy_chain_port(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainPort", params).await
}

/**
 * 
*/
pub async fn option_proxy_chain_realm(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainRealm", params).await
}

/**
 * 
*/
pub async fn option_proxy_chain_user_name(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainUserName", params).await
}

/**
 * Gets the connection time out, in seconds.
*/
pub async fn option_timeout_in_secs(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionTimeoutInSecs", params).await
}

/**
 * 
*/
pub async fn option_http_state_enabled(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionHttpStateEnabled", params).await
}

/**
 * 
*/
pub async fn option_proxy_chain_prompt(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainPrompt", params).await
}

/**
 * 
*/
pub async fn option_single_cookie_request_header(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionSingleCookieRequestHeader", params).await
}

/**
 * 
*/
pub async fn option_use_proxy_chain(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionUseProxyChain", params).await
}

/**
 * 
*/
pub async fn option_use_proxy_chain_auth(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionUseProxyChainAuth", params).await
}

/**
 * Gets whether or not the SOCKS proxy should be used.
*/
pub async fn option_use_socks_proxy(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionUseSocksProxy", params).await
}

/**
 * Convenient and simple action to access a URL, optionally following redirections. Returns the request sent and response received and followed redirections, if any. Other actions are available which offer more control on what is sent, like, 'sendRequest' or 'sendHarRequest'.
*/
pub async fn access_url(service: &ZapService, url: &str, followredirects: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url.to_string());
    params.insert("followRedirects".to_string(), followredirects.to_string());
    super::call(service, "core", "action", "accessUrl", params).await
}

/**
 * Shuts down ZAP
*/
pub async fn shutdown(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "shutdown", params).await
}

/**
 * Creates a new session, optionally overwriting existing files. If a relative path is specified it will be resolved against the "session" directory in ZAP "home" dir.
*/
pub async fn new_session(service: &ZapService, name: &str, overwrite: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("name".to_string(), name.to_string());
    params.insert("overwrite".to_string(), overwrite.to_string());
    super::call(service, "core", "action", "newSession", params).await
}

/**
 * Loads the session with the given name. If a relative path is specified it will be resolved against the "session" directory in ZAP "home" dir.
*/
pub async fn load_session(service: &ZapService, name: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("name".to_string(), name.to_string());
    super::call(service, "core", "action", "loadSession", params).await
}

/**
 * Saves the session.
*/
pub async fn save_session(service: &ZapService, name: &str, overwrite: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("name".to_string(), name.to_string());
    params.insert("overwrite".to_string(), overwrite.to_string());
    super::call(service, "core", "action", "saveSession", params).await
}

/**
 * Snapshots the session, optionally with the given name, and overwriting existing files. If no name is specified the name of the current session with a timestamp appended is used. If a relative path is specified it will be resolved against the "session" directory in ZAP "home" dir.
*/
pub async fn snapshot_session(service: &ZapService, name: &str, overwrite: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("name".to_string(), name.to_string());
    params.insert("overwrite".to_string(), overwrite.to_string());
    super::call(service, "core", "action", "snapshotSession", params).await
}

/**
 * Clears the regexes of URLs excluded from the local proxies.
*/
pub async fn clear_excluded_from_proxy(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "clearExcludedFromProxy", params).await
}

/**
 * Adds a regex of URLs that should be excluded from the local proxies.
*/
pub async fn exclude_from_proxy(service: &ZapService, regex: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex.to_string());
    super::call(service, "core", "action", "excludeFromProxy", params).await
}

/**
 * 
*/
pub async fn set_home_directory(service: &ZapService, dir: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("dir".to_string(), dir.to_string());
    super::call(service, "core", "action", "setHomeDirectory", params).await
}

/**
 * Sets the mode, which may be one of [safe, protect, standard, attack]
*/
pub async fn set_mode(service: &ZapService, mode: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("mode".to_string(), mode.to_string());
    super::call(service, "core", "action", "setMode", params).await
}

/**
 * Generates a new Root CA certificate for the local proxies.
*/
pub async fn generate_root_ca(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "generateRootCA", params).await
}

/**
 * Sends the HTTP request, optionally following redirections. Returns the request sent and response received and followed redirections, if any. The Mode is enforced when sending the request (and following redirections), custom manual requests are not allowed in 'Safe' mode nor in 'Protected' mode if out of scope.
*/
pub async fn send_request(service: &ZapService, request: &str, followredirects: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("request".to_string(), request.to_string());
    params.insert("followRedirects".to_string(), followredirects.to_string());
    super::call(service, "core", "action", "sendRequest", params).await
}

/**
 * 
*/
pub async fn run_garbage_collection(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "runGarbageCollection", params).await
}

/**
 * Deletes the site node found in the Sites Tree on the basis of the URL, HTTP method, and post data (if applicable and specified). 
*/
pub async fn delete_site_node(service: &ZapService, url: &str, method: &str, postdata: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url.to_string());
    params.insert("method".to_string(), method.to_string());
    params.insert("postData".to_string(), postdata.to_string());
    super::call(service, "core", "action", "deleteSiteNode", params).await
}

/**
 * Adds a domain to be excluded from the outgoing proxy, using the specified value. Optionally sets if the new entry is enabled (default, true) and whether or not the new value is specified as a regex (default, false).
*/
pub async fn add_proxy_chain_excluded_domain(service: &ZapService, value: &str, isregex: &str, isenabled: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("value".to_string(), value.to_string());
    params.insert("isRegex".to_string(), isregex.to_string());
    params.insert("isEnabled".to_string(), isenabled.to_string());
    super::call(service, "core", "action", "addProxyChainExcludedDomain", params).await
}

/**
 * Modifies a domain excluded from the outgoing proxy. Allows to modify the value, if enabled or if a regex. The domain is selected with its index, which can be obtained with the view proxyChainExcludedDomains.
*/
pub async fn modify_proxy_chain_excluded_domain(service: &ZapService, idx: &str, value: &str, isregex: &str, isenabled: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("idx".to_string(), idx.to_string());
    params.insert("value".to_string(), value.to_string());
    params.insert("isRegex".to_string(), isregex.to_string());
    params.insert("isEnabled".to_string(), isenabled.to_string());
    super::call(service, "core", "action", "modifyProxyChainExcludedDomain", params).await
}

/**
 * Removes a domain excluded from the outgoing proxy, with the given index. The index can be obtained with the view proxyChainExcludedDomains.
*/
pub async fn remove_proxy_chain_excluded_domain(service: &ZapService, idx: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("idx".to_string(), idx.to_string());
    super::call(service, "core", "action", "removeProxyChainExcludedDomain", params).await
}

/**
 * Enables all domains excluded from the outgoing proxy.
*/
pub async fn enable_all_proxy_chain_excluded_domains(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "enableAllProxyChainExcludedDomains", params).await
}

/**
 * Disables all domains excluded from the outgoing proxy.
*/
pub async fn disable_all_proxy_chain_excluded_domains(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "disableAllProxyChainExcludedDomains", params).await
}

/**
 * Sets the maximum number of alert instances to include in a report. A value of zero is treated as unlimited.
*/
pub async fn set_option_maximum_alert_instances(service: &ZapService, numberofinstances: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("numberOfInstances".to_string(), numberofinstances.to_string());
    super::call(service, "core", "action", "setOptionMaximumAlertInstances", params).await
}

/**
 * Sets whether or not related alerts will be merged in any reports generated.
*/
pub async fn set_option_merge_related_alerts(service: &ZapService, enabled: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("enabled".to_string(), enabled.to_string());
    super::call(service, "core", "action", "setOptionMergeRelatedAlerts", params).await
}

/**
 * Sets (or clears, if empty) the path to the file with alert overrides.
*/
pub async fn set_option_alert_overrides_file_path(service: &ZapService, filepath: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("filePath".to_string(), filepath.to_string());
    super::call(service, "core", "action", "setOptionAlertOverridesFilePath", params).await
}

/**
 * Enables use of a PKCS12 client certificate for the certificate with the given file system path, password, and optional index.
*/
pub async fn enable_pkcs_12_client_certificate(service: &ZapService, filepath: &str, password: &str, index: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("filePath".to_string(), filepath.to_string());
    params.insert("password".to_string(), password.to_string());
    params.insert("index".to_string(), index.to_string());
    super::call(service, "core", "action", "enablePKCS12ClientCertificate", params).await
}

/**
 * Disables the option for use of client certificates.
*/
pub async fn disable_client_certificate(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "disableClientCertificate", params).await
}

/**
 * Deletes all alerts of the current session.
*/
#[deprecated(note="Use the API endpoint with the same name in the 'alert' component instead.")]
pub async fn delete_all_alerts(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "deleteAllAlerts", params).await
}

/**
 * Deletes the alert with the given ID. 
*/
#[deprecated(note="Use the API endpoint with the same name in the 'alert' component instead.")]
pub async fn delete_alert(service: &ZapService, id: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    super::call(service, "core", "action", "deleteAlert", params).await
}

/**
 * Sets the user agent that ZAP should use when creating HTTP messages (for example, spider messages or CONNECT requests to outgoing proxy).
*/
pub async fn set_option_default_user_agent(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "core", "action", "setOptionDefaultUserAgent", params).await
}

/**
 * 
*/
pub async fn set_option_proxy_chain_name(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "core", "action", "setOptionProxyChainName", params).await
}

/**
 * 
*/
pub async fn set_option_proxy_chain_password(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "core", "action", "setOptionProxyChainPassword", params).await
}

/**
 * 
*/
pub async fn set_option_proxy_chain_realm(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "core", "action", "setOptionProxyChainRealm", params).await
}

/**
 * Use actions [add|modify|remove]ProxyChainExcludedDomain instead.
*/
#[deprecated(note="Option no longer in effective use.")]
pub async fn set_option_proxy_chain_skip_name(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "core", "action", "setOptionProxyChainSkipName", params).await
}

/**
 * 
*/
pub async fn set_option_proxy_chain_user_name(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "core", "action", "setOptionProxyChainUserName", params).await
}

/**
 * Sets the TTL (in seconds) of successful DNS queries (applies after ZAP restart).
*/
pub async fn set_option_dns_ttl_successful_queries(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "core", "action", "setOptionDnsTtlSuccessfulQueries", params).await
}

/**
 * 
*/
pub async fn set_option_http_state_enabled(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "core", "action", "setOptionHttpStateEnabled", params).await
}

/**
 * 
*/
pub async fn set_option_proxy_chain_port(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "core", "action", "setOptionProxyChainPort", params).await
}

/**
 * 
*/
pub async fn set_option_proxy_chain_prompt(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "core", "action", "setOptionProxyChainPrompt", params).await
}

/**
 * 
*/
pub async fn set_option_single_cookie_request_header(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "core", "action", "setOptionSingleCookieRequestHeader", params).await
}

/**
 * Sets the connection time out, in seconds.
*/
pub async fn set_option_timeout_in_secs(service: &ZapService, integer: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer.to_string());
    super::call(service, "core", "action", "setOptionTimeoutInSecs", params).await
}

/**
 * Sets whether or not the outgoing proxy should be used. The address/hostname of the outgoing proxy must be set to enable this option.
*/
pub async fn set_option_use_proxy_chain(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "core", "action", "setOptionUseProxyChain", params).await
}

/**
 * 
*/
pub async fn set_option_use_proxy_chain_auth(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "core", "action", "setOptionUseProxyChainAuth", params).await
}

/**
 * Sets whether or not the SOCKS proxy should be used.
*/
pub async fn set_option_use_socks_proxy(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "core", "action", "setOptionUseSocksProxy", params).await
}

/**
 * 
*/
pub async fn proxy_pac(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "other", "proxy.pac", params).await
}

/**
 * Gets the Root CA certificate used by the local proxies.
*/
pub async fn rootcert(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "other", "rootcert", params).await
}

/**
 * 
*/
pub async fn setproxy(service: &ZapService, proxy: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("proxy".to_string(), proxy.to_string());
    super::call(service, "core", "other", "setproxy", params).await
}

/**
 * Generates a report in XML format
*/
pub async fn xmlreport(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "other", "xmlreport", params).await
}

/**
 * Generates a report in HTML format
*/
pub async fn htmlreport(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "other", "htmlreport", params).await
}

/**
 * Generates a report in JSON format
*/
pub async fn jsonreport(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "other", "jsonreport", params).await
}

/**
 * Generates a report in Markdown format
*/
pub async fn mdreport(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "other", "mdreport", params).await
}

/**
 * Gets the message with the given ID in HAR format
*/
pub async fn message_har(service: &ZapService, id: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    super::call(service, "core", "other", "messageHar", params).await
}

/**
 * Gets the HTTP messages sent through/by ZAP, in HAR format, optionally filtered by URL and paginated with 'start' position and 'count' of messages
*/
pub async fn messages_har(service: &ZapService, baseurl: &str, start: &str, count: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl.to_string());
    params.insert("start".to_string(), start.to_string());
    params.insert("count".to_string(), count.to_string());
    super::call(service, "core", "other", "messagesHar", params).await
}

/**
 * Gets the HTTP messages with the given IDs, in HAR format.
*/
pub async fn messages_har_by_id(service: &ZapService, ids: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids.to_string());
    super::call(service, "core", "other", "messagesHarById", params).await
}

/**
 * Sends the first HAR request entry, optionally following redirections. Returns, in HAR format, the request sent and response received and followed redirections, if any. The Mode is enforced when sending the request (and following redirections), custom manual requests are not allowed in 'Safe' mode nor in 'Protected' mode if out of scope.
*/
pub async fn send_har_request(service: &ZapService, request: &str, followredirects: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("request".to_string(), request.to_string());
    params.insert("followRedirects".to_string(), followredirects.to_string());
    super::call(service, "core", "other", "sendHarRequest", params).await
}

