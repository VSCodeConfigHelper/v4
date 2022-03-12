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

use anyhow::Result;
use serde::Deserialize;

use std::path::PathBuf;
use std::{path::Path, sync::Arc};

use crate::steps::compiler::{get_setup, CompilerSetup};
use crate::steps::{compiler::Compiler, options::Options};
use crate::utils::ToString;

mod dotvscode;
mod extension;
mod run;
mod test;

#[derive(Deserialize)]
pub struct TaskInitArgs {
  pub vscode: String,
  pub compiler: Compiler,
  pub workspace: String,
  pub options: Options,
}

pub struct TaskArgs {
  pub vscode: String,
  pub compiler_setup: &'static CompilerSetup,
  pub compiler_path: PathBuf,
  pub workspace: String,
  pub compatible_mode: bool,
  pub is_c: bool,
  pub file_ext: &'static str,
  pub standard: Option<String>,
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
  use super::TaskArgs;
  #[cfg(target_os = "windows")]
  use crate::utils::winreg;
  use anyhow::Result;

  #[cfg(target_os = "windows")]
  pub fn add_to_path(args: &TaskArgs) -> Result<()> {
    let compiler_path = args.compiler_path.parent().unwrap().to_str().unwrap();
    if winreg::get_machine_env("Path")
      .unwrap_or_default()
      .split(';')
      .collect::<Vec<&str>>()
      .contains(&compiler_path)
    {
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

    winreg::set_user_env("Path", &path)
  }

  #[cfg(not(target_os = "windows"))]
  pub fn add_to_path(_args: &TaskArgs) -> Result<()> {
    panic!("Not available on this platform")
  }
}
mod shortcut {
  use super::TaskArgs;
  use anyhow::Result;
  #[cfg(target_os = "windows")]
  use crate::utils::winapi::create_lnk;

  #[cfg(target_os = "windows")]
  pub fn create(args: &TaskArgs) -> Result<()> {
    let path = dirs::desktop_dir().unwrap().join("Visual Studio Code.lnk");
    create_lnk(
      path.to_str().unwrap(),
      &args.vscode,
      &format!("Open VS Code at {}", args.workspace),
      &format!("\"{}\"", args.workspace),
    )?;
    Ok(())
  }

  #[cfg(not(target_os = "windows"))]
  pub fn create(_args: &TaskArgs) -> Result<()> {
    panic!("Not available on this platform")
  }
}

mod vscode {
  use super::TaskArgs;
  use anyhow::Result;

  pub fn open(args: &TaskArgs) -> Result<()> {
    let mut vscode_args = vec![args.workspace.as_str()];
    if let Some(test_file) = &args.test_file {
      vscode_args.push("--goto");
      vscode_args.push(test_file.as_str());
    }
    std::process::Command::new(&args.vscode)
      .args(vscode_args)
      .spawn()?;
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

pub fn list(args: TaskInitArgs) -> Vec<(&'static str, Box<dyn Fn() -> Result<()> + Send>)> {
  let is_c = args.options.language == "C";
  let file_ext = if is_c { "c" } else { "cpp" };
  let test_file = {
    let mut path = Path::new(&args.workspace).join(format!("helloworld.{}", file_ext));
    if match args.options.test {
      Some(test) => test,
      None => !path.exists(),
    } {
      let mut i = 1;
      while path.exists() {
        path = Path::new(&args.workspace).join(format!("helloworld({}).{}", i, file_ext));
        i += 1;
      }
      Some(path.to_string())
    } else {
      None
    }
  };
  let setup = get_setup(&args.compiler.setup);

  let args = Arc::from(TaskArgs {
    vscode: args.vscode,
    compiler_setup: setup,
    compiler_path: (setup.path_to_exe)(&args.compiler.path, is_c),
    workspace: args.workspace,
    compatible_mode: args.options.compatible_mode,
    is_c: is_c,
    file_ext: file_ext,
    standard: args.options.standard,
    args: args.options.args,
    ascii_check: args.options.ascii_check,
    remove_extensions: args.options.remove_extensions,
    add_to_path: args.options.add_to_path,
    open_vscode: args.options.open_vscode,
    test_file: test_file,
    desktop_shortcut: args.options.desktop_shortcut,
    collect_data: args.options.collect_data,
  });

  let mapper = |task: Task| -> (&'static str, Box<dyn Fn() -> Result<()> + Send>) {
    let args = Arc::clone(&args);
    (task.name, Box::new(move || (task.action)(args.as_ref())))
  };

  generate_task![
    (extension::remove_unrecommended, a => a.remove_extensions),
    (extension::install_c_cpp, _ => true),
    (extension::install_code_lldb, a => llvm_setup(&a.compiler_setup)),
    (run::create_pauser, a => !a.compatible_mode),
    (run::create_keybinding, a => !a.compatible_mode),
    (debug::create_checker, a => a.ascii_check),
    (compiler::add_to_path, a => a.add_to_path),
    (dotvscode::create_folder, _ => true),
    (dotvscode::tasks_json, _ => true),
    (dotvscode::launch_json, _ => true),
    (dotvscode::c_cpp_properties_json, _ => true),
    (test::generate, a => a.test_file.is_some()),
    (shortcut::create, a => a.desktop_shortcut),
    (vscode::open, a => a.open_vscode)
  ]
  .into_iter()
  .filter(|t| (t.validator)(&args))
  .map(mapper)
  .collect()
}

fn llvm_setup(setup: &CompilerSetup) -> bool {
  ["llvm-mingw", "llvm", "apple"].contains(&setup.id)
}

fn mingw_setup(setup: &CompilerSetup) -> bool {
  ["gcc-mingw", "llvm-mingw"].contains(&setup.id)
}
