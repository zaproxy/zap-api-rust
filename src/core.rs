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
 * Gets the name of the hosts accessed through/by ZAP
*/
pub fn hosts(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "hosts", params)
}

/**
 * Gets the sites accessed through/by ZAP (scheme and domain)
*/
pub fn sites(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "sites", params)
}

/**
 * Gets the URLs accessed through/by ZAP, optionally filtering by (base) URL.
*/
pub fn urls(service: &ZapService, baseurl: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl);
    super::call(service, "core", "view", "urls", params)
}

/**
 * Gets the child nodes underneath the specified URL in the Sites tree
*/
pub fn child_nodes(service: &ZapService, url: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url);
    super::call(service, "core", "view", "childNodes", params)
}

/**
 * Gets the HTTP message with the given ID. Returns the ID, request/response headers and bodies, cookies, note, type, RTT, and timestamp.
*/
pub fn message(service: &ZapService, id: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    super::call(service, "core", "view", "message", params)
}

/**
 * Gets the HTTP messages sent by ZAP, request and response, optionally filtered by URL and paginated with 'start' position and 'count' of messages
*/
pub fn messages(
    service: &ZapService,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "core", "view", "messages", params)
}

/**
 * Gets the HTTP messages with the given IDs.
*/
pub fn messages_by_id(service: &ZapService, ids: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids);
    super::call(service, "core", "view", "messagesById", params)
}

/**
 * Gets the number of messages, optionally filtering by URL
*/
pub fn number_of_messages(service: &ZapService, baseurl: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl);
    super::call(service, "core", "view", "numberOfMessages", params)
}

/**
 * Gets the mode
*/
pub fn mode(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "mode", params)
}

/**
 * Gets ZAP version
*/
pub fn version(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "version", params)
}

/**
 * Gets the regular expressions, applied to URLs, to exclude from the local proxies.
*/
pub fn excluded_from_proxy(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "excludedFromProxy", params)
}

pub fn home_directory(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "homeDirectory", params)
}

/**
 * Gets the location of the current session file
*/
pub fn session_location(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "sessionLocation", params)
}

/**
 * Gets all the domains that are excluded from the outgoing proxy. For each domain the following are shown: the index, the value (domain), if enabled, and if specified as a regex.
*/
pub fn proxy_chain_excluded_domains(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "proxyChainExcludedDomains", params)
}

/**
 * Use view proxyChainExcludedDomains instead.
 * Deprecated
*/
pub fn option_proxy_chain_skip_name(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainSkipName", params)
}

/**
 * Use view proxyChainExcludedDomains instead.
 * Deprecated
*/
pub fn option_proxy_excluded_domains(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "core",
        "view",
        "optionProxyExcludedDomains",
        params,
    )
}

/**
 * Use view proxyChainExcludedDomains instead.
 * Deprecated
*/
pub fn option_proxy_excluded_domains_enabled(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "core",
        "view",
        "optionProxyExcludedDomainsEnabled",
        params,
    )
}

/**
 * Gets the path to ZAP's home directory.
*/
pub fn zap_home_path(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "zapHomePath", params)
}

/**
 * Gets the maximum number of alert instances to include in a report.
*/
pub fn option_maximum_alert_instances(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "core",
        "view",
        "optionMaximumAlertInstances",
        params,
    )
}

/**
 * Gets whether or not related alerts will be merged in any reports generated.
*/
pub fn option_merge_related_alerts(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionMergeRelatedAlerts", params)
}

/**
 * Gets the path to the file with alert overrides.
*/
pub fn option_alert_overrides_file_path(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "core",
        "view",
        "optionAlertOverridesFilePath",
        params,
    )
}

/**
 * Gets the alert with the given ID, the corresponding HTTP message can be obtained with the 'messageId' field and 'message' API method
 * Deprecated Use the API endpoint with the same name in the 'alert' component instead.
*/
pub fn alert(service: &ZapService, id: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    super::call(service, "core", "view", "alert", params)
}

/**
 * Gets the alerts raised by ZAP, optionally filtering by URL or riskId, and paginating with 'start' position and 'count' of alerts
 * Deprecated Use the API endpoint with the same name in the 'alert' component instead.
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
    super::call(service, "core", "view", "alerts", params)
}

/**
 * Gets number of alerts grouped by each risk level, optionally filtering by URL
 * Deprecated Use the API endpoint with the same name in the 'alert' component instead.
*/
pub fn alerts_summary(service: &ZapService, baseurl: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl);
    super::call(service, "core", "view", "alertsSummary", params)
}

