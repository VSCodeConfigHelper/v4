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

use once_cell::sync::OnceCell;
use anyhow::Result;

use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use crate::steps::vscode::adjust_path;
#[cfg(target_os = "windows")]
use crate::utils::winapi::CREATE_NO_WINDOW;

use super::TaskArgs;

struct ExtensionManager {
  path: PathBuf,
  installed: Vec<String>,
}

impl ExtensionManager {
  pub fn get(args: &TaskArgs) -> &Mutex<Self> {
    static INSTANCE: OnceCell<Mutex<ExtensionManager>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
      let path = adjust_path(Path::new(&args.vscode));
      let mut instance = ExtensionManager {
        path: path,
        installed: vec![],
      };
      let _ = instance.update();
      Mutex::new(instance)
    })
  }

  fn run(&self, args: &[&str]) -> Result<String> {
    let mut command = Command::new(&self.path);
    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);
    
    let stdout = command
      .args(args)
      .output()?
      .stdout;
    let stdout = String::from_utf8(stdout)?;
    Ok(stdout)
  }

  fn update(&mut self) -> Result<()> {
    let output = self.run(&["--list-extensions"])?;
    self.installed = output.lines().map(|line| line.to_string()).collect();
    Ok(())
  }

  fn install(&self, id: &str) -> Result<()> {
    if self.installed.contains(&id.to_string()) {
      return Ok(());
    }
    self.run(&["--install-extension", id])?;
    Ok(())
  }

  fn uninstall(&self, id: &str) -> Result<()> {
    if !self.installed.contains(&id.to_string()) {
      return Ok(());
    }
    self.run(&["--uninstall-extension", id])?;
    Ok(())
  }
}

static C_CPP_ID: &str = "ms-vscode.cpptools";
static CODE_LLDB_ID: &str = "vadimcn.vscode-lldb";

pub fn install_c_cpp(args: &TaskArgs) -> Result<()> {
  let m = ExtensionManager::get(args).lock().unwrap();
  m.install(C_CPP_ID)
}

pub fn install_code_lldb(args: &TaskArgs) -> Result<()> {
  let m = ExtensionManager::get(args).lock().unwrap();
  m.install(CODE_LLDB_ID)
}

pub fn remove_unrecommended(args: &TaskArgs) -> Result<()> {
  let m = ExtensionManager::get(args).lock().unwrap();
  [
    "formulahendry.code-runner",
    "austin.code-gnu-global",
    "danielpinto8zz6.c-cpp-compile-run",
    "mitaki28.vscode-clang",
    "jaycetyle.vscode-gnu-global",
    "franneck94.c-cpp-runner",
    "ajshort.include-autocomplete",
    "xaver.clang-format",
    "jbenden.c-cpp-flylint",
  ]
  .iter()
  .map(|id| m.uninstall(id))
  .collect()
}
