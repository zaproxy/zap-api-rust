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
 * Lists the alert filters of the context with the given ID.
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub async fn alert_filter_list(
    service: &ZapService,
    contextid: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid);
    super::call(service, "alertFilter", "view", "alertFilterList", params).await
}

/**
 * Adds a new alert filter for the context with the given ID.
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
#[allow(clippy::too_many_arguments)]
pub async fn add_alert_filter(
    service: &ZapService,
    contextid: String,
    ruleid: String,
    newlevel: String,
    url: String,
    urlisregex: String,
    parameter: String,
    enabled: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid);
    params.insert("ruleId".to_string(), ruleid);
    params.insert("newLevel".to_string(), newlevel);
    params.insert("url".to_string(), url);
    params.insert("urlIsRegex".to_string(), urlisregex);
    params.insert("parameter".to_string(), parameter);
    params.insert("enabled".to_string(), enabled);
    super::call(service, "alertFilter", "action", "addAlertFilter", params).await
}

/**
 * Removes an alert filter from the context with the given ID.
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
#[allow(clippy::too_many_arguments)]
pub async fn remove_alert_filter(
    service: &ZapService,
    contextid: String,
    ruleid: String,
    newlevel: String,
    url: String,
    urlisregex: String,
    parameter: String,
    enabled: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextId".to_string(), contextid);
    params.insert("ruleId".to_string(), ruleid);
    params.insert("newLevel".to_string(), newlevel);
    params.insert("url".to_string(), url);
    params.insert("urlIsRegex".to_string(), urlisregex);
    params.insert("parameter".to_string(), parameter);
    params.insert("enabled".to_string(), enabled);
    super::call(
        service,
        "alertFilter",
        "action",
        "removeAlertFilter",
        params,
    )
    .await
}
