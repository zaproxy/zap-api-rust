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
 * List context names of current session
*/
pub fn context_list(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "context", "view", "contextList", params)
}

/**
 * List excluded regexs for context
*/
pub fn exclude_regexs(service: &ZapService, contextname: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    super::call(service, "context", "view", "excludeRegexs", params)
}

/**
 * List included regexs for context
*/
pub fn include_regexs(service: &ZapService, contextname: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    super::call(service, "context", "view", "includeRegexs", params)
}

/**
 * List the information about the named context
*/
pub fn context(service: &ZapService, contextname: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    super::call(service, "context", "view", "context", params)
}

/**
 * Lists the names of all built in technologies
*/
pub fn technology_list(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "context", "view", "technologyList", params)
}

/**
 * Lists the names of all technologies included in a context
*/
pub fn included_technology_list(
    service: &ZapService,
    contextname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    super::call(service, "context", "view", "includedTechnologyList", params)
}

/**
 * Lists the names of all technologies excluded from a context
*/
pub fn excluded_technology_list(
    service: &ZapService,
    contextname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    super::call(service, "context", "view", "excludedTechnologyList", params)
}

/**
 * Lists the URLs accessed through/by ZAP, that belong to the context with the given name.
*/
pub fn urls(service: &ZapService, contextname: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    super::call(service, "context", "view", "urls", params)
}

/**
 * Add exclude regex to context
*/
pub fn exclude_from_context(
    service: &ZapService,
    contextname: String,
    regex: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    params.insert("regex".to_string(), regex);
    super::call(service, "context", "action", "excludeFromContext", params)
}

/**
 * Add include regex to context
*/
pub fn include_in_context(
    service: &ZapService,
    contextname: String,
    regex: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    params.insert("regex".to_string(), regex);
    super::call(service, "context", "action", "includeInContext", params)
}

/**
 * Set the regexs to include and exclude for a context, both supplied as JSON string arrays
*/
pub fn set_context_regexs(
    service: &ZapService,
    contextname: String,
    incregexs: String,
    excregexs: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    params.insert("incRegexs".to_string(), incregexs);
    params.insert("excRegexs".to_string(), excregexs);
    super::call(service, "context", "action", "setContextRegexs", params)
}

/**
 * Creates a new context with the given name in the current session
*/
pub fn new_context(service: &ZapService, contextname: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    super::call(service, "context", "action", "newContext", params)
}

/**
 * Removes a context in the current session
*/
pub fn remove_context(service: &ZapService, contextname: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    super::call(service, "context", "action", "removeContext", params)
}

/**
 * Exports the context with the given name to a file. If a relative file path is specified it will be resolved against the "contexts" directory in ZAP "home" dir.
*/
pub fn export_context(
    service: &ZapService,
    contextname: String,
    contextfile: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    params.insert("contextFile".to_string(), contextfile);
    super::call(service, "context", "action", "exportContext", params)
}

/**
 * Imports a context from a file. If a relative file path is specified it will be resolved against the "contexts" directory in ZAP "home" dir.
*/
pub fn import_context(service: &ZapService, contextfile: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextFile".to_string(), contextfile);
    super::call(service, "context", "action", "importContext", params)
}

/**
 * Includes technologies with the given names, separated by a comma, to a context
*/
pub fn include_context_technologies(
    service: &ZapService,
    contextname: String,
    technologynames: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    params.insert("technologyNames".to_string(), technologynames);
    super::call(
        service,
        "context",
        "action",
        "includeContextTechnologies",
        params,
    )
}

/**
 * Includes all built in technologies in to a context
*/
pub fn include_all_context_technologies(
    service: &ZapService,
    contextname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    super::call(
        service,
        "context",
        "action",
        "includeAllContextTechnologies",
        params,
    )
}

/**
 * Excludes technologies with the given names, separated by a comma, from a context
*/
pub fn exclude_context_technologies(
    service: &ZapService,
    contextname: String,
    technologynames: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    params.insert("technologyNames".to_string(), technologynames);
    super::call(
        service,
        "context",
        "action",
        "excludeContextTechnologies",
        params,
    )
}

/**
 * Excludes all built in technologies from a context
*/
pub fn exclude_all_context_technologies(
    service: &ZapService,
    contextname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    super::call(
        service,
        "context",
        "action",
        "excludeAllContextTechnologies",
        params,
    )
}

/**
 * Sets a context to in scope (contexts are in scope by default)
*/
pub fn set_context_in_scope(
    service: &ZapService,
    contextname: String,
    booleaninscope: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    params.insert("booleanInScope".to_string(), booleaninscope);
    super::call(service, "context", "action", "setContextInScope", params)
}
