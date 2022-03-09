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

#![windows_subsystem = "windows"]

mod handlers;
mod steps;
mod tasks;
mod utils;

use handlers::*;

fn main() {
  std::env::set_var("RUST_BACKTRACE", "1");
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
