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
use derivative::*;
use log::{debug, info, trace, warn};
use serde::Deserialize;

use std::path::PathBuf;
use std::{path::Path, sync::Arc};

use crate::steps::compiler::{stdchoose, CompilerSetup, CompilerType};
use crate::steps::vscode::adjust_path as adjust_vscode;
use crate::steps::{compiler::Compiler, options::Options};
use crate::utils::ToString;

pub mod dotvscode;
pub mod extension;
pub mod run;
pub mod statistics;
pub mod test;

#[derive(Deserialize, Debug)]
pub struct TaskInitArgs {
  pub vscode: String,
  pub compiler: Compiler,
  pub workspace: String,
  pub options: Options,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct TaskArgs {
  pub vscode: PathBuf,
  #[derivative(Debug = "ignore")]
  pub setup: &'static CompilerSetup,
  pub compiler_path: PathBuf,
  pub workspace: PathBuf,
  pub run_hotkey: String,
  pub compatible_mode: bool,
  pub is_c: bool,
  pub file_ext: &'static str,
  pub standard: String,
  pub args: Vec<String>,
  pub ascii_check: bool,
  pub remove_extensions: bool,
  pub add_to_path: bool,
  pub open_vscode: bool,
  pub test_file: Option<String>,
  pub desktop_shortcut: bool,
  pub collect_data: bool,
}

struct Task {
  name: &'static str,
  action: fn(&TaskArgs) -> Result<()>,
  validator: fn(&TaskArgs) -> bool,
}

mod debug {
  pub use super::run::create_checker;
}

mod compiler {
  use super::*;
  #[cfg(windows)]
  use crate::utils::winreg;

  #[cfg(windows)]
  pub fn add_to_path(args: &TaskArgs) -> Result<()> {
    let compiler_path = args.compiler_path.parent().unwrap().to_str().unwrap();
    debug!("将编译器 {} 添加到用户 Path...", compiler_path);
    if winreg::get_machine_env("Path")
      .unwrap_or_default()
      .split(';')
      .collect::<Vec<&str>>()
      .contains(&compiler_path)
    {
      debug!("系统 Path 已包含 {}，不再添加。", compiler_path);
      return Ok(());
    }

    let path = std::iter::once(compiler_path)
      .chain(
        winreg::get_user_env("Path")
          .unwrap_or_default()
          .split(';')
          .filter(|s| s != &compiler_path),
      )
      .collect::<Vec<&str>>()
      .join(";");

    debug!("新的用户 Path：{}", path);
    winreg::set_user_env("Path", &path)
  }

  #[cfg(not(windows))]
  pub fn add_to_path(_args: &TaskArgs) -> Result<()> {
    Err(anyhow!("不支持在此操作系统上将编译器添加到 PATH。"))
  }
}
mod shortcut {
  use super::*;
  #[cfg(windows)]
  use crate::utils::winapi::create_lnk;

  #[cfg(windows)]
  pub fn create(args: &TaskArgs) -> Result<()> {
    let path = dirs::desktop_dir()
      .ok_or(anyhow!("找不到桌面路径。"))?
      .join("Visual Studio Code.lnk");
    if path.exists() {
      warn!("快捷方式 {:?} 已存在，将被覆盖。", path);
    }
    // Use exe instead of cmd, for showing vscode icon
    let vscode_exe = Some(&args.vscode)
      .and_then(|p| p.parent())
      .and_then(|p| p.parent())
      .unwrap()
      .join("Code.exe")
      .to_string();
    create_lnk(
      path.to_str().unwrap(),
      &vscode_exe,
      &format!("Open VS Code at {}", args.workspace.to_string()),
      &format!("\"{}\"", args.workspace.to_string()),
    )?;
    Ok(())
  }

  #[cfg(not(windows))]
  pub fn create(_args: &TaskArgs) -> Result<()> {
    Err(anyhow!("不支持在此操作系统上创建快捷方式。"))
  }
}

mod vscode {
  #[cfg(windows)]
  use std::os::windows::process::CommandExt;
  #[cfg(windows)]
  use crate::utils::winapi::CREATE_NO_WINDOW;

  use super::*;

