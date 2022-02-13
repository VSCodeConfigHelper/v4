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

use std::path::Path;

use super::reg;

#[cfg(target_os = "windows")]
pub fn scan() -> Option<String> {
  let cmd = reg::get(reg::HKEY_CLASSES_ROOT, "vscode\\shell\\open\\command", "")?;
  // The value should be like:
  // "C:\Program Files\Microsoft VS Code\Code.exe" --open-url -- "%1"
  // and we just use the string inside the first quotation marks
  let parts = cmd.split("\"").nth(1)?;
  match verify(parts) {
    Ok(_) => Some(parts.to_string()),
    Err(_) => None,
  }
}

#[cfg(target_os = "windows")]
pub fn verify(path: &str) -> Result<(), &'static str> {
  let path = Path::new(path);
  if !path.exists() {
    return Err("路径不存在");
  }
  let vscode_folder = if path.is_dir() {
    path
  } else {
    path.parent().unwrap()
  };
  if vscode_folder.join("bin").join("code.cmd").exists() {
    Ok(())
  } else {
    Err("找不到 code.cmd")
  }
}
