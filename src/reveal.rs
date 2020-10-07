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
 * Tells if shows hidden fields and enables disabled fields
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub async fn reveal(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "reveal", "view", "reveal", params).await
}

/**
 * Sets if shows hidden fields and enables disabled fields
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub async fn set_reveal(service: &ZapService, reveal: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("reveal".to_string(), reveal);
    super::call(service, "reveal", "action", "setReveal", params).await
}
