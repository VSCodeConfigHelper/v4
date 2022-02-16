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

use serde::Deserialize;
use std::path::Path;

use crate::steps::{compiler::Compiler, options::Options};

mod dotvscode;
mod extension;
mod run;
mod test;

#[derive(Deserialize)]
pub struct TaskArgs {
  pub vscode: String,
  pub compiler: Compiler,
  pub workspace: String,
  pub options: Options,
}

struct Task {
  name: &'static str,
  action: fn(&TaskArgs) -> Result<(), &'static str>,
  validator: fn(&TaskArgs) -> bool,
}

mod debug {
  pub use super::run::create_checker;
}

mod compiler {
  use super::TaskArgs;
  use crate::utils::winreg;
  use crate::steps::compiler::mingw::check_bin;

  pub fn add_to_path(args: &TaskArgs) -> Result<(), &'static str> {
    let compiler_path = check_bin(&args.compiler.path).unwrap();
    let compiler_path = compiler_path.as_str();
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
          .filter(|s| s != &compiler_path)
      )
      .collect::<Vec<&str>>()
      .join(";");

    if winreg::set_user_env("Path", &path) {
      Ok(())
    } else {
      Err("Failed to set user Path env")
    }
  }
}
mod shortcut {
  use super::TaskArgs;
  #[cfg(target_os = "windows")]
  use crate::utils::winapi::create_lnk;

  #[cfg(target_os = "windows")]
  pub fn create(args: &TaskArgs) -> Result<(), &'static str> {
    let path = dirs::desktop_dir().unwrap().join("Visual Studio Code.lnk");
    create_lnk(
      path.to_str().unwrap(),
      &args.vscode,
      &format!("Open VS Code at {}", args.workspace),
      &format!("\"{}\"", args.workspace),
    )
    .map_err(|_| "Failed to create shortcut")
  }

  #[cfg(not(target_os = "windows"))]
  pub fn create(_args: &TaskArgs) -> Result<(), &'static str> {
    Err("Not supported on this platform")
  }
}

mod vscode {
  use super::test::filepath;
  use super::TaskArgs;
  pub fn open(args: &TaskArgs) -> Result<(), &'static str> {
    let mut vscode_args = vec![args.workspace.as_str()];
    let test_filepath = filepath(args);
    if args.options.test != Some(false) {
      vscode_args.push("--goto");
      vscode_args.push(test_filepath.to_str().unwrap());
    }
    std::process::Command::new(&args.vscode)
      .args(vscode_args)
      .spawn()
      .map_err(|_| "Failed to open vscode")?;
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

pub fn list(args: &TaskArgs) -> Vec<(&'static str, fn(&TaskArgs) -> Result<(), &'static str>)> {
  generate_task![
    (extension::remove_unrecommended, a => a.options.remove_extensions),
    (extension::install_c_cpp, _ => true),
    (extension::install_code_lldb, a => llvm_setup(&a.compiler.setup)),
    (run::create_pauser, a => !a.options.compatible_mode),
    (run::create_keybinding, a => !a.options.compatible_mode),
    (debug::create_checker, a => a.options.ascii_check),
    (compiler::add_to_path, a => a.options.add_to_path),
    (dotvscode::tasks_json, _ => true),
    (dotvscode::launch_json, _ => true),
    (dotvscode::c_cpp_properties_json, _ => true),
    (test::generate, a => should_test(&a)),
    (shortcut::create, a => a.options.desktop_shortcut),
    (vscode::open, a => a.options.open_vscode)
  ]
  .iter()
  .filter(|t| (t.validator)(&args))
  .map(|t| (t.name, t.action))
  .collect()
}


fn llvm_setup(setup: &str) -> bool {
  ["llvm-mingw", "llvm", "apple"].contains(&setup)
}

fn should_test(args: &TaskArgs) -> bool {
  let test = args.options.test;
  if test.is_none() {
    let hello_word_filename = if args.options.language == "C" {
      "helloworld.c"
    } else {
      "helloworld.cpp"
    };
    let hello_world_path = Path::new(&args.workspace).join(hello_word_filename);
    !hello_world_path.exists()
  } else {
    test.unwrap()
  }
}