  pub fn open(args: &TaskArgs) -> Result<()> {
    let mut vscode_args = vec![args.workspace.to_str().expect("to_str err")];
    if let Some(test_file) = &args.test_file {
      vscode_args.push("--goto");
      vscode_args.push(test_file.as_str());
    }
    trace!("Open command: {:?} {:?}", args.vscode, vscode_args);

    let mut cmd = std::process::Command::new(&args.vscode);
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd.args(vscode_args).spawn()?;
    Ok(())
  }
}

macro_rules! generate_task {
  ($( ($t:path, $vp:pat => $vb:expr) ),* ) => {
    vec![
      $( Task {
        name: stringify!($t),
        action: $t,
        validator: (|$vp: &TaskArgs| $vb)
      } ),*
    ]
  };
}

pub fn list(mut args: TaskInitArgs) -> Vec<(&'static str, Box<dyn Fn() -> Result<()> + Send>)> {
  let is_c = args.options.language == "C";
  let file_ext = if is_c { "c" } else { "cpp" };
  let vscode = adjust_vscode(Path::new(&args.vscode));
  let workspace = {
    let path = Path::new(&args.workspace);
    if path.is_absolute() {
      path.to_path_buf()
    } else {
      std::env::current_dir().unwrap().join(path)
    }
  };
  let test_file = {
    let mut path = workspace.join(format!("helloworld.{}", file_ext));
    if match args.options.test {
      Some(test) => test,
      None => !path.exists(),
    } {
      let mut i = 1;
      while path.exists() {
        path = workspace.join(format!("helloworld({}).{}", i, file_ext));
        i += 1;
      }
      Some(path.to_string())
    } else {
      None
    }
  };
  let setup = *args.compiler.setup;
  let standard = args.options.standard.as_ref().cloned().unwrap_or_else(|| {
    let std_p = match setup.ty {
      CompilerType::GCC => stdchoose::gcc,
      CompilerType::LLVM => stdchoose::clang,
      CompilerType::MSVC => (|_| ("c++20", "c17")) as fn(&str) -> (&'static str, &'static str),
    }(&args.compiler.version);
    (if is_c { std_p.1 } else { std_p.0 }).into()
  });
  {
    let std_arg_prefix = if setup.is_msvc() { "/std:" } else { "-std=" };
    if !args
      .options
      .args
      .iter()
      .any(|a| a.starts_with(std_arg_prefix))
    {
      let std_arg = format!("{}{}", std_arg_prefix, standard);
      info!("在编译选项中添加 {}。", std_arg);
      args.options.args.push(std_arg);
    }
  }
  statistics::set(args.options.collect_data);

  let args = Arc::from(TaskArgs {
    vscode,
    setup,
    compiler_path: (setup.path_to_exe)(&args.compiler.path, is_c),
    workspace,
    run_hotkey: args.options.run_hotkey,
    compatible_mode: args.options.compatible_mode,
    is_c: is_c,
    file_ext: file_ext,
    standard: standard.to_string(),
    args: args.options.args,
    ascii_check: args.options.ascii_check,
    remove_extensions: args.options.remove_extensions,
    add_to_path: args.options.add_to_path,
    open_vscode: args.options.open_vscode,
    test_file: test_file,
    desktop_shortcut: args.options.desktop_shortcut,
    collect_data: args.options.collect_data,
  });

  trace!("args passed to tasks: {:?}", args);

  let mapper = |task: Task| -> (&'static str, Box<dyn Fn() -> Result<()> + Send>) {
    let args = Arc::clone(&args);
    (task.name, Box::new(move || (task.action)(args.as_ref())))
  };

  generate_task![
    (extension::remove_unrecommended, a => a.remove_extensions),
    (extension::install_c_cpp, _ => true),
    (extension::install_code_lldb, a => a.setup.ty == CompilerType::LLVM),
    (extension::install_pauser, a => !a.compatible_mode),
    (run::create_keybinding, a => !a.compatible_mode),
    (debug::create_checker, a => a.ascii_check),
    (compiler::add_to_path, a => a.setup.is_mingw() && a.add_to_path),
    (dotvscode::create_folder, _ => true),
    (dotvscode::tasks_json, _ => true),
    (dotvscode::launch_json, _ => true),
    (dotvscode::c_cpp_properties_json, _ => true),
    (test::generate, a => a.test_file.is_some()),
    (shortcut::create, a => a.desktop_shortcut),
    (vscode::open, a => a.open_vscode),
    (statistics::send, a => a.collect_data)
  ]
  .into_iter()
  .filter(|t| (t.validator)(&args))
  .map(mapper)
  .collect()
}
