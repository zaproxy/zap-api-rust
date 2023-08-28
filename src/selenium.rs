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
 * Returns the current path to ChromeDriver
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub async fn option_chrome_driver_path(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "selenium",
        "view",
        "optionChromeDriverPath",
        params,
    )
    .await
}

/**
 * Returns the current path to Firefox binary
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub async fn option_firefox_binary_path(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "selenium",
        "view",
        "optionFirefoxBinaryPath",
        params,
    )
    .await
}

/**
 * Returns the current path to Firefox driver (geckodriver)
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub async fn option_firefox_driver_path(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "selenium",
        "view",
        "optionFirefoxDriverPath",
        params,
    )
    .await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn option_ie_driver_path(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "selenium", "view", "optionIeDriverPath", params).await
}

/**
 * Returns the current path to PhantomJS binary
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub async fn option_phantom_js_binary_path(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "selenium",
        "view",
        "optionPhantomJsBinaryPath",
        params,
    )
    .await
}

/**
 * Sets the current path to ChromeDriver
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub async fn set_option_chrome_driver_path(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "selenium",
        "action",
        "setOptionChromeDriverPath",
        params,
    )
    .await
}

/**
 * Sets the current path to Firefox binary
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub async fn set_option_firefox_binary_path(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "selenium",
        "action",
        "setOptionFirefoxBinaryPath",
        params,
    )
    .await
}

/**
 * Sets the current path to Firefox driver (geckodriver)
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub async fn set_option_firefox_driver_path(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "selenium",
        "action",
        "setOptionFirefoxDriverPath",
        params,
    )
    .await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn set_option_ie_driver_path(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "selenium",
        "action",
        "setOptionIeDriverPath",
        params,
    )
    .await
}

/**
 * Sets the current path to PhantomJS binary
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub async fn set_option_phantom_js_binary_path(
    service: &ZapService,
    string: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "selenium",
        "action",
        "setOptionPhantomJsBinaryPath",
        params,
    )
    .await
}
