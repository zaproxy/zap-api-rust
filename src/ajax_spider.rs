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
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn status(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ajaxSpider", "view", "status", params)
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn results(service: &ZapService, start: String, count: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    super::call(service, "ajaxSpider", "view", "results", params)
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn number_of_results(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ajaxSpider", "view", "numberOfResults", params)
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn full_results(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ajaxSpider", "view", "fullResults", params)
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn option_browser_id(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ajaxSpider", "view", "optionBrowserId", params)
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn option_event_wait(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ajaxSpider", "view", "optionEventWait", params)
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn option_max_crawl_depth(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ajaxSpider", "view", "optionMaxCrawlDepth", params)
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn option_max_crawl_states(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "ajaxSpider",
        "view",
        "optionMaxCrawlStates",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn option_max_duration(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ajaxSpider", "view", "optionMaxDuration", params)
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn option_number_of_browsers(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "ajaxSpider",
        "view",
        "optionNumberOfBrowsers",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn option_reload_wait(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ajaxSpider", "view", "optionReloadWait", params)
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn option_click_default_elems(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "ajaxSpider",
        "view",
        "optionClickDefaultElems",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn option_click_elems_once(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "ajaxSpider",
        "view",
        "optionClickElemsOnce",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn option_random_inputs(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ajaxSpider", "view", "optionRandomInputs", params)
}

/**
 * Runs the spider against the given URL and/or context, optionally, spidering everything in scope. The parameter 'contextName' can be used to constrain the scan to a Context, the option 'in scope' is ignored if a context was also specified. The parameter 'subtreeOnly' allows to restrict the spider under a site's subtree (using the specified 'url').
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn scan(
    service: &ZapService,
    url: String,
    inscope: String,
    contextname: String,
    subtreeonly: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("url".to_string(), url);
    params.insert("inScope".to_string(), inscope);
    params.insert("contextName".to_string(), contextname);
    params.insert("subtreeOnly".to_string(), subtreeonly);
    super::call(service, "ajaxSpider", "action", "scan", params)
}

/**
 * Runs the spider from the perspective of a User, obtained using the given context name and user name. The parameter 'url' allows to specify the starting point for the spider, otherwise it's used an existing URL from the context (if any). The parameter 'subtreeOnly' allows to restrict the spider under a site's subtree (using the specified 'url').
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn scan_as_user(
    service: &ZapService,
    contextname: String,
    username: String,
    url: String,
    subtreeonly: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("contextName".to_string(), contextname);
    params.insert("userName".to_string(), username);
    params.insert("url".to_string(), url);
    params.insert("subtreeOnly".to_string(), subtreeonly);
    super::call(service, "ajaxSpider", "action", "scanAsUser", params)
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn stop(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "ajaxSpider", "action", "stop", params)
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn set_option_browser_id(service: &ZapService, string: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("String".to_string(), string);
    super::call(
        service,
        "ajaxSpider",
        "action",
        "setOptionBrowserId",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn set_option_click_default_elems(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "ajaxSpider",
        "action",
        "setOptionClickDefaultElems",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn set_option_click_elems_once(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "ajaxSpider",
        "action",
        "setOptionClickElemsOnce",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn set_option_event_wait(service: &ZapService, integer: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "ajaxSpider",
        "action",
        "setOptionEventWait",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn set_option_max_crawl_depth(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "ajaxSpider",
        "action",
        "setOptionMaxCrawlDepth",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn set_option_max_crawl_states(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "ajaxSpider",
        "action",
        "setOptionMaxCrawlStates",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn set_option_max_duration(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "ajaxSpider",
        "action",
        "setOptionMaxDuration",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn set_option_number_of_browsers(
    service: &ZapService,
    integer: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "ajaxSpider",
        "action",
        "setOptionNumberOfBrowsers",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn set_option_random_inputs(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "ajaxSpider",
        "action",
        "setOptionRandomInputs",
        params,
    )
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub fn set_option_reload_wait(service: &ZapService, integer: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Integer".to_string(), integer);
    super::call(
        service,
        "ajaxSpider",
        "action",
        "setOptionReloadWait",
        params,
    )
}