/**
 * Gets the number of alerts, optionally filtering by URL or riskId
 * Deprecated Use the API endpoint with the same name in the 'alert' component instead.
*/
pub fn number_of_alerts(
    service: &ZapService,
    baseurl: String,
    riskid: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl);
    params.insert("riskId".to_string(), riskid);
    super::call(service, "core", "view", "numberOfAlerts", params)
}

/**
 * Gets the user agent that ZAP should use when creating HTTP messages (for example, spider messages or CONNECT requests to outgoing proxy).
*/
pub fn option_default_user_agent(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionDefaultUserAgent", params)
}

/**
 * Gets the TTL (in seconds) of successful DNS queries.
*/
pub fn option_dns_ttl_successful_queries(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "core",
        "view",
        "optionDnsTtlSuccessfulQueries",
        params,
    )
}

pub fn option_http_state(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionHttpState", params)
}

pub fn option_proxy_chain_name(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainName", params)
}

pub fn option_proxy_chain_password(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainPassword", params)
}

pub fn option_proxy_chain_port(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainPort", params)
}

pub fn option_proxy_chain_realm(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainRealm", params)
}

pub fn option_proxy_chain_user_name(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainUserName", params)
}

/**
 * Gets the connection time out, in seconds.
*/
pub fn option_timeout_in_secs(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionTimeoutInSecs", params)
}

pub fn option_http_state_enabled(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionHttpStateEnabled", params)
}

pub fn option_proxy_chain_prompt(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionProxyChainPrompt", params)
}

pub fn option_single_cookie_request_header(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "core",
        "view",
        "optionSingleCookieRequestHeader",
        params,
    )
}

pub fn option_use_proxy_chain(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionUseProxyChain", params)
}

pub fn option_use_proxy_chain_auth(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "view", "optionUseProxyChainAuth", params)
}

/**
 * Convenient and simple action to access a URL, optionally following redirections. Returns the request sent and response received and followed redirections, if any. Other actions are available which offer more control on what is sent, like, 'sendRequest' or 'sendHarRequest'.
*/
pub fn access_url(
    service: &ZapService,
    url: String,
    followredirects: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url);
    params.insert("followRedirects".to_string(), followredirects);
    super::call(service, "core", "action", "accessUrl", params)
}

/**
 * Shuts down ZAP
*/
pub fn shutdown(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "shutdown", params)
}

/**
 * Creates a new session, optionally overwriting existing files. If a relative path is specified it will be resolved against the "session" directory in ZAP "home" dir.
*/
pub fn new_session(
    service: &ZapService,
    name: String,
    overwrite: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("name".to_string(), name);
    params.insert("overwrite".to_string(), overwrite);
    super::call(service, "core", "action", "newSession", params)
}

/**
 * Loads the session with the given name. If a relative path is specified it will be resolved against the "session" directory in ZAP "home" dir.
*/
pub fn load_session(service: &ZapService, name: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("name".to_string(), name);
    super::call(service, "core", "action", "loadSession", params)
}

/**
 * Saves the session with the name supplied, optionally overwriting existing files. If a relative path is specified it will be resolved against the "session" directory in ZAP "home" dir.
*/
pub fn save_session(
    service: &ZapService,
    name: String,
    overwrite: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("name".to_string(), name);
    params.insert("overwrite".to_string(), overwrite);
    super::call(service, "core", "action", "saveSession", params)
}

/**
 * Snapshots the session, optionally with the given name, and overwriting existing files. If no name is specified the name of the current session with a timestamp appended is used. If a relative path is specified it will be resolved against the "session" directory in ZAP "home" dir.
*/
pub fn snapshot_session(
    service: &ZapService,
    name: String,
    overwrite: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("name".to_string(), name);
    params.insert("overwrite".to_string(), overwrite);
    super::call(service, "core", "action", "snapshotSession", params)
}

/**
 * Clears the regexes of URLs excluded from the local proxies.
*/
pub fn clear_excluded_from_proxy(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "clearExcludedFromProxy", params)
}

/**
 * Adds a regex of URLs that should be excluded from the local proxies.
*/
pub fn exclude_from_proxy(service: &ZapService, regex: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("regex".to_string(), regex);
    super::call(service, "core", "action", "excludeFromProxy", params)
}

pub fn set_home_directory(service: &ZapService, dir: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("dir".to_string(), dir);
    super::call(service, "core", "action", "setHomeDirectory", params)
}

/**
 * Sets the mode, which may be one of [safe, protect, standard, attack]
*/
pub fn set_mode(service: &ZapService, mode: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("mode".to_string(), mode);
    super::call(service, "core", "action", "setMode", params)
}

