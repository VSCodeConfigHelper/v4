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
use log::{debug, info, trace};
use serde::Serialize;

use crate::steps::{
  compiler::get_setup, compiler::Compiler, compiler::ENABLED_SETUPS, options::*, vscode, workspace,
};
use crate::tasks;
use crate::tasks::TaskInitArgs;

pub fn gui() -> Result<()> {
  debug!("即将启动 tauri GUI。");
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
    .map_err(|t| anyhow!(t.to_string()))?;
  Ok(())
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
enum VerifyResult<T = ()> {
  Ok { value: T },
  Warn { message: &'static str },
  Err { message: &'static str },
}

#[tauri::command]
fn vscode_verify(path: String) -> VerifyResult {
  trace!("vscode_verify: <- {}", path);
  let result = match vscode::verify(&path) {
    Ok(_) => VerifyResult::Ok { value: () },
    Err(e) => VerifyResult::Err { message: e },
  };
  trace!("vscode_verify: -> {:?}", result);
  result
}

#[tauri::command]
fn vscode_scan() -> Option<String> {
  trace!("vscode_scan: <- ()");
  let result = vscode::scan();
  trace!("vscode_scan: -> {:?}", result);
  result
}

#[derive(Serialize, Debug)]
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
  trace!("compiler_setup_list: <- ()");
  let result: Vec<_> = ENABLED_SETUPS
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
    .collect();
  trace!("compiler_setup_list: -> {:?}", result);
  result
}

#[tauri::command]
fn compiler_scan(setup: String) -> Vec<Compiler> {
  trace!("compiler_scan: <- {}", setup);
  let result = (get_setup(&setup).scan)();
  trace!("compiler_scan: -> {:?}", result);
  result
}

#[tauri::command]
fn compiler_verify(setup: String, path: String) -> VerifyResult<Compiler> {
  trace!("compiler_verify: <- {} {}", setup, path);
  let result = if let Some(verify) = get_setup(&setup).verify {
    match verify(&path) {
      Ok(compiler) => VerifyResult::Ok { value: compiler },
      Err(e) => VerifyResult::Err { message: e },
    }
  } else {
    VerifyResult::Err {
      message: "不可以验证该编译器。",
    }
  };
  trace!("compiler_verify: -> {:?}", result);
  result
}

#[tauri::command]
fn compiler_install(setup: String) -> bool {
  trace!("compiler_install: <- {}", setup);
  let result = if let Some(install) = get_setup(&setup).install {
    install()
  } else {
    false
  };
  trace!("compiler_install: -> {}", result);
  result
}

#[tauri::command]
fn workspace_verify(path: String) -> VerifyResult {
  trace!("workspace_verify: <- {}", path);
  let result = if let Err(msg) = workspace::path_available(&path) {
    VerifyResult::Err { message: msg }
  } else if workspace::exists(&path) {
    VerifyResult::Warn {
      message: "此工作文件夹下已有配置。若继续则原有配置会被覆盖。",
    }
  } else if path.chars().any(|c| c == '&' || c == ' ') {
    VerifyResult::Warn {
      message: "包含字符 '&' 或空格的路径可能导致问题。建议重命名或更换路径。",
    }
  } else {
    VerifyResult::Ok { value: () }
  };
  trace!("workspace_verify: -> {:?}", result);
  result
}

#[derive(Serialize, Debug)]
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
  trace!("options_scan: <- {}", setup);
  let result = EnabledOptions {
    use_gnu_enabled: use_gnu_enabled(setup),
    pedantic_enabled: pedantic_enabled(setup),
    acp_output_enabled: acp_output_enabled(setup),
    ascii_check_enabled: ascii_check_enabled(setup),
    add_to_path_enabled: add_to_path_enabled(setup),
    desktop_shortcut_enabled: desktop_shortcut_enabled(setup),
  };
  trace!("options_scan: -> {:?}", result);
  result
}

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type")]
enum TaskFinishResult {
  Ok { name: &'static str },
  Err { name: &'static str, message: String },
}

#[tauri::command]
fn task_init(args: TaskInitArgs, window: tauri::Window) -> Vec<&'static str> {
  trace!("task_init: <- {:?}", args);
  let t = tasks::list(args);
  let names = t.iter().map(|t| t.0).collect::<Vec<_>>();
  std::thread::spawn(move || {
    for (name, action) in t {
      info!("正在执行任务 {}...", name);
      let res = action();
      info!("任务 {} 执行完毕。", name);
      let payload = match &res {
        Ok(_) => TaskFinishResult::Ok { name },
        Err(e) => TaskFinishResult::Err {
          name,
          message: e.backtrace().to_string() + e.to_string().as_str(),
        },
      };
      trace!("task_finish: -> {:?}", payload);
      window.emit("task_finish", payload).unwrap();
      if res.is_err() {
        break;
      }
    }
  });
  trace!("task_init: -> {:?}", names);
  names
}
