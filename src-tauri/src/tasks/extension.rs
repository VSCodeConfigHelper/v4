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
use std::sync::Mutex;

use super::TaskArgs;

struct ExtensionManager {
  path: String,
  installed: Vec<String>
}

impl ExtensionManager {
  pub fn get(args: &TaskArgs) -> &Mutex<Self> {
    static INSTANCE: OnceCell<Mutex<ExtensionManager>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
      Mutex::new(ExtensionManager {
        path: args.vscode.clone(),
        installed: vec![]
      })
    })
  }

  fn install(&self, id: &str) -> Result<(), &'static str> {
    Err("not implemented")
  }

  fn uninstall(&self, id: &str) -> Result<(), &'static str> {
    Err("not implemented")
  }
}

static C_CPP_ID: &str = "ms-vscode.cpptools";
static CODE_LLDB_ID: &str = "vadimcn.vscode-lldb";

pub fn install_c_cpp(args: &TaskArgs) -> Result<(), &'static str> {
  let m = ExtensionManager::get(args).lock().unwrap();
  m.install(C_CPP_ID)
}

pub fn install_code_lldb(args: &TaskArgs) -> Result<(), &'static str> {
  let m = ExtensionManager::get(args).lock().unwrap();
  m.install(CODE_LLDB_ID)
}

pub fn remove_unrecommended(args: &TaskArgs) -> Result<(), &'static str> {
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
