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
#[cfg(windows)]
use crate::utils::winapi::get_acp;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Options {
  pub compatible_mode: bool,
  #[serde(rename = "activeLanguage")]
  pub language: String,
  #[serde(rename = "activeStandard")]
  pub standard: Option<String>,
  pub args: Vec<String>,
  pub ascii_check: bool,
  pub remove_extensions: bool,
  pub add_to_path: bool,
  pub open_vscode: bool,
  pub test: Option<bool>,
  pub desktop_shortcut: bool,
  pub collect_data: bool,
}

pub fn use_gnu_enabled(setup: &str) -> bool {
  ["gcc-mingw", "gcc"].contains(&setup)
}

pub fn pedantic_enabled(setup: &str) -> bool {
  setup != "msvc"
}

#[cfg(windows)]
pub fn acp_output_enabled(setup: &str) -> bool {
  ["gcc-mingw", "msvc"].contains(&setup) && get_acp() == 936
}

#[cfg(not(windows))]
pub fn acp_output_enabled(_setup: &str) -> bool {
  false
}

pub fn ascii_check_enabled(setup: &str) -> bool {
  setup == "gcc-mingw"
}

pub fn add_to_path_enabled(setup: &str) -> bool {
  ["gcc-mingw", "llvm-mingw"].contains(&setup)
}

pub fn desktop_shortcut_enabled(_setup: &str) -> bool {
  cfg!(windows)
}
