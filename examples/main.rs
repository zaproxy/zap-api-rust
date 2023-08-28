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

use serde::{Deserialize, Serialize};
use std::thread;
use std::time;
use zap_api::ZapApiError;
use zap_api::ZapService;
#[tokio::main]
async fn main() -> Result<(), ZapApiError> {
    /*
     * These examples assume:
     *    ZAP is running on http://localhost:8080
     *    ZAP has an API key of "ChangeMe" (or its disabled)
     *    Theres a suitable target app listening on localhost:3000
     */
    // You will need a ZAP instance running for these API calls to work.
    // Change the URL if your ZAP instance is listening on another host / port and the API key if you are using one.

    let zap_url = "http://localhost:8080".to_string();
    let zap_api_key = "ChangeMe".to_string();
    let target_url = "http://localhost:3000".to_string();

    let service = ZapService {
        url: zap_url,
        api_key: zap_api_key,
    };

    // Include target url in scope
    zap_api::context::new_context(&service, "api").await?;
    zap_api::context::include_in_context(&service, "api", "http://localhost:3000/*").await?;

    version(&service).await?;
    spider(&service, &target_url).await?;
    scan(&service, &target_url).await?;
    alerts(&service, &target_url).await?;

    Ok(())
}

// Get the ZAP version
async fn version(service: &ZapService) -> Result<(), ZapApiError> {
    let res = zap_api::core::version(service).await?;
    let zap_version = res["version"].to_string();
    println!("ZAP version : {}", zap_version);

    Ok(())
}

//Start the ZAP (std) spider
async fn spider(service: &ZapService, target_url: &str) -> Result<(), ZapApiError> {
    println!("Starting the std spider");
    let res = zap_api::spider::scan(
        service, target_url, "-1",    // max children
        "true",  // recurse
        "",      // context name
        "false", // subtree only
    )
    .await?;

    let scan_id = res["scan"].as_str().unwrap().to_owned();
    println!("Scan id : {}", scan_id);

    // Loop until the spider has completed
    let mut status: i32 = 0;
    while status < 100 {
        thread::sleep(time::Duration::from_secs(3));
        let res = zap_api::spider::status(&service, &scan_id).await?;
        status = res["status"].as_str().unwrap().parse::<i32>().unwrap();
        println!("Spider status : {}", status);
    }

    Ok(())
}

//Start the ZAP (std) scanner
async fn scan(service: &ZapService, target_url: &str) -> Result<(), ZapApiError> {
    println!("Starting the std scanner");
    let res = zap_api::ascan::scan(
        service, target_url, "true", // recurse
        "true", // inscope only
        "",     // scan policy name
        "",     // method
        "",     // post data
        "",     // context ID
    )
    .await?;

    let scan_id = res["scan"].as_str().unwrap().to_owned();
    println!("Scan id : {}", scan_id);

    // Loop until the scan has completed
    let mut status: i32 = 0;
    while status < 100 {
        thread::sleep(time::Duration::from_secs(5));
        let res = zap_api::ascan::status(&service, &scan_id).await?;
        status = res["status"].as_str().unwrap().parse::<i32>().unwrap();
        println!("Scan status : {}", status);
    }

    Ok(())
}

// Alerts
async fn alerts(service: &ZapService, baseurl: &str) -> Result<(), ZapApiError> {
    let res = zap_api::alert::alerts(service, baseurl, "", "", "").await?;
    let alert_data: ZapAlerts = serde_json::from_value(res).unwrap();

    println!("Number of alerts: {}", alert_data.alerts.len());
    alert_data.alerts.into_iter().for_each(|alert| {
        println!(
            "ID: {}\tConfidence: {}\tAlert: {}",
            alert.id, alert.confidence, alert.alert
        )
    });

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct ZapAlerts {
    alerts: Vec<Alerts>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Alerts {
    id: String,
    alert: String,
    confidence: String,
}
