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
 * Obtains all the configuration of the authorization detection method that is currently set for a context.
*/
pub fn get_authorization_detection_method(
    service: &ZapService,
    contextid: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid);
    super::call(
        service,
        "authorization",
        "view",
        "getAuthorizationDetectionMethod",
        params,
    )
}

/**
 * Sets the authorization detection method for a context as one that identifies un-authorized messages based on: the message's status code or a regex pattern in the response's header or body. Also, whether all conditions must match or just some can be specified via the logicalOperator parameter, which accepts two values: "AND" (default), "OR".  
*/
pub fn set_basic_authorization_detection_method(
    service: &ZapService,
    contextid: String,
    headerregex: String,
    bodyregex: String,
    statuscode: String,
    logicaloperator: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid);
    params.insert("headerRegex".to_string(), headerregex);
    params.insert("bodyRegex".to_string(), bodyregex);
    params.insert("statusCode".to_string(), statuscode);
    params.insert("logicalOperator".to_string(), logicaloperator);
    super::call(
        service,
        "authorization",
        "action",
        "setBasicAuthorizationDetectionMethod",
        params,
    )
}
