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
 * Gets a list of users that belong to the context with the given ID, or all users if none provided.
*/
pub async fn users_list(service: &ZapService, contextid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    super::call(service, "users", "view", "usersList", params).await
}

/**
 * Gets the data of the user with the given ID that belongs to the context with the given ID.
*/
pub async fn get_user_by_id(service: &ZapService, contextid: &str, userid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("userId".to_string(), userid.to_string());
    super::call(service, "users", "view", "getUserById", params).await
}

/**
 * Gets the configuration parameters for the credentials of the context with the given ID.
*/
pub async fn get_authentication_credentials_config_params(service: &ZapService, contextid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    super::call(service, "users", "view", "getAuthenticationCredentialsConfigParams", params).await
}

/**
 * Gets the authentication credentials of the user with given ID that belongs to the context with the given ID.
*/
pub async fn get_authentication_credentials(service: &ZapService, contextid: &str, userid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("userId".to_string(), userid.to_string());
    super::call(service, "users", "view", "getAuthenticationCredentials", params).await
}

/**
 * Creates a new user with the given name for the context with the given ID.
*/
pub async fn new_user(service: &ZapService, contextid: &str, name: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("name".to_string(), name.to_string());
    super::call(service, "users", "action", "newUser", params).await
}

/**
 * Removes the user with the given ID that belongs to the context with the given ID.
*/
pub async fn remove_user(service: &ZapService, contextid: &str, userid: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("userId".to_string(), userid.to_string());
    super::call(service, "users", "action", "removeUser", params).await
}

/**
 * Sets whether or not the user, with the given ID that belongs to the context with the given ID, should be enabled.
*/
pub async fn set_user_enabled(service: &ZapService, contextid: &str, userid: &str, enabled: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("userId".to_string(), userid.to_string());
    params.insert("enabled".to_string(), enabled.to_string());
    super::call(service, "users", "action", "setUserEnabled", params).await
}

/**
 * Renames the user with the given ID that belongs to the context with the given ID.
*/
pub async fn set_user_name(service: &ZapService, contextid: &str, userid: &str, name: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("userId".to_string(), userid.to_string());
    params.insert("name".to_string(), name.to_string());
    super::call(service, "users", "action", "setUserName", params).await
}

/**
 * Sets the authentication credentials for the user with the given ID that belongs to the context with the given ID.
*/
pub async fn set_authentication_credentials(service: &ZapService, contextid: &str, userid: &str, authcredentialsconfigparams: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid.to_string());
    params.insert("userId".to_string(), userid.to_string());
    params.insert("authCredentialsConfigParams".to_string(), authcredentialsconfigparams.to_string());
    super::call(service, "users", "action", "setAuthenticationCredentials", params).await
}

