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

use std::fs;
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

    get_version(&service)?;
    spider(&service, &target_url)?;
    scan(&service, &target_url)?;
    save_report_html(&service)?;
    save_report_json(&service)?;

    Ok(())
}

// Get the ZAP version
fn get_version(service: &ZapService) -> Result<(), ZapApiError> {
    let res = zap_api::core::version(&service);
    let zap_version;
    match res {
        Ok(v) => zap_version = v["version"].to_string(),
        Err(e) => return Err(e),
    }

    println!("ZAP version : {}", zap_version);

    Ok(())
}

// Start the ZAP (std) spider
fn spider(service: &ZapService, target_url: &String) -> Result<(), ZapApiError> {
    println!("Starting the std spider");
    let max_children = String::from("-1");
    let recurse = String::from("true");
    let context_name = String::from("");
    let subtree_only = String::from("false");

    let res = zap_api::spider::scan(
        &service,
        target_url.clone(),
        max_children,
        recurse,
        context_name,
        subtree_only,
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

    Ok(())
}

// Start the ZAP active scan
fn scan(service: &ZapService, target_url: &String) -> Result<(), ZapApiError> {
    println!("Starting the active scanner");
    let recurse = String::from("true");
    let in_scope_only: String = String::from("true");
    let scan_policy_name: String = String::from("");
    let method: String = String::from("");
    let post_data: String = String::from("");
    let context_id = String::from("");

    let res = zap_api::ascan::scan(
        &service,
        target_url.clone(),
        recurse,
        in_scope_only,
        scan_policy_name,
        method,
        post_data,
        context_id,
    );
    let scan_id;
    match res {
        Ok(v) => scan_id = v["scan"].to_string(),
        Err(e) => return Err(e),
    }
    println!("Scan id : {}", scan_id);

    // Loop until the scanner has completed
    let mut status: i32 = 0;
    while status < 100 {
        thread::sleep(time::Duration::from_secs(1));

        let res = zap_api::ascan::status(&service, scan_id.clone());
        match res {
            Ok(v) => {
                let res_status = &v["status"].as_str().unwrap();
                status = res_status.parse::<i32>().unwrap();
            }
            Err(e) => return Err(e),
        }
        println!("Scan status : {}", status);
    }

    Ok(())
}

// Save HTML Report
fn save_report_html(service: &ZapService) -> Result<(), ZapApiError> {
    println!("Saving the HTML report");
    let res = zap_api::core::htmlreport(&service);
    match res {
        Ok(v) => {
            let report_data = v;
            fs::write("zap_report.html", report_data).expect("Unable to save ZAP report");
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

// Save JSON Report
fn save_report_json(service: &ZapService) -> Result<(), ZapApiError> {
    println!("Saving the JSON report");
    let res = zap_api::core::jsonreport(&service);
    match res {
        Ok(v) => {
            let report_data = serde_json::to_string_pretty(&v).unwrap();
            fs::write("zap_report.json", report_data).expect("Unable to save ZAP report");
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
