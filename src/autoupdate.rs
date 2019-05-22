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
 * Returns the latest version number
*/
pub fn latest_version_number(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "latestVersionNumber", params)
}

/**
 * Returns 'true' if ZAP is on the latest version
*/
pub fn is_latest_version(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "isLatestVersion", params)
}

/**
 * Return a list of all of the installed add-ons
*/
pub fn installed_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "installedAddons", params)
}

/**
 * Returns a list with all local add-ons, installed or not.
*/
pub fn local_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "localAddons", params)
}

/**
 * Return a list of any add-ons that have been added to the Marketplace since the last check for updates
*/
pub fn new_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "newAddons", params)
}

/**
 * Return a list of any add-ons that have been changed in the Marketplace since the last check for updates
*/
pub fn updated_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "updatedAddons", params)
}

/**
 * Return a list of all of the add-ons on the ZAP Marketplace (this information is read once and then cached)
*/
pub fn marketplace_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "marketplaceAddons", params)
}

pub fn option_addon_directories(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "view",
        "optionAddonDirectories",
        params,
    )
}

pub fn option_day_last_checked(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "view",
        "optionDayLastChecked",
        params,
    )
}

pub fn option_day_last_install_warned(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "view",
        "optionDayLastInstallWarned",
        params,
    )
}

pub fn option_day_last_update_warned(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "view",
        "optionDayLastUpdateWarned",
        params,
    )
}

pub fn option_download_directory(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "view",
        "optionDownloadDirectory",
        params,
    )
}

pub fn option_check_addon_updates(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "view",
        "optionCheckAddonUpdates",
        params,
    )
}

pub fn option_check_on_start(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "autoupdate", "view", "optionCheckOnStart", params)
}

pub fn option_download_new_release(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "view",
        "optionDownloadNewRelease",
        params,
    )
}

pub fn option_install_addon_updates(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "view",
        "optionInstallAddonUpdates",
        params,
    )
}

pub fn option_install_scanner_rules(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "view",
        "optionInstallScannerRules",
        params,
    )
}

pub fn option_report_alpha_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "view",
        "optionReportAlphaAddons",
        params,
    )
}

pub fn option_report_beta_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "view",
        "optionReportBetaAddons",
        params,
    )
}

pub fn option_report_release_addons(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "view",
        "optionReportReleaseAddons",
        params,
    )
}

/**
 * Downloads the latest release, if any
*/
pub fn download_latest_release(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(
        service,
        "autoupdate",
        "action",
        "downloadLatestRelease",
        params,
    )
}

/**
 * Installs or updates the specified add-on, returning when complete (ie not asynchronously)
*/
pub fn install_addon(service: &ZapService, id: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    super::call(service, "autoupdate", "action", "installAddon", params)
}

pub fn install_local_addon(service: &ZapService, file: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("file".to_string(), file);
    super::call(service, "autoupdate", "action", "installLocalAddon", params)
}

/**
 * Uninstalls the specified add-on
*/
pub fn uninstall_addon(service: &ZapService, id: String) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), id);
    super::call(service, "autoupdate", "action", "uninstallAddon", params)
}

pub fn set_option_check_addon_updates(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "autoupdate",
        "action",
        "setOptionCheckAddonUpdates",
        params,
    )
}

pub fn set_option_check_on_start(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "autoupdate",
        "action",
        "setOptionCheckOnStart",
        params,
    )
}

pub fn set_option_download_new_release(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "autoupdate",
        "action",
        "setOptionDownloadNewRelease",
        params,
    )
}

pub fn set_option_install_addon_updates(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "autoupdate",
        "action",
        "setOptionInstallAddonUpdates",
        params,
    )
}

pub fn set_option_install_scanner_rules(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "autoupdate",
        "action",
        "setOptionInstallScannerRules",
        params,
    )
}

pub fn set_option_report_alpha_addons(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "autoupdate",
        "action",
        "setOptionReportAlphaAddons",
        params,
    )
}

pub fn set_option_report_beta_addons(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "autoupdate",
        "action",
        "setOptionReportBetaAddons",
        params,
    )
}

pub fn set_option_report_release_addons(
    service: &ZapService,
    boolean: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("Boolean".to_string(), boolean);
    super::call(
        service,
        "autoupdate",
        "action",
        "setOptionReportReleaseAddons",
        params,
    )
}
