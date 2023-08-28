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
pub async fn import_zap_log_from_file(
    service: &ZapService,
    filepath: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("FilePath".to_string(), filepath);
    super::call(
        service,
        "importLogFiles",
        "action",
        "ImportZAPLogFromFile",
        params,
    )
    .await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn import_mod_security_log_from_file(
    service: &ZapService,
    filepath: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("FilePath".to_string(), filepath);
    super::call(
        service,
        "importLogFiles",
        "action",
        "ImportModSecurityLogFromFile",
        params,
    )
    .await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn import_zap_http_request_response_pair(
    service: &ZapService,
    httprequest: String,
    httpresponse: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("HTTPRequest".to_string(), httprequest);
    params.insert("HTTPResponse".to_string(), httpresponse);
    super::call(
        service,
        "importLogFiles",
        "action",
        "ImportZAPHttpRequestResponsePair",
        params,
    )
    .await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn post_mod_security_audit_event(
    service: &ZapService,
    auditeventstring: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("AuditEventString".to_string(), auditeventstring);
    super::call(
        service,
        "importLogFiles",
        "action",
        "PostModSecurityAuditEvent",
        params,
    )
    .await
}

/**
 * This component is optional and therefore the API will only work if it is installed
 */
pub async fn other_post_mod_security_audit_event(
    service: &ZapService,
    auditeventstring: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("AuditEventString".to_string(), auditeventstring);
    super::call(
        service,
        "importLogFiles",
        "other",
        "OtherPostModSecurityAuditEvent",
        params,
    )
    .await
}
