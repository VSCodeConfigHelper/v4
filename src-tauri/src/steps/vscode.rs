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

#![allow(unused_imports)]

use log::debug;

use std::path::{Path,PathBuf};
use crate::utils::ToString;

#[cfg(windows)]
use crate::utils::winreg;

#[cfg(target_os = "windows")]
pub fn scan() -> Option<String> {
  let cmd = winreg::get(winreg::HKEY_CLASSES_ROOT, "vscode\\shell\\open\\command", "")?;
  debug!("vscode:// 的注册表项：{}", &cmd);
  // The value should be like:
  // "C:\Program Files\Microsoft VS Code\Code.exe" --open-url -- "%1"
  // and we just use the string inside the first quotation marks
  let parts = cmd.split("\"").nth(1)?;
  match verify(parts) {
    Ok(_) => Some(parts.to_string()),
    Err(_) => None,
  }
}

#[cfg(target_os = "linux")]
pub fn scan() -> Option<String> {
  match which::which("code") {
    Ok(path) => {
      let path = path.to_string();
      match verify(&path) {
        Ok(_) => Some(path),
        Err(_) => None,
      }
    },
    Err(_) => None,
  }
}

#[cfg(target_os = "macos")]
pub fn scan() -> Option<String> {
  if let Ok(path) = which::which("code") {
    let path = path.to_string();
    if verify(&path).is_ok() {
      return Some(path);
    }
  }
  let common_installation = "/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code";
  if Path::new(common_installation).exists() {
    Some(common_installation.to_string())
  } else {
    None
  }
}

pub fn adjust_path(path: &Path) -> PathBuf {
  if cfg!(windows) {
    let folder = path.parent().unwrap();
    folder.join("bin").join("code.cmd")
  } else {
    path.to_path_buf()
  }
}

pub fn verify(path: &str) -> Result<(), &'static str> {
  let path = Path::new(path);
  if !path.is_file() {
    return Err("路径不存在");
  }
  if adjust_path(path).exists() {
    Ok(())
  } else {
    Err("找不到 code.cmd")
  }
}
