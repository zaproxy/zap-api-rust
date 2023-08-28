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
 * Gets the name of the authentication methods.
*/
pub async fn get_supported_authentication_methods(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "authentication", "view", "getSupportedAuthenticationMethods", params).await
}

/**
 * Gets the configuration parameters for the authentication method with the given name.
*/
pub async fn get_authentication_method_config_params(service: &ZapService, authmethodname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("authMethodName".to_string(), authmethodname.to_string());
    super::call(service, "authentication", "view", "getAuthenticationMethodConfigParams", params).await
}

/**
 * Gets the name of the authentication method for the context with the given ID.
*/
pub async fn get_authentication_method(service: &ZapService, contextid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    super::call(service, "authentication", "view", "getAuthenticationMethod", params).await
}

/**
 * Gets the logged in indicator for the context with the given ID.
*/
pub async fn get_logged_in_indicator(service: &ZapService, contextid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    super::call(service, "authentication", "view", "getLoggedInIndicator", params).await
}

/**
 * Gets the logged out indicator for the context with the given ID.
*/
pub async fn get_logged_out_indicator(service: &ZapService, contextid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    super::call(service, "authentication", "view", "getLoggedOutIndicator", params).await
}

/**
 * Sets the authentication method for the context with the given ID.
*/
pub async fn set_authentication_method(service: &ZapService, contextid: &str, authmethodname: &str, authmethodconfigparams: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("authMethodName".to_string(), authmethodname.to_string());
    params.insert("authMethodConfigParams".to_string(), authmethodconfigparams.to_string());
    super::call(service, "authentication", "action", "setAuthenticationMethod", params).await
}

/**
 * Sets the logged in indicator for the context with the given ID.
*/
pub async fn set_logged_in_indicator(service: &ZapService, contextid: &str, loggedinindicatorregex: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("loggedInIndicatorRegex".to_string(), loggedinindicatorregex.to_string());
    super::call(service, "authentication", "action", "setLoggedInIndicator", params).await
}

/**
 * Sets the logged out indicator for the context with the given ID.
*/
pub async fn set_logged_out_indicator(service: &ZapService, contextid: &str, loggedoutindicatorregex: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("loggedOutIndicatorRegex".to_string(), loggedoutindicatorregex.to_string());
    super::call(service, "authentication", "action", "setLoggedOutIndicator", params).await
}