/**
 * Generates a new Root CA certificate for the local proxies.
*/
pub fn generate_root_ca(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "generateRootCA", params)
}

/**
 * Sends the HTTP request, optionally following redirections. Returns the request sent and response received and followed redirections, if any. The Mode is enforced when sending the request (and following redirections), custom manual requests are not allowed in 'Safe' mode nor in 'Protected' mode if out of scope.
*/
pub fn send_request(
    service: &ZapService,
    request: String,
    followredirects: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("request".to_string(), request);
    params.insert("followRedirects".to_string(), followredirects);
    super::call(service, "core", "action", "sendRequest", params)
}

pub fn run_garbage_collection(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "runGarbageCollection", params)
}

/**
 * Deletes the site node found in the Sites Tree on the basis of the URL, HTTP method, and post data (if applicable and specified).
*/
pub fn delete_site_node(
    service: &ZapService,
    url: String,
    method: String,
    postdata: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url);
    params.insert("method".to_string(), method);
    params.insert("postData".to_string(), postdata);
    super::call(service, "core", "action", "deleteSiteNode", params)
}

/**
 * Adds a domain to be excluded from the outgoing proxy, using the specified value. Optionally sets if the new entry is enabled (default, true) and whether or not the new value is specified as a regex (default, false).
*/
pub fn add_proxy_chain_excluded_domain(
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
        "core",
        "action",
        "addProxyChainExcludedDomain",
        params,
    )
}

/**
 * Modifies a domain excluded from the outgoing proxy. Allows to modify the value, if enabled or if a regex. The domain is selected with its index, which can be obtained with the view proxyChainExcludedDomains.
*/
pub fn modify_proxy_chain_excluded_domain(
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
        "core",
        "action",
        "modifyProxyChainExcludedDomain",
        params,
    )
}

/**
 * Removes a domain excluded from the outgoing proxy, with the given index. The index can be obtained with the view proxyChainExcludedDomains.
*/
pub fn remove_proxy_chain_excluded_domain(
    service: &ZapService,
    idx: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("idx".to_string(), idx);
    super::call(
        service,
        "core",
        "action",
        "removeProxyChainExcludedDomain",
        params,
    )
}

/**
 * Enables all domains excluded from the outgoing proxy.
*/
pub fn enable_all_proxy_chain_excluded_domains(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "core",
        "action",
        "enableAllProxyChainExcludedDomains",
        params,
    )
}

/**
 * Disables all domains excluded from the outgoing proxy.
*/
pub fn disable_all_proxy_chain_excluded_domains(
    service: &ZapService,
) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "core",
        "action",
        "disableAllProxyChainExcludedDomains",
        params,
    )
}

/**
 * Sets the maximum number of alert instances to include in a report. A value of zero is treated as unlimited.
*/
pub fn set_option_maximum_alert_instances(
    service: &ZapService,
    numberofinstances: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("numberOfInstances".to_string(), numberofinstances);
    super::call(
        service,
        "core",
        "action",
        "setOptionMaximumAlertInstances",
        params,
    )
}

/**
 * Sets whether or not related alerts will be merged in any reports generated.
*/
pub fn set_option_merge_related_alerts(
    service: &ZapService,
    enabled: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("enabled".to_string(), enabled);
    super::call(
        service,
        "core",
        "action",
        "setOptionMergeRelatedAlerts",
        params,
    )
}

/**
 * Sets (or clears, if empty) the path to the file with alert overrides.
*/
pub fn set_option_alert_overrides_file_path(
    service: &ZapService,
    filepath: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("filePath".to_string(), filepath);
    super::call(
        service,
        "core",
        "action",
        "setOptionAlertOverridesFilePath",
        params,
    )
}

/**
 * Enables use of a PKCS12 client certificate for the certificate with the given file system path, password, and optional index.
*/
pub fn enable_pkcs_12_client_certificate(
    service: &ZapService,
    filepath: String,
    password: String,
    index: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("filePath".to_string(), filepath);
    params.insert("password".to_string(), password);
    params.insert("index".to_string(), index);
    super::call(
        service,
        "core",
        "action",
        "enablePKCS12ClientCertificate",
        params,
    )
}

/**
 * Disables the option for use of client certificates.
*/
pub fn disable_client_certificate(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "core",
        "action",
        "disableClientCertificate",
        params,
    )
}

/**
 * Deletes all alerts of the current session.
 * Deprecated Use the API endpoint with the same name in the 'alert' component instead.
*/
pub fn delete_all_alerts(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "action", "deleteAllAlerts", params)
}

