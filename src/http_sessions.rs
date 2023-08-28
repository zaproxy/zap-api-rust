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
 * Gets all of the sites that have sessions.
*/
pub async fn sites(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "httpSessions", "view", "sites", params).await
}

/**
 * Gets the sessions for the given site. Optionally returning just the session with the given name.
*/
pub async fn sessions(service: &ZapService, site: &str, session: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site.to_string());
    params.insert("session".to_string(), session.to_string());
    super::call(service, "httpSessions", "view", "sessions", params).await
}

/**
 * Gets the name of the active session for the given site.
*/
pub async fn active_session(service: &ZapService, site: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site.to_string());
    super::call(service, "httpSessions", "view", "activeSession", params).await
}

/**
 * Gets the names of the session tokens for the given site.
*/
pub async fn session_tokens(service: &ZapService, site: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site.to_string());
    super::call(service, "httpSessions", "view", "sessionTokens", params).await
}

/**
 * Gets the default session tokens.
*/
pub async fn default_session_tokens(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "httpSessions", "view", "defaultSessionTokens", params).await
}

/**
 * Creates an empty session for the given site. Optionally with the given name.
*/
pub async fn create_empty_session(service: &ZapService, site: &str, session: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site.to_string());
    params.insert("session".to_string(), session.to_string());
    super::call(service, "httpSessions", "action", "createEmptySession", params).await
}

/**
 * Removes the session from the given site.
*/
pub async fn remove_session(service: &ZapService, site: &str, session: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site.to_string());
    params.insert("session".to_string(), session.to_string());
    super::call(service, "httpSessions", "action", "removeSession", params).await
}

/**
 * Sets the given session as active for the given site.
*/
pub async fn set_active_session(service: &ZapService, site: &str, session: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site.to_string());
    params.insert("session".to_string(), session.to_string());
    super::call(service, "httpSessions", "action", "setActiveSession", params).await
}

/**
 * Unsets the active session of the given site.
*/
pub async fn unset_active_session(service: &ZapService, site: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site.to_string());
    super::call(service, "httpSessions", "action", "unsetActiveSession", params).await
}

/**
 * Adds the session token to the given site.
*/
pub async fn add_session_token(service: &ZapService, site: &str, sessiontoken: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site.to_string());
    params.insert("sessionToken".to_string(), sessiontoken.to_string());
    super::call(service, "httpSessions", "action", "addSessionToken", params).await
}

/**
 * Removes the session token from the given site.
*/
pub async fn remove_session_token(service: &ZapService, site: &str, sessiontoken: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site.to_string());
    params.insert("sessionToken".to_string(), sessiontoken.to_string());
    super::call(service, "httpSessions", "action", "removeSessionToken", params).await
}

/**
 * Sets the value of the session token of the given session for the given site.
*/
pub async fn set_session_token_value(service: &ZapService, site: &str, session: &str, sessiontoken: &str, tokenvalue: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site.to_string());
    params.insert("session".to_string(), session.to_string());
    params.insert("sessionToken".to_string(), sessiontoken.to_string());
    params.insert("tokenValue".to_string(), tokenvalue.to_string());
    super::call(service, "httpSessions", "action", "setSessionTokenValue", params).await
}

/**
 * Renames the session of the given site.
*/
pub async fn rename_session(service: &ZapService, site: &str, oldsessionname: &str, newsessionname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site.to_string());
    params.insert("oldSessionName".to_string(), oldsessionname.to_string());
    params.insert("newSessionName".to_string(), newsessionname.to_string());
    super::call(service, "httpSessions", "action", "renameSession", params).await
}

/**
 * Adds a default session token with the given name and enabled state.
*/
pub async fn add_default_session_token(service: &ZapService, sessiontoken: &str, tokenenabled: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("sessionToken".to_string(), sessiontoken.to_string());
    params.insert("tokenEnabled".to_string(), tokenenabled.to_string());
    super::call(service, "httpSessions", "action", "addDefaultSessionToken", params).await
}

/**
 * Sets whether or not the default session token with the given name is enabled.
*/
pub async fn set_default_session_token_enabled(service: &ZapService, sessiontoken: &str, tokenenabled: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("sessionToken".to_string(), sessiontoken.to_string());
    params.insert("tokenEnabled".to_string(), tokenenabled.to_string());
    super::call(service, "httpSessions", "action", "setDefaultSessionTokenEnabled", params).await
}

/**
 * Removes the default session token with the given name.
*/
pub async fn remove_default_session_token(service: &ZapService, sessiontoken: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("sessionToken".to_string(), sessiontoken.to_string());
    super::call(service, "httpSessions", "action", "removeDefaultSessionToken", params).await
}

