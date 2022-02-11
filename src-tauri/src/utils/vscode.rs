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

use std::io;
use std::path::Path;

#[cfg(target_os = "windows")]
use winreg::RegKey;

#[cfg(target_os = "windows")]
pub fn scan() -> Option<String> {
  let cmd = (|| -> io::Result<String> {
    let hkcr = RegKey::predef(winreg::enums::HKEY_CLASSES_ROOT);
    let key = hkcr.open_subkey("vscode\\shell\\open\\command")?;
    let val: String = key.get_value("")?;
    Ok(val)
  })();
  if cmd.is_err() {
    return None;
  }
  let cmd = cmd.unwrap();
  // The value should be like:
  // "C:\Program Files\Microsoft VS Code\Code.exe" --open-url -- "%1"
  // and we just use the string inside the first quotation marks
  let parts = cmd.split("\"").nth(1);
  if parts.is_none() {
    return None;
  }
  let parts = parts.unwrap();
  match verify(parts) {
    Ok(_) => Some(parts.to_string()),
    Err(_) => None,
  }
}

#[cfg(target_os = "windows")]
pub fn verify(path: &str) -> Result<(), &'static str> {
  let path = Path::new(path);
  if !path.exists() {
    return Err("File not exist");
  }
  let vscode_folder = if path.is_dir() {
    Some(path)
  } else {
    path.parent()
  };
  if let Some(vscode_folder) = vscode_folder {
    let script_path = vscode_folder.join("bin").join("code.cmd");
    if script_path.exists() {
      return Ok(());
    } else {
      return Err("Could not find code.cmd");
    }
  } else {
    return Err("Should not be root");
  }
}
