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

use super::compiler::Compiler;
use super::compiler_setup::ENABLED_SETUPS;
use super::vscode;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum VerifyResult<T = ()> {
  Ok { value: T },
  Err { message: &'static str },
}

#[tauri::command]
pub fn vscode_verify(path: String) -> VerifyResult {
  match vscode::verify(&path) {
    Ok(_) => VerifyResult::Ok { value: () },
    Err(e) => VerifyResult::Err { message: e },
  }
}

#[tauri::command]
pub fn vscode_scan() -> Option<String> {
  vscode::scan()
}

#[derive(Serialize)]
pub struct CompilerSetupListResult {
  id: &'static str,
  name: &'static str,
  description: &'static str,
  how_to_install: &'static str,
  can_verify: bool,
  can_install: bool,
}

#[tauri::command]
pub fn compiler_setup_list() -> Vec<CompilerSetupListResult> {
  ENABLED_SETUPS
    .iter()
    .map(|s| CompilerSetupListResult {
      id: s.id,
      name: s.name,
      description: s.description,
      how_to_install: s.how_to_install,
      can_verify: (s.verify).is_some(),
      can_install: (s.install).is_some(),
    })
    .collect()
}

#[tauri::command]
pub fn compiler_scan(setup_no: usize) -> Vec<Compiler> {
  let setup = ENABLED_SETUPS[setup_no];
  (setup.scan)()
}

#[tauri::command]
pub fn compiler_verify(setup_no: usize, path: String) -> VerifyResult<Compiler> {
  let setup = ENABLED_SETUPS[setup_no];
  if let Some(verify) = setup.verify {
    match verify(&path) {
      Ok(compiler) => VerifyResult::Ok { value: compiler },
      Err(e) => VerifyResult::Err { message: e },
    }
  } else {
    VerifyResult::Err {
      message: "Not implemented",
    }
  }
}

#[tauri::command]
pub fn compiler_install(setup_no: usize) -> bool {
  let setup = ENABLED_SETUPS[setup_no];
  if let Some(install) = setup.install {
    install()
  } else {
    false
  }
}
