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
 * List context names of current session
*/
pub async fn context_list(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "context", "view", "contextList", params).await
}

/**
 * List excluded regexs for context
*/
pub async fn exclude_regexs(service: &ZapService, contextname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    super::call(service, "context", "view", "excludeRegexs", params).await
}

/**
 * List included regexs for context
*/
pub async fn include_regexs(service: &ZapService, contextname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    super::call(service, "context", "view", "includeRegexs", params).await
}

/**
 * List the information about the named context
*/
pub async fn context(service: &ZapService, contextname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    super::call(service, "context", "view", "context", params).await
}

/**
 * Lists the names of all built in technologies
*/
pub async fn technology_list(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "context", "view", "technologyList", params).await
}

/**
 * Lists the names of all technologies included in a context
*/
pub async fn included_technology_list(service: &ZapService, contextname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    super::call(service, "context", "view", "includedTechnologyList", params).await
}

/**
 * Lists the names of all technologies excluded from a context
*/
pub async fn excluded_technology_list(service: &ZapService, contextname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    super::call(service, "context", "view", "excludedTechnologyList", params).await
}

/**
 * Lists the URLs accessed through/by ZAP, that belong to the context with the given name.
*/
pub async fn urls(service: &ZapService, contextname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    super::call(service, "context", "view", "urls", params).await
}

/**
 * Add exclude regex to context
*/
pub async fn exclude_from_context(service: &ZapService, contextname: &str, regex: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    params.insert("regex".to_string(), regex.to_string());
    super::call(service, "context", "action", "excludeFromContext", params).await
}

/**
 * Add include regex to context
*/
pub async fn include_in_context(service: &ZapService, contextname: &str, regex: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    params.insert("regex".to_string(), regex.to_string());
    super::call(service, "context", "action", "includeInContext", params).await
}

/**
 * Set the regexs to include and exclude for a context, both supplied as JSON string arrays
*/
pub async fn set_context_regexs(service: &ZapService, contextname: &str, incregexs: &str, excregexs: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    params.insert("incRegexs".to_string(), incregexs.to_string());
    params.insert("excRegexs".to_string(), excregexs.to_string());
    super::call(service, "context", "action", "setContextRegexs", params).await
}

/**
 * Creates a new context with the given name in the current session
*/
pub async fn new_context(service: &ZapService, contextname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    super::call(service, "context", "action", "newContext", params).await
}

/**
 * Removes a context in the current session
*/
pub async fn remove_context(service: &ZapService, contextname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    super::call(service, "context", "action", "removeContext", params).await
}

/**
 * Exports the context with the given name to a file. If a relative file path is specified it will be resolved against the "contexts" directory in ZAP "home" dir.
*/
pub async fn export_context(service: &ZapService, contextname: &str, contextfile: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    params.insert("contextFile".to_string(), contextfile.to_string());
    super::call(service, "context", "action", "exportContext", params).await
}

/**
 * Imports a context from a file. If a relative file path is specified it will be resolved against the "contexts" directory in ZAP "home" dir.
*/
pub async fn import_context(service: &ZapService, contextfile: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextFile".to_string(), contextfile.to_string());
    super::call(service, "context", "action", "importContext", params).await
}

/**
 * Includes technologies with the given names, separated by a comma, to a context
*/
pub async fn include_context_technologies(service: &ZapService, contextname: &str, technologynames: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    params.insert("technologyNames".to_string(), technologynames.to_string());
    super::call(service, "context", "action", "includeContextTechnologies", params).await
}

/**
 * Includes all built in technologies in to a context
*/
pub async fn include_all_context_technologies(service: &ZapService, contextname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    super::call(service, "context", "action", "includeAllContextTechnologies", params).await
}

/**
 * Excludes technologies with the given names, separated by a comma, from a context
*/
pub async fn exclude_context_technologies(service: &ZapService, contextname: &str, technologynames: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    params.insert("technologyNames".to_string(), technologynames.to_string());
    super::call(service, "context", "action", "excludeContextTechnologies", params).await
}

/**
 * Excludes all built in technologies from a context
*/
pub async fn exclude_all_context_technologies(service: &ZapService, contextname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    super::call(service, "context", "action", "excludeAllContextTechnologies", params).await
}

/**
 * Sets a context to in scope (contexts are in scope by default)
*/
pub async fn set_context_in_scope(service: &ZapService, contextname: &str, booleaninscope: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname.to_string());
    params.insert("booleanInScope".to_string(), booleaninscope.to_string());
    super::call(service, "context", "action", "setContextInScope", params).await
}