/**
 * Deletes the alert with the given ID.
 * Deprecated Use the API endpoint with the same name in the 'alert' component instead.
*/
pub fn delete_alert(service: &ZapService, id: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    super::call(service, "core", "action", "deleteAlert", params)
}

/**
 * Sets the user agent that ZAP should use when creating HTTP messages (for example, spider messages or CONNECT requests to outgoing proxy).
*/
pub fn set_option_default_user_agent(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "core",
        "action",
        "setOptionDefaultUserAgent",
        params,
    )
}

pub fn set_option_proxy_chain_name(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(service, "core", "action", "setOptionProxyChainName", params)
}

pub fn set_option_proxy_chain_password(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "core",
        "action",
        "setOptionProxyChainPassword",
        params,
    )
}

pub fn set_option_proxy_chain_realm(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "core",
        "action",
        "setOptionProxyChainRealm",
        params,
    )
}

/**
 * Use actions [add|modify|remove]ProxyChainExcludedDomain instead.
 * Deprecated Option no longer in effective use.
*/
pub fn set_option_proxy_chain_skip_name(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "core",
        "action",
        "setOptionProxyChainSkipName",
        params,
    )
}

pub fn set_option_proxy_chain_user_name(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "core",
        "action",
        "setOptionProxyChainUserName",
        params,
    )
}

/**
 * Sets the TTL (in seconds) of successful DNS queries (applies after ZAP restart).
*/
pub fn set_option_dns_ttl_successful_queries(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "core",
        "action",
        "setOptionDnsTtlSuccessfulQueries",
        params,
    )
}

pub fn set_option_http_state_enabled(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "core",
        "action",
        "setOptionHttpStateEnabled",
        params,
    )
}

pub fn set_option_proxy_chain_port(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(service, "core", "action", "setOptionProxyChainPort", params)
}

pub fn set_option_proxy_chain_prompt(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "core",
        "action",
        "setOptionProxyChainPrompt",
        params,
    )
}

pub fn set_option_single_cookie_request_header(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "core",
        "action",
        "setOptionSingleCookieRequestHeader",
        params,
    )
}

/**
 * Sets the connection time out, in seconds.
*/
pub fn set_option_timeout_in_secs(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(service, "core", "action", "setOptionTimeoutInSecs", params)
}

/**
 * Sets whether or not the outgoing proxy should be used. The address/hostname of the outgoing proxy must be set to enable this option.
*/
pub fn set_option_use_proxy_chain(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(service, "core", "action", "setOptionUseProxyChain", params)
}

pub fn set_option_use_proxy_chain_auth(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "core",
        "action",
        "setOptionUseProxyChainAuth",
        params,
    )
}

pub fn proxy_pac(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "other", "proxy.pac", params)
}

/**
 * Gets the Root CA certificate used by the local proxies.
*/
pub fn rootcert(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "other", "rootcert", params)
}

pub fn setproxy(service: &ZapService, proxy: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("proxy".to_string(), proxy);
    super::call(service, "core", "other", "setproxy", params)
}

/**
 * Generates a report in XML format
*/
pub fn xmlreport(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "other", "xmlreport", params)
}

/**
 * Generates a report in HTML format
*/
pub fn htmlreport(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "other", "htmlreport", params)
}

/**
 * Generates a report in JSON format
*/
pub fn jsonreport(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "other", "jsonreport", params)
}

/**
 * Generates a report in Markdown format
*/
pub fn mdreport(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "core", "other", "mdreport", params)
}

/**
 * Gets the message with the given ID in HAR format
*/
pub fn message_har(service: &ZapService, id: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    super::call(service, "core", "other", "messageHar", params)
}

/**
 * Gets the HTTP messages sent through/by ZAP, in HAR format, optionally filtered by URL and paginated with 'start' position and 'count' of messages
*/
pub fn messages_har(
    service: &ZapService,
    baseurl: String,
    start: String,
    count: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("baseurl".to_string(), baseurl);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "core", "other", "messagesHar", params)
}

/**
 * Gets the HTTP messages with the given IDs, in HAR format.
*/
pub fn messages_har_by_id(service: &ZapService, ids: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("ids".to_string(), ids);
    super::call(service, "core", "other", "messagesHarById", params)
}

/**
 * Sends the first HAR request entry, optionally following redirections. Returns, in HAR format, the request sent and response received and followed redirections, if any. The Mode is enforced when sending the request (and following redirections), custom manual requests are not allowed in 'Safe' mode nor in 'Protected' mode if out of scope.
*/
pub fn send_har_request(
    service: &ZapService,
    request: String,
    followredirects: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("request".to_string(), request);
    params.insert("followRedirects".to_string(), followredirects);
    super::call(service, "core", "other", "sendHarRequest", params)
}
