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
use serde_json::Value;
use std::collections::HashMap;
trait Serialize {}

pub mod acsrf;
pub mod alert;
pub mod ascan;
pub mod authentication;
pub mod authorization;
pub mod autoupdate;
pub mod brk;
pub mod context;
pub mod core;
pub mod forced_user;
pub mod http_sessions;
pub mod params;
pub mod pscan;
pub mod script;
pub mod search;
pub mod session_management;
pub mod spider;
pub mod stats;
pub mod users;

#[derive(Debug)]
pub struct ZapService {
    pub url: String,     // base url of the ZAP API, eg http://localhost:8080
    pub api_key: String, // API key used for connecting securely to the API
}

#[derive(Debug)]
pub struct ZapApiError {
    kind: String,    // type of the error
    message: String, // error message
}

impl From<reqwest::Error> for ZapApiError {
    fn from(error: reqwest::Error) -> Self {
        ZapApiError {
            kind: String::from("comms"),
            message: error.to_string(),
        }
    }
}

impl From<serde_json::error::Error> for ZapApiError {
    fn from(error: serde_json::error::Error) -> Self {
        ZapApiError {
            kind: String::from("serde"),
            message: error.to_string(),
        }
    }
}

//pub fn call<S: ::std::hash::BuildHasher>(service: &ZapService, component: &str, calltype: &str, method: &str, _params: HashMap<String, String>) -> Result<Value, ZapApiError> {
pub fn call(
    service: &ZapService,
    component: &str,
    calltype: &str,
    method: &str,
    _params: HashMap<String, String>,
) -> Result<Value, ZapApiError> {
    let mut url = [&service.url, "JSON", component, calltype, method, ""].join("/");
    if _params.keys().len() > 0 {
        url.push_str("?");
        for (key, value) in _params {
            url.push_str(&key);
            url.push_str("=");
            url.push_str(&value);
            url.push_str("&");
        }
    }

    println!(
        "Component {}, calltype {}, method {}",
        component, calltype, method
    );
    println!("Url {}", url);

    //let text = reqwest::get("http://localhost:8090/JSON/core/view/version/")?
    let client = reqwest::Client::new();
    let text = client
        .get(&url)
        .header("X-ZAP-API-Key", &*service.api_key)
        .send()?
        .text()?;
    let json = serde_json::from_str(&text)?;
    Ok(json)
}
