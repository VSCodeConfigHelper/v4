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

use anyhow::{anyhow, Result};

use log::{debug, trace};
use super::TaskArgs;

#[cfg(target_os = "windows")]
const COUNT_API_URL: &str = "https://api.countapi.xyz/hit/v4.vscch.tk/windows";

#[cfg(target_os = "linux")]
const COUNT_API_URL: &str = "https://api.countapi.xyz/hit/v4.vscch.tk/linux";

#[cfg(target_os = "macos")]
const COUNT_API_URL: &str = "https://api.countapi.xyz/hit/v4.vscch.tk/macos";

pub fn send(_: &TaskArgs) -> Result<()> {
  debug!("发送到 Count API {} ...", COUNT_API_URL);
  let body = reqwest::blocking::get(COUNT_API_URL)?.text()?;
  trace!("body: {}", body);
  Ok(())
}

fn get_smtp_key() -> String {
  String::from(include_str!("./smtp_key.txt")).trim().into()
}

pub fn send_error() -> Result<()> {
  Err(anyhow!("Not implemented"))
}
