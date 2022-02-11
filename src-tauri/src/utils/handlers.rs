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

use serde::Serialize;

use super::vscode;
use super::compiler_setup::ENABLED_SETUPS;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum VscodeVerifyResult {
  Ok,
  Err { message: &'static str },
}

#[tauri::command]
pub fn vscode_verify(path: String) -> VscodeVerifyResult {
  match vscode::verify(&path) {
    Ok(_) => VscodeVerifyResult::Ok,
    Err(e) => VscodeVerifyResult::Err { message: e },
  }  
}

#[tauri::command]
pub fn vscode_scan() -> Option<String> {
  vscode::scan()
}

#[derive(Serialize)]
pub struct CompilerSetupListResult {
  name: &'static str,
  description: &'static str
}

#[tauri::command]
pub fn compiler_setup_list() -> Vec<CompilerSetupListResult> {
  ENABLED_SETUPS.iter().map(|s| {
    CompilerSetupListResult {
      name: s.name,
      description: s.description
    }
  }).collect()
}
