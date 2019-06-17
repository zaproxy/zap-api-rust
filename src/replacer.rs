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
 * Returns full details of all of the rules
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn rules(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "replacer", "view", "rules", params)
}

/**
 * Adds a replacer rule. For the parameters: desc is a user friendly description, enabled is true or false, matchType is one of [REQ_HEADER, REQ_HEADER_STR, REQ_BODY_STR, RESP_HEADER, RESP_HEADER_STR, RESP_BODY_STR], matchRegex should be true if the matchString should be treated as a regex otherwise false, matchString is the string that will be matched against, replacement is the replacement string, initiators may be blank (for all initiators) or a comma separated list of integers as defined in <a href="https://github.com/zaproxy/zaproxy/blob/develop/src/org/parosproxy/paros/network/HttpSender.java">HttpSender</a>  
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
#[allow(clippy::too_many_arguments)]
pub fn add_rule(
    service: &ZapService,
    description: String,
    enabled: String,
    matchtype: String,
    matchregex: String,
    matchstring: String,
    replacement: String,
    initiators: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("description".to_string(), description);
    params.insert("enabled".to_string(), enabled);
    params.insert("matchType".to_string(), matchtype);
    params.insert("matchRegex".to_string(), matchregex);
    params.insert("matchString".to_string(), matchstring);
    params.insert("replacement".to_string(), replacement);
    params.insert("initiators".to_string(), initiators);
    super::call(service, "replacer", "action", "addRule", params)
}

/**
 * Removes the rule with the given description
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn remove_rule(service: &ZapService, description: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("description".to_string(), description);
    super::call(service, "replacer", "action", "removeRule", params)
}

/**
 * Enables or disables the rule with the given description based on the bool parameter  
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn set_enabled(
    service: &ZapService,
    description: String,
    bool: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("description".to_string(), description);
    params.insert("bool".to_string(), bool);
    super::call(service, "replacer", "action", "setEnabled", params)
}
