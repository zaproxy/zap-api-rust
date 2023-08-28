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
 * Gets all of the additional proxies that have been configured.
*/
pub async fn additional_proxies(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "localProxies", "view", "additionalProxies", params).await
}

/**
 * Adds an new proxy using the details supplied.
*/
pub async fn add_additional_proxy(service: &ZapService, address: &str, port: &str, behindnat: &str, alwaysdecodezip: &str, removeunsupportedencodings: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("address".to_string(), address.to_string());
    params.insert("port".to_string(), port.to_string());
    params.insert("behindNat".to_string(), behindnat.to_string());
    params.insert("alwaysDecodeZip".to_string(), alwaysdecodezip.to_string());
    params.insert("removeUnsupportedEncodings".to_string(), removeunsupportedencodings.to_string());
    super::call(service, "localProxies", "action", "addAdditionalProxy", params).await
}

/**
 * Removes the additional proxy with the specified address and port.
*/
pub async fn remove_additional_proxy(service: &ZapService, address: &str, port: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("address".to_string(), address.to_string());
    params.insert("port".to_string(), port.to_string());
    super::call(service, "localProxies", "action", "removeAdditionalProxy", params).await
}

