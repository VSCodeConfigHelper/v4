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

#![cfg(target_os = "windows")]

pub use winreg::enums::{
  HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
};
use winreg::{RegKey, enums::{KEY_READ, KEY_WRITE}};
use anyhow::Result;

use super::winapi::expand_environment_strings;

pub fn get(hkey: winreg::HKEY, path: &str, key: &str) -> Option<String> {
  let hkey = RegKey::predef(hkey);
  let path = match hkey.open_subkey(path) {
    Ok(key) => key,
    Err(_) => return None,
  };
  match path.get_value(key) {
    Ok(value) => Some(value),
    Err(_) => None,
  }
}

fn set(hkey: winreg::HKEY, path: &str, key: &str, value: &str) -> Result<()> {
  let hkey = RegKey::predef(hkey);
  let path = hkey.open_subkey_with_flags(path, KEY_READ | KEY_WRITE)?;
  path.set_value(key, &value)?;
  Ok(())
}

fn expand(path: &str) -> Option<String> {
  expand_environment_strings(path).ok()
}

pub fn get_user_env(key: &str) -> Option<String> {
  let value = get(HKEY_CURRENT_USER, "Environment", key)?;
  expand(&value)
}

pub fn get_machine_env(key: &str) -> Option<String> {
  let value = get(
    HKEY_LOCAL_MACHINE,
    "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
    key,
  )?;
  expand(&value)
}

pub fn set_user_env(key: &str, value: &str) -> Result<()> {
  set(HKEY_CURRENT_USER, "Environment", key, value)
}
