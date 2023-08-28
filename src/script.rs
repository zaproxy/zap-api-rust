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
 * Lists the script engines available
*/
pub async fn list_engines(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "script", "view", "listEngines", params).await
}

/**
 * Lists the script types available.
*/
pub async fn list_types(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "script", "view", "listTypes", params).await
}

/**
 * Lists the scripts available, with its engine, name, description, type and error state.
*/
pub async fn list_scripts(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "script", "view", "listScripts", params).await
}

/**
 * Gets the value of the global variable with the given key. Returns an API error (DOES_NOT_EXIST) if no value was previously set.
*/
pub async fn global_var(service: &ZapService, varkey: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("varKey".to_string(), varkey.to_string());
    super::call(service, "script", "view", "globalVar", params).await
}

/**
 * Gets the value (string representation) of a global custom variable. Returns an API error (DOES_NOT_EXIST) if no value was previously set.
*/
pub async fn global_custom_var(service: &ZapService, varkey: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("varKey".to_string(), varkey.to_string());
    super::call(service, "script", "view", "globalCustomVar", params).await
}

/**
 * Gets all the global variables (key/value pairs).
*/
pub async fn global_vars(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "script", "view", "globalVars", params).await
}

/**
 * Gets all the global custom variables (key/value pairs, the value is the string representation).
*/
pub async fn global_custom_vars(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "script", "view", "globalCustomVars", params).await
}

/**
 * Gets the value of the variable with the given key for the given script. Returns an API error (DOES_NOT_EXIST) if no script with the given name exists or if no value was previously set.
*/
pub async fn script_var(service: &ZapService, scriptname: &str, varkey: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    params.insert("varKey".to_string(), varkey.to_string());
    super::call(service, "script", "view", "scriptVar", params).await
}

/**
 * Gets the value (string representation) of a custom variable. Returns an API error (DOES_NOT_EXIST) if no script with the given name exists or if no value was previously set.
*/
pub async fn script_custom_var(service: &ZapService, scriptname: &str, varkey: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    params.insert("varKey".to_string(), varkey.to_string());
    super::call(service, "script", "view", "scriptCustomVar", params).await
}

/**
 * Gets all the variables (key/value pairs) of the given script. Returns an API error (DOES_NOT_EXIST) if no script with the given name exists.
*/
pub async fn script_vars(service: &ZapService, scriptname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    super::call(service, "script", "view", "scriptVars", params).await
}

/**
 * Gets all the custom variables (key/value pairs, the value is the string representation) of a script. Returns an API error (DOES_NOT_EXIST) if no script with the given name exists.
*/
pub async fn script_custom_vars(service: &ZapService, scriptname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    super::call(service, "script", "view", "scriptCustomVars", params).await
}

/**
 * Enables the script with the given name
*/
pub async fn enable(service: &ZapService, scriptname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    super::call(service, "script", "action", "enable", params).await
}

/**
 * Disables the script with the given name
*/
pub async fn disable(service: &ZapService, scriptname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    super::call(service, "script", "action", "disable", params).await
}

/**
 * Loads a script into ZAP from the given local file, with the given name, type and engine, optionally with a description, and a charset name to read the script (the charset name is required if the script is not in UTF-8, for example, in ISO-8859-1).
*/
pub async fn load(service: &ZapService, scriptname: &str, scripttype: &str, scriptengine: &str, filename: &str, scriptdescription: &str, charset: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    params.insert("scriptType".to_string(), scripttype.to_string());
    params.insert("scriptEngine".to_string(), scriptengine.to_string());
    params.insert("fileName".to_string(), filename.to_string());
    params.insert("scriptDescription".to_string(), scriptdescription.to_string());
    params.insert("charset".to_string(), charset.to_string());
    super::call(service, "script", "action", "load", params).await
}

/**
 * Removes the script with the given name
*/
pub async fn remove(service: &ZapService, scriptname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    super::call(service, "script", "action", "remove", params).await
}

/**
 * Runs the stand alone script with the given name
*/
pub async fn run_stand_alone_script(service: &ZapService, scriptname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    super::call(service, "script", "action", "runStandAloneScript", params).await
}

/**
 * Clears the global variable with the given key.
*/
pub async fn clear_global_var(service: &ZapService, varkey: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("varKey".to_string(), varkey.to_string());
    super::call(service, "script", "action", "clearGlobalVar", params).await
}

/**
 * Clears a global custom variable.
*/
pub async fn clear_global_custom_var(service: &ZapService, varkey: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("varKey".to_string(), varkey.to_string());
    super::call(service, "script", "action", "clearGlobalCustomVar", params).await
}

/**
 * Clears the global variables.
*/
pub async fn clear_global_vars(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "script", "action", "clearGlobalVars", params).await
}

/**
 * Clears the variable with the given key of the given script. Returns an API error (DOES_NOT_EXIST) if no script with the given name exists.
*/
pub async fn clear_script_var(service: &ZapService, scriptname: &str, varkey: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    params.insert("varKey".to_string(), varkey.to_string());
    super::call(service, "script", "action", "clearScriptVar", params).await
}

/**
 * Clears a script custom variable.
*/
pub async fn clear_script_custom_var(service: &ZapService, scriptname: &str, varkey: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    params.insert("varKey".to_string(), varkey.to_string());
    super::call(service, "script", "action", "clearScriptCustomVar", params).await
}

/**
 * Clears the variables of the given script. Returns an API error (DOES_NOT_EXIST) if no script with the given name exists.
*/
pub async fn clear_script_vars(service: &ZapService, scriptname: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    super::call(service, "script", "action", "clearScriptVars", params).await
}

/**
 * Sets the value of the variable with the given key of the given script. Returns an API error (DOES_NOT_EXIST) if no script with the given name exists.
*/
pub async fn set_script_var(service: &ZapService, scriptname: &str, varkey: &str, varvalue: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname.to_string());
    params.insert("varKey".to_string(), varkey.to_string());
    params.insert("varValue".to_string(), varvalue.to_string());
    super::call(service, "script", "action", "setScriptVar", params).await
}

/**
 * Sets the value of the global variable with the given key.
*/
pub async fn set_global_var(service: &ZapService, varkey: &str, varvalue: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("varKey".to_string(), varkey.to_string());
    params.insert("varValue".to_string(), varvalue.to_string());
    super::call(service, "script", "action", "setGlobalVar", params).await
}

