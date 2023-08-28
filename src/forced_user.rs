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
 * Returns 'true' if 'forced user' mode is enabled, 'false' otherwise
*/
pub async fn is_forced_user_mode_enabled(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "forcedUser", "view", "isForcedUserModeEnabled", params).await
}

/**
 * Gets the user (ID) set as 'forced user' for the given context (ID)
*/
pub async fn get_forced_user(service: &ZapService, contextid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    super::call(service, "forcedUser", "view", "getForcedUser", params).await
}

/**
 * Sets the user (ID) that should be used in 'forced user' mode for the given context (ID)
*/
pub async fn set_forced_user(service: &ZapService, contextid: &str, userid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("userId".to_string(), userid.to_string());
    super::call(service, "forcedUser", "action", "setForcedUser", params).await
}

/**
 * Sets if 'forced user' mode should be enabled or not
*/
pub async fn set_forced_user_mode_enabled(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("boolean".to_string(), boolean.to_string());
    super::call(service, "forcedUser", "action", "setForcedUserModeEnabled", params).await
}

