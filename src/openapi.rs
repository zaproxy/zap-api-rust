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
 * Import an Open API definition from a local file.
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn import_file(service: &ZapService, file: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("file".to_string(), file);
    super::call(service, "openapi", "action", "importFile", params)
}

/**
 * Import an Open API definition from a URL, hostOverride allows the host to be replaced
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn import_url(
    service: &ZapService,
    url: String,
    hostoverride: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url);
    params.insert("hostOverride".to_string(), hostoverride);
    super::call(service, "openapi", "action", "importUrl", params)
}
