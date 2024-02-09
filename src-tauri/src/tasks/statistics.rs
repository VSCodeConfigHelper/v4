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
use reqwest::header::CONTENT_TYPE;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

use crate::log::get_log_path;

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
  fn do_send() -> Result<()> {
    debug!("发送到 Count API {} ...", COUNT_API_URL);
    let body = reqwest::blocking::get(COUNT_API_URL)?.text()?;
    trace!("body: {}", body);
    Ok(())
  }
  if let Err(e) = do_send() {
    warn!("发送到 Count API 时出错：{}", e);
  }
  Ok(())
}

/// 如果启用了日志发送，返回标识码
pub fn send_error(e: &Error) -> Option<u64> {
  error!("错误：{:?}", e);
  if !*ENABLED.lock().unwrap() {
    return None;
  }
  match (|| -> Result<u64> {
    let mut id = hardware_id::get_id()
      .map(|id| {
        debug!("本机硬件 ID {}", id);
        let mut s = DefaultHasher::new();
        id.hash(&mut s);
        s.finish()
      })
      .unwrap_or_else(|_| rand::random());
    id %= 1_000_000;
    warn!("发送错误日志。标识码 {}", id);
    let res = reqwest::blocking::Client::new()
      .post(format!("https://api.guyutongxue.site/vscch/errorLog/{}", id))
      .header(CONTENT_TYPE, "text/plain")
      .body(fs::read_to_string(get_log_path())?)
      .send()?;
    if res.status().is_success() {
      Ok(id)
    } else {
      Err(anyhow!(res.text()?))
    }
  })() {
    Err(e) => {
      error!("发送错误日志时出错：{:?}", e);
      None
    }
    Ok(id) => Some(id),
  }
}
