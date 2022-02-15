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

use std::path::Path;

use crate::steps::{compiler::Compiler, options::Options};

mod extension;
mod test;

pub struct TaskArgs {
  pub vscode: String,
  pub compiler: Compiler,
  pub workspace: String,
  pub options: Options,
}

type Task = fn(args: &TaskArgs) -> Result<(), &'static str>;

fn llvm_setup(setup: &str) -> bool {
  ["llvm-mingw", "llvm", "apple"].contains(&setup)
}

pub fn find_tasks(args: &TaskArgs) -> Vec<Task> {
  let mut tasks: Vec<Task> = vec![];
  if args.options.remove_extensions {
    tasks.push(extension::remove_unrecommended);
  }
  tasks.push(extension::install_c_cpp);
  if llvm_setup(&args.compiler.setup) {
    tasks.push(extension::install_code_lldb);
  }
  if !args.options.compatible_mode {
    // save console pauser
    // add f6 keybinding
  }
  if args.options.ascii_check {
    // add ascii check
  }
  if args.options.add_to_path {
    // add to path
  }
  // tasks.json
  // launch.json
  // c_cpp_properties.json
  let mut test = args.options.test;
  if test.is_none() {
    let hello_word_filename = if args.options.language == "C" {
      "helloworld.c"
    } else {
      "helloworld.cpp"
    };
    let hello_world_path = Path::new(&args.workspace).join(hello_word_filename);
    test = Some(!hello_world_path.exists())
  }
  if test.unwrap() {
    tasks.push(test::generate)
  }
  if args.options.desktop_shortcut {
    // generate shortcut
  }
  if args.options.open_vscode {
    // open vscode
  }
  tasks
}
