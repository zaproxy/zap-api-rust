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
 * Gets all of the sites that have sessions.
*/
pub fn sites(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "httpSessions", "view", "sites", params)
}

/**
 * Gets the sessions for the given site. Optionally returning just the session with the given name.
*/
pub fn sessions(service: &ZapService, site: String, session: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site);
    params.insert("session".to_string(), session);
    super::call(service, "httpSessions", "view", "sessions", params)
}

/**
 * Gets the name of the active session for the given site.
*/
pub fn active_session(service: &ZapService, site: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site);
    super::call(service, "httpSessions", "view", "activeSession", params)
}

/**
 * Gets the names of the session tokens for the given site.
*/
pub fn session_tokens(service: &ZapService, site: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site);
    super::call(service, "httpSessions", "view", "sessionTokens", params)
}

/**
 * Gets the default session tokens.
*/
pub fn default_session_tokens(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "httpSessions",
        "view",
        "defaultSessionTokens",
        params,
    )
}

/**
 * Creates an empty session for the given site. Optionally with the given name.
*/
pub fn create_empty_session(
    service: &ZapService,
    site: String,
    session: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site);
    params.insert("session".to_string(), session);
    super::call(
        service,
        "httpSessions",
        "action",
        "createEmptySession",
        params,
    )
}

/**
 * Removes the session from the given site.
*/
pub fn remove_session(
    service: &ZapService,
    site: String,
    session: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site);
    params.insert("session".to_string(), session);
    super::call(service, "httpSessions", "action", "removeSession", params)
}

/**
 * Sets the given session as active for the given site.
*/
pub fn set_active_session(
    service: &ZapService,
    site: String,
    session: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site);
    params.insert("session".to_string(), session);
    super::call(
        service,
        "httpSessions",
        "action",
        "setActiveSession",
        params,
    )
}

/**
 * Unsets the active session of the given site.
*/
pub fn unset_active_session(service: &ZapService, site: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site);
    super::call(
        service,
        "httpSessions",
        "action",
        "unsetActiveSession",
        params,
    )
}

/**
 * Adds the session token to the given site.
*/
pub fn add_session_token(
    service: &ZapService,
    site: String,
    sessiontoken: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site);
    params.insert("sessionToken".to_string(), sessiontoken);
    super::call(service, "httpSessions", "action", "addSessionToken", params)
}

/**
 * Removes the session token from the given site.
*/
pub fn remove_session_token(
    service: &ZapService,
    site: String,
    sessiontoken: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site);
    params.insert("sessionToken".to_string(), sessiontoken);
    super::call(
        service,
        "httpSessions",
        "action",
        "removeSessionToken",
        params,
    )
}

/**
 * Sets the value of the session token of the given session for the given site.
*/
pub fn set_session_token_value(
    service: &ZapService,
    site: String,
    session: String,
    sessiontoken: String,
    tokenvalue: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site);
    params.insert("session".to_string(), session);
    params.insert("sessionToken".to_string(), sessiontoken);
    params.insert("tokenValue".to_string(), tokenvalue);
    super::call(
        service,
        "httpSessions",
        "action",
        "setSessionTokenValue",
        params,
    )
}

/**
 * Renames the session of the given site.
*/
pub fn rename_session(
    service: &ZapService,
    site: String,
    oldsessionname: String,
    newsessionname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("site".to_string(), site);
    params.insert("oldSessionName".to_string(), oldsessionname);
    params.insert("newSessionName".to_string(), newsessionname);
    super::call(service, "httpSessions", "action", "renameSession", params)
}

/**
 * Adds a default session token with the given name and enabled state.
*/
pub fn add_default_session_token(
    service: &ZapService,
    sessiontoken: String,
    tokenenabled: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("sessionToken".to_string(), sessiontoken);
    params.insert("tokenEnabled".to_string(), tokenenabled);
    super::call(
        service,
        "httpSessions",
        "action",
        "addDefaultSessionToken",
        params,
    )
}

/**
 * Sets whether or not the default session token with the given name is enabled.
*/
pub fn set_default_session_token_enabled(
    service: &ZapService,
    sessiontoken: String,
    tokenenabled: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("sessionToken".to_string(), sessiontoken);
    params.insert("tokenEnabled".to_string(), tokenenabled);
    super::call(
        service,
        "httpSessions",
        "action",
        "setDefaultSessionTokenEnabled",
        params,
    )
}

/**
 * Removes the default session token with the given name.
*/
pub fn remove_default_session_token(
    service: &ZapService,
    sessiontoken: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("sessionToken".to_string(), sessiontoken);
    super::call(
        service,
        "httpSessions",
        "action",
        "removeDefaultSessionToken",
        params,
    )
}
