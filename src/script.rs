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
 * Lists the script engines available
*/
pub fn list_engines(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "script", "view", "listEngines", params)
}

/**
 * Lists the script types available.
*/
pub fn list_types(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "script", "view", "listTypes", params)
}

/**
 * Lists the scripts available, with its engine, name, description, type and error state.
*/
pub fn list_scripts(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "script", "view", "listScripts", params)
}

/**
 * Gets the value of the global variable with the given key. Returns an API error (DOES_NOT_EXIST) if no value was previously set.
*/
pub fn global_var(service: &ZapService, varkey: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("varKey".to_string(), varkey);
    super::call(service, "script", "view", "globalVar", params)
}

/**
 * Gets all the global variables (key/value pairs).
*/
pub fn global_vars(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "script", "view", "globalVars", params)
}

/**
 * Gets the value of the variable with the given key for the given script. Returns an API error (DOES_NOT_EXIST) if no script with the given name exists or if no value was previously set.
*/
pub fn script_var(
    service: &ZapService,
    scriptname: String,
    varkey: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname);
    params.insert("varKey".to_string(), varkey);
    super::call(service, "script", "view", "scriptVar", params)
}

/**
 * Gets all the variables (key/value pairs) of the given script. Returns an API error (DOES_NOT_EXIST) if no script with the given name exists.
*/
pub fn script_vars(service: &ZapService, scriptname: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname);
    super::call(service, "script", "view", "scriptVars", params)
}

/**
 * Enables the script with the given name
*/
pub fn enable(service: &ZapService, scriptname: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname);
    super::call(service, "script", "action", "enable", params)
}

/**
 * Disables the script with the given name
*/
pub fn disable(service: &ZapService, scriptname: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname);
    super::call(service, "script", "action", "disable", params)
}

/**
 * Loads a script into ZAP from the given local file, with the given name, type and engine, optionally with a description, and a charset name to read the script (the charset name is required if the script is not in UTF-8, for example, in ISO-8859-1).
*/
pub fn load(
    service: &ZapService,
    scriptname: String,
    scripttype: String,
    scriptengine: String,
    filename: String,
    scriptdescription: String,
    charset: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname);
    params.insert("scriptType".to_string(), scripttype);
    params.insert("scriptEngine".to_string(), scriptengine);
    params.insert("fileName".to_string(), filename);
    params.insert("scriptDescription".to_string(), scriptdescription);
    params.insert("charset".to_string(), charset);
    super::call(service, "script", "action", "load", params)
}

/**
 * Removes the script with the given name
*/
pub fn remove(service: &ZapService, scriptname: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname);
    super::call(service, "script", "action", "remove", params)
}

/**
 * Runs the stand alone script with the given name
*/
pub fn run_stand_alone_script(
    service: &ZapService,
    scriptname: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname);
    super::call(service, "script", "action", "runStandAloneScript", params)
}

/**
 * Clears the global variable with the given key.
*/
pub fn clear_global_var(service: &ZapService, varkey: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("varKey".to_string(), varkey);
    super::call(service, "script", "action", "clearGlobalVar", params)
}

/**
 * Clears the global variables.
*/
pub fn clear_global_vars(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "script", "action", "clearGlobalVars", params)
}

/**
 * Clears the variable with the given key of the given script. Returns an API error (DOES_NOT_EXIST) if no script with the given name exists.
*/
pub fn clear_script_var(
    service: &ZapService,
    scriptname: String,
    varkey: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname);
    params.insert("varKey".to_string(), varkey);
    super::call(service, "script", "action", "clearScriptVar", params)
}

/**
 * Clears the variables of the given script. Returns an API error (DOES_NOT_EXIST) if no script with the given name exists.
*/
pub fn clear_script_vars(service: &ZapService, scriptname: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname);
    super::call(service, "script", "action", "clearScriptVars", params)
}

/**
 * Sets the value of the variable with the given key of the given script. Returns an API error (DOES_NOT_EXIST) if no script with the given name exists.
*/
pub fn set_script_var(
    service: &ZapService,
    scriptname: String,
    varkey: String,
    varvalue: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("scriptName".to_string(), scriptname);
    params.insert("varKey".to_string(), varkey);
    params.insert("varValue".to_string(), varvalue);
    super::call(service, "script", "action", "setScriptVar", params)
}

/**
 * Sets the value of the global variable with the given key.
*/
pub fn set_global_var(
    service: &ZapService,
    varkey: String,
    varvalue: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("varKey".to_string(), varkey);
    params.insert("varValue".to_string(), varvalue);
    super::call(service, "script", "action", "setGlobalVar", params)
}
