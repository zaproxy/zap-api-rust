/* Zed Attack Proxy (ZAP) and its related class files.
 *
 * ZAP is an HTTP/HTTPS proxy for assessing web application security.
 *
 * Copyright 2020 the ZAP development team
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
 * Returns the latest version number
*/
pub async fn latest_version_number(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "latestVersionNumber", params).await
}

/**
 * Returns 'true' if ZAP is on the latest version
*/
pub async fn is_latest_version(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "isLatestVersion", params).await
}

/**
 * Return a list of all of the installed add-ons
*/
pub async fn installed_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "installedAddons", params).await
}

/**
 * Returns a list with all local add-ons, installed or not.
*/
pub async fn local_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "localAddons", params).await
}

/**
 * Return a list of any add-ons that have been added to the Marketplace since the last check for updates
*/
pub async fn new_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "newAddons", params).await
}

/**
 * Return a list of any add-ons that have been changed in the Marketplace since the last check for updates
*/
pub async fn updated_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "updatedAddons", params).await
}

/**
 * Return a list of all of the add-ons on the ZAP Marketplace (this information is read once and then cached)
*/
pub async fn marketplace_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "marketplaceAddons", params).await
}

/**
 * 
*/
pub async fn option_addon_directories(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionAddonDirectories", params).await
}

/**
 * 
*/
pub async fn option_day_last_checked(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionDayLastChecked", params).await
}

/**
 * 
*/
pub async fn option_day_last_install_warned(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionDayLastInstallWarned", params).await
}

/**
 * 
*/
pub async fn option_day_last_update_warned(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionDayLastUpdateWarned", params).await
}

/**
 * 
*/
pub async fn option_download_directory(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionDownloadDirectory", params).await
}

/**
 * 
*/
pub async fn option_check_addon_updates(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionCheckAddonUpdates", params).await
}

/**
 * 
*/
pub async fn option_check_on_start(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionCheckOnStart", params).await
}

/**
 * 
*/
pub async fn option_download_new_release(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionDownloadNewRelease", params).await
}

/**
 * 
*/
pub async fn option_install_addon_updates(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionInstallAddonUpdates", params).await
}

/**
 * 
*/
pub async fn option_install_scanner_rules(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionInstallScannerRules", params).await
}

/**
 * 
*/
pub async fn option_report_alpha_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionReportAlphaAddons", params).await
}

/**
 * 
*/
pub async fn option_report_beta_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionReportBetaAddons", params).await
}

/**
 * 
*/
pub async fn option_report_release_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionReportReleaseAddons", params).await
}

/**
 * Downloads the latest release, if any 
*/
pub async fn download_latest_release(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "action", "downloadLatestRelease", params).await
}

/**
 * Installs or updates the specified add-on, returning when complete (i.e. not asynchronously)
*/
pub async fn install_addon(service: &ZapService, id: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    super::call(service, "autoupdate", "action", "installAddon", params).await
}

/**
 * 
*/
pub async fn install_local_addon(service: &ZapService, file: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("file".to_string(), file.to_string());
    super::call(service, "autoupdate", "action", "installLocalAddon", params).await
}

/**
 * Uninstalls the specified add-on 
*/
pub async fn uninstall_addon(service: &ZapService, id: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id.to_string());
    super::call(service, "autoupdate", "action", "uninstallAddon", params).await
}

/**
 * 
*/
pub async fn set_option_check_addon_updates(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "autoupdate", "action", "setOptionCheckAddonUpdates", params).await
}

/**
 * 
*/
pub async fn set_option_check_on_start(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "autoupdate", "action", "setOptionCheckOnStart", params).await
}

/**
 * 
*/
pub async fn set_option_download_new_release(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "autoupdate", "action", "setOptionDownloadNewRelease", params).await
}

/**
 * 
*/
pub async fn set_option_install_addon_updates(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "autoupdate", "action", "setOptionInstallAddonUpdates", params).await
}

/**
 * 
*/
pub async fn set_option_install_scanner_rules(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "autoupdate", "action", "setOptionInstallScannerRules", params).await
}

/**
 * 
*/
pub async fn set_option_report_alpha_addons(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "autoupdate", "action", "setOptionReportAlphaAddons", params).await
}

/**
 * 
*/
pub async fn set_option_report_beta_addons(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "autoupdate", "action", "setOptionReportBetaAddons", params).await
}

/**
 * 
*/
pub async fn set_option_report_release_addons(service: &ZapService, boolean: &str) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean.to_string());
    super::call(service, "autoupdate", "action", "setOptionReportReleaseAddons", params).await
}

