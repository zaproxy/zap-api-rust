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
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn monitor(
    service: &ZapService,
    id: String,
    message: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    params.insert("message".to_string(), message);
    super::call(service, "pnh", "action", "monitor", params).await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn oracle(service: &ZapService, id: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    super::call(service, "pnh", "action", "oracle", params).await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn start_monitoring(service: &ZapService, url: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url);
    super::call(service, "pnh", "action", "startMonitoring", params).await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn stop_monitoring(service: &ZapService, id: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    super::call(service, "pnh", "action", "stopMonitoring", params).await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn pnh(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pnh", "other", "pnh", params).await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn manifest(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pnh", "other", "manifest", params).await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn service(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pnh", "other", "service", params).await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn fx_pnh_xpi(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "pnh", "other", "fx_pnh.xpi", params).await
}
