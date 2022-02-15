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

use crate::steps::{
  compiler::get_setup, compiler::Compiler, compiler::ENABLED_SETUPS, options::*, vscode, workspace,
};
use crate::tasks::{find_tasks, TaskArgs};

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum VerifyResult<T = ()> {
  Ok { value: T },
  Warn { message: &'static str },
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
#[serde(rename_all = "camelCase")]
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
pub fn compiler_scan(setup: String) -> Vec<Compiler> {
  (get_setup(&setup).scan)()
}

#[tauri::command]
pub fn compiler_verify(setup: String, path: String) -> VerifyResult<Compiler> {
  if let Some(verify) = get_setup(&setup).verify {
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
pub fn compiler_install(setup: String) -> bool {
  if let Some(install) = get_setup(&setup).install {
    install()
  } else {
    false
  }
}

#[tauri::command]
pub fn workspace_verify(path: String) -> VerifyResult {
  if let Err(msg) = workspace::path_available(&path) {
    return VerifyResult::Err { message: msg };
  }
  if workspace::exists(&path) {
    return VerifyResult::Warn {
      message: "此工作文件夹下已有配置。若继续则原有配置会被覆盖。",
    };
  }
  VerifyResult::Ok { value: () }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnabledOptions {
  use_gnu_enabled: bool,
  pedantic_enabled: bool,
  acp_output_enabled: bool,
  ascii_check_enabled: bool,
  add_to_path_enabled: bool,
  desktop_shortcut_enabled: bool,
}

#[tauri::command]
pub fn options_scan(setup: &str) -> EnabledOptions {
  EnabledOptions {
    use_gnu_enabled: use_gnu_enabled(setup),
    pedantic_enabled: pedantic_enabled(setup),
    acp_output_enabled: acp_output_enabled(setup),
    ascii_check_enabled: ascii_check_enabled(setup),
    add_to_path_enabled: add_to_path_enabled(setup),
    desktop_shortcut_enabled: desktop_shortcut_enabled(setup),
  }
}

#[tauri::command]
pub fn task_init(vscode: String, compiler: Compiler, workspace: String, options: Options) -> usize {
  let tasks = find_tasks(&TaskArgs {
    vscode,
    compiler,
    workspace,
    options,
  });
  tasks.len()
}
