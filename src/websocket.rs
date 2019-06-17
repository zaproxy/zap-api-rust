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
 * Returns all of the registered web socket channels
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn channels(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "websocket", "view", "channels", params)
}

/**
 * Returns full details of the message specified by the channelId and messageId
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn message(
    service: &ZapService,
    channelid: String,
    messageid: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("channelId".to_string(), channelid);
    params.insert("messageId".to_string(), messageid);
    super::call(service, "websocket", "view", "message", params)
}

/**
 * Returns a list of all of the messages that meet the given criteria (all optional), where channelId is a channel identifier, start is the offset to start returning messages from (starting from 0), count is the number of messages to return (default no limit) and payloadPreviewLength is the maximum number bytes to return for the payload contents
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn messages(
    service: &ZapService,
    channelid: String,
    start: String,
    count: String,
    payloadpreviewlength: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("channelId".to_string(), channelid);
    params.insert("start".to_string(), start);
    params.insert("count".to_string(), count);
    params.insert("payloadPreviewLength".to_string(), payloadpreviewlength);
    super::call(service, "websocket", "view", "messages", params)
}

/**
 * Returns a text representation of an intercepted websockets message
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn break_text_message(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "websocket", "view", "breakTextMessage", params)
}

/**
 * Sends the specified message on the channel specified by channelId, if outgoing is 'True' then the message will be sent to the server and if it is 'False' then it will be sent to the client
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn send_text_message(
    service: &ZapService,
    channelid: String,
    outgoing: String,
    message: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("channelId".to_string(), channelid);
    params.insert("outgoing".to_string(), outgoing);
    params.insert("message".to_string(), message);
    super::call(service, "websocket", "action", "sendTextMessage", params)
}

/**
 * Sets the text message for an intercepted websockets message
 * <p>
 * This component is optional and therefore the API will only work if it is installed
*/
pub fn set_break_text_message(
    service: &ZapService,
    message: String,
    outgoing: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("message".to_string(), message);
    params.insert("outgoing".to_string(), outgoing);
    super::call(
        service,
        "websocket",
        "action",
        "setBreakTextMessage",
        params,
    )
}
