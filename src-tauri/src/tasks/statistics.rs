// Copyright (C) 2022 Guyutongxue
//
// This file is part of vscch4.
//
// vscch4 is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// vscch4 is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with vscch4.  If not, see <http://www.gnu.org/licenses/>.

use anyhow::{anyhow, Error, Result};
use log::{debug, error, trace, warn};
use once_cell::sync::Lazy;
use rand::distributions::Slice;
use rand::Rng;
use reqwest::header::CONTENT_TYPE;
use std::fs;
use std::sync::Mutex;

use crate::log::LOG_PATH;

use super::TaskArgs;

#[cfg(target_os = "windows")]
const COUNT_API_URL: &str = "https://api.countapi.xyz/hit/v4.vscch.tk/windows";

#[cfg(target_os = "linux")]
const COUNT_API_URL: &str = "https://api.countapi.xyz/hit/v4.vscch.tk/linux";

#[cfg(target_os = "macos")]
const COUNT_API_URL: &str = "https://api.countapi.xyz/hit/v4.vscch.tk/macos";

static ENABLED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));

pub fn set(value: bool) {
  *ENABLED.lock().unwrap() = value;
}

pub fn send(_: &TaskArgs) -> Result<()> {
  debug!("发送到 Count API {} ...", COUNT_API_URL);
  let body = reqwest::blocking::get(COUNT_API_URL)?.text()?;
  trace!("body: {}", body);
  Ok(())
}

/// 如果启用了日志发送，返回日志 ID
pub fn send_error(e: &Error) -> Option<String> {
  error!("错误：{}", e.to_string().as_str());
  error!("{}", e.backtrace().to_string());
  if !*ENABLED.lock().unwrap() {
    return None;
  }
  match (|| -> Result<String> {
    let id: String = rand::thread_rng()
      .sample_iter(&Slice::new(&[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
      ])?)
      .take(16)
      .collect();
    warn!("发送错误日志。ID：{}", id);
    let client = reqwest::blocking::Client::new();
    let _result: serde_json::Value = client
      .post(format!(
        "https://vscch4-report.herokuapp.com/errorLog/{}",
        id
      ))
      .header(CONTENT_TYPE, "text/plain")
      .body(fs::read_to_string(LOG_PATH.as_path())?)
      .send()?
      .json()?;
    // if let serde_json::Value::Object(m) = result && let Some((_, v)) = m.get_key_value("success") && v == &serde_json::Value::Bool(true) {
    Ok(id)
    // } else {
    //   Err(anyhow!("{}", result))
    // }
  })() {
    Err(e) => {
      error!("发送错误日志时出错：{:?}", e);
      None
    }
    Ok(id) => Some(id),
  }
}
