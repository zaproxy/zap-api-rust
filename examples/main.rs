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

use std::{thread, time};
use zap_api::ZapApiError;
use zap_api::ZapService;

fn main() -> Result<(), ZapApiError> {
    /*
     * These examples assume:
     *    ZAP is running on http://localhost:8090
     *    ZAP has an API key of "ChangeMe" (or its disabled)
     *    Theres a suitable target app listening on localhost:8080
     */
    // You will need a ZAP instance running for these API calls to work.
    // Change the URL if your ZAP instance is listening on another host / port and the API key if you are using one.

    let zap_url = "http://localhost:8090".to_string();
    let zap_api_key = "ChangeMe".to_string();
    let target_url = "http://localhost:8080".to_string();

    let service = ZapService {
        url: zap_url,
        api_key: zap_api_key,
    };

    // Get the ZAP version
    let res = zap_api::core::version(&service);
    let zap_version;
    match res {
        Ok(v) => zap_version = v["version"].to_string(),
        Err(e) => return Err(e),
    }

    println!("ZAP version : {}", zap_version);

    // Start the ZAP (std) spider
    println!("Starting the std spider");
    let res = zap_api::spider::scan(
        &service,
        target_url,
        "-1".to_string(),    // max children
        "true".to_string(),  // recurse
        "".to_string(),      // context name
        "false".to_string(), // subtree only
    );
    let scan_id;
    match res {
        Ok(v) => scan_id = v["scan"].to_string(),
        Err(e) => return Err(e),
    }
    println!("Scan id : {}", scan_id);

    // Loop until the spider has completed
    let mut status: i32 = 0;
    while status < 100 {
        thread::sleep(time::Duration::from_secs(1));

        let res = zap_api::spider::status(&service, scan_id.clone());
        match res {
            Ok(v) => {
                let res_status = &v["status"].as_str().unwrap();
                status = res_status.parse::<i32>().unwrap();
            }
            Err(e) => return Err(e),
        }
        println!("Scan status : {}", status);
    }

    // TODO run active scanner

    // TODO display alerts

    Ok(())
}
