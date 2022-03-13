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
use crate::tasks;
use crate::tasks::TaskInitArgs;

pub fn gui() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      vscode_verify,
      vscode_scan,
      compiler_setup_list,
      compiler_scan,
      compiler_verify,
      compiler_install,
      workspace_verify,
      options_scan,
      task_init
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum VerifyResult<T = ()> {
  Ok { value: T },
  Warn { message: &'static str },
  Err { message: &'static str },
}

#[tauri::command]
fn vscode_verify(path: String) -> VerifyResult {
  match vscode::verify(&path) {
    Ok(_) => VerifyResult::Ok { value: () },
    Err(e) => VerifyResult::Err { message: e },
  }
}

#[tauri::command]
fn vscode_scan() -> Option<String> {
  vscode::scan()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CompilerSetupListResult {
  id: &'static str,
  name: &'static str,
  description: &'static str,
  how_to_install: &'static str,
  is_mingw: bool,
  can_verify: bool,
  can_install: bool,
}

#[tauri::command]
fn compiler_setup_list() -> Vec<CompilerSetupListResult> {
  ENABLED_SETUPS
    .iter()
    .map(|s| CompilerSetupListResult {
      id: s.id,
      name: s.name,
      description: s.description,
      how_to_install: s.how_to_install,
      is_mingw: ["llvm-mingw", "gcc-mingw"].contains(&s.id),
      can_verify: (s.verify).is_some(),
      can_install: (s.install).is_some(),
    })
    .collect()
}

#[tauri::command]
fn compiler_scan(setup: String) -> Vec<Compiler> {
  (get_setup(&setup).scan)()
}

#[tauri::command]
fn compiler_verify(setup: String, path: String) -> VerifyResult<Compiler> {
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
fn compiler_install(setup: String) -> bool {
  if let Some(install) = get_setup(&setup).install {
    install()
  } else {
    false
  }
}

#[tauri::command]
fn workspace_verify(path: String) -> VerifyResult {
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
struct EnabledOptions {
  use_gnu_enabled: bool,
  pedantic_enabled: bool,
  acp_output_enabled: bool,
  ascii_check_enabled: bool,
  add_to_path_enabled: bool,
  desktop_shortcut_enabled: bool,
}

#[tauri::command]
fn options_scan(setup: &str) -> EnabledOptions {
  EnabledOptions {
    use_gnu_enabled: use_gnu_enabled(setup),
    pedantic_enabled: pedantic_enabled(setup),
    acp_output_enabled: acp_output_enabled(setup),
    ascii_check_enabled: ascii_check_enabled(setup),
    add_to_path_enabled: add_to_path_enabled(setup),
    desktop_shortcut_enabled: desktop_shortcut_enabled(setup),
  }
}

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
enum TaskFinishResult {
  Ok { name: &'static str },
  Err { name: &'static str, message: String },
}

#[tauri::command]
fn task_init(args: TaskInitArgs, window: tauri::Window) -> usize {
  let t = tasks::list(args);
  let len = t.len();
  std::thread::spawn(move || {
    for (name, action) in t {
      let res = action();
      let payload = match &res {
        Ok(_) => TaskFinishResult::Ok { name },
        Err(e) => TaskFinishResult::Err {
          name,
          message: e.backtrace().to_string() + e.to_string().as_str(),
        },
      };
      window.emit("task_finish", payload).unwrap();
      if res.is_err() {
        break;
      }
      // let dur = std::time::Duration::from_millis(100);
      // std::thread::sleep(dur);
    }
  });
  len
}
