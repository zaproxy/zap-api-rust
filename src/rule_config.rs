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
 * Show the specified rule configuration
*/
pub async fn rule_config_value(service: &ZapService, key: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("key".to_string(), key.to_string());
    super::call(service, "ruleConfig", "view", "ruleConfigValue", params).await
}

/**
 * Show all of the rule configurations
*/
pub async fn all_rule_configs(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ruleConfig", "view", "allRuleConfigs", params).await
}

/**
 * Reset the specified rule configuration, which must already exist
*/
pub async fn reset_rule_config_value(service: &ZapService, key: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("key".to_string(), key.to_string());
    super::call(service, "ruleConfig", "action", "resetRuleConfigValue", params).await
}

/**
 * Reset all of the rule configurations
*/
pub async fn reset_all_rule_config_values(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ruleConfig", "action", "resetAllRuleConfigValues", params).await
}

/**
 * Set the specified rule configuration, which must already exist
*/
pub async fn set_rule_config_value(service: &ZapService, key: &str, value: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("key".to_string(), key.to_string());
    params.insert("value".to_string(), value.to_string());
    super::call(service, "ruleConfig", "action", "setRuleConfigValue", params).await
}

