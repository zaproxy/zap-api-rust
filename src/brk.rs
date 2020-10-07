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
 * Returns True if ZAP will break on both requests and responses
*/
pub async fn is_break_all(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "view", "isBreakAll", params).await
}

/**
 * Returns True if ZAP will break on requests
*/
pub async fn is_break_request(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "view", "isBreakRequest", params).await
}

/**
 * Returns True if ZAP will break on responses
*/
pub async fn is_break_response(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "view", "isBreakResponse", params).await
}

/**
 * Returns the HTTP message currently intercepted (if any)
*/
pub async fn http_message(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "view", "httpMessage", params).await
}

/**
 * Controls the global break functionality. The type may be one of: http-all, http-request or http-response. The state may be true (for turning break on for the specified type) or false (for turning break off). Scope is not currently used.
*/
pub async fn brk(service: &ZapService, typ: &str, state: &str, scope: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("type".to_string(), typ.to_string());
    params.insert("state".to_string(), state.to_string());
    params.insert("scope".to_string(), scope.to_string());
    super::call(service, "break", "action", "break", params).await
}

/**
 * Overwrites the currently intercepted message with the data provided
*/
pub async fn set_http_message(service: &ZapService, httpheader: &str, httpbody: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("httpHeader".to_string(), httpheader.to_string());
    params.insert("httpBody".to_string(), httpbody.to_string());
    super::call(service, "break", "action", "setHttpMessage", params).await
}

/**
 * Submits the currently intercepted message and unsets the global request/response breakpoints
*/
pub async fn cont(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "action", "continue", params).await
}

/**
 * Submits the currently intercepted message, the next request or response will automatically be intercepted
*/
pub async fn step(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "action", "step", params).await
}

/**
 * Drops the currently intercepted message
*/
pub async fn drop(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "action", "drop", params).await
}

/**
 * Adds a custom HTTP breakpoint. The string is the string to match. Location may be one of: url, request_header, request_body, response_header or response_body. Match may be: contains or regex. Inverse (match) may be true or false. Lastly, ignorecase (when matching the string) may be true or false.  
*/
pub async fn add_http_breakpoint(service: &ZapService, string: &str, location: &str, mtch: &str, inverse: &str, ignorecase: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("string".to_string(), string.to_string());
    params.insert("location".to_string(), location.to_string());
    params.insert("match".to_string(), mtch.to_string());
    params.insert("inverse".to_string(), inverse.to_string());
    params.insert("ignorecase".to_string(), ignorecase.to_string());
    super::call(service, "break", "action", "addHttpBreakpoint", params).await
}

/**
 * Removes the specified breakpoint
*/
pub async fn remove_http_breakpoint(service: &ZapService, string: &str, location: &str, mtch: &str, inverse: &str, ignorecase: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("string".to_string(), string.to_string());
    params.insert("location".to_string(), location.to_string());
    params.insert("match".to_string(), mtch.to_string());
    params.insert("inverse".to_string(), inverse.to_string());
    params.insert("ignorecase".to_string(), ignorecase.to_string());
    super::call(service, "break", "action", "removeHttpBreakpoint", params).await
}

