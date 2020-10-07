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
 * Gets the name of the session management methods.
*/
pub async fn get_supported_session_management_methods(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "sessionManagement", "view", "getSupportedSessionManagementMethods", params).await
}

/**
 * Gets the configuration parameters for the session management method with the given name.
*/
pub async fn get_session_management_method_config_params(service: &ZapService, methodname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("methodName".to_string(), methodname.to_string());
    super::call(service, "sessionManagement", "view", "getSessionManagementMethodConfigParams", params).await
}

/**
 * Gets the name of the session management method for the context with the given ID.
*/
pub async fn get_session_management_method(service: &ZapService, contextid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    super::call(service, "sessionManagement", "view", "getSessionManagementMethod", params).await
}

/**
 * Sets the session management method for the context with the given ID.
*/
pub async fn set_session_management_method(service: &ZapService, contextid: &str, methodname: &str, methodconfigparams: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("methodName".to_string(), methodname.to_string());
    params.insert("methodConfigParams".to_string(), methodconfigparams.to_string());
    super::call(service, "sessionManagement", "action", "setSessionManagementMethod", params).await
}

