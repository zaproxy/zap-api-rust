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
 * Lists the names of all anti-CSRF tokens
*/
pub async fn option_tokens_names(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "acsrf", "view", "optionTokensNames", params).await
}

/**
 * Adds an anti-CSRF token with the given name, enabled by default
*/
pub async fn add_option_token(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "acsrf", "action", "addOptionToken", params).await
}

/**
 * Removes the anti-CSRF token with the given name
*/
pub async fn remove_option_token(service: &ZapService, string: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string.to_string());
    super::call(service, "acsrf", "action", "removeOptionToken", params).await
}

/**
 * Generate a form for testing lack of anti-CSRF tokens - typically invoked via ZAP
*/
pub async fn gen_form(service: &ZapService, hrefid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("hrefId".to_string(), hrefid.to_string());
    super::call(service, "acsrf", "other", "genForm", params).await
}

