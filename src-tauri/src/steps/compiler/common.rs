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

use std::{process::Command, path::Path};
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
use crate::utils::winapi::CREATE_NO_WINDOW;
use super::{Compiler, CompilerSetup};

pub fn test_compiler(path: &str, name: Option<&'static str>, setup: &'static CompilerSetup) -> Option<Compiler> {
  let mut compiler = Path::new(path).to_path_buf();
  if let Some(name) = name {
    compiler = compiler.join(name);
  }
  if !compiler.exists() {
    return None;
  }
  let mut cmd = Command::new(compiler);

  #[cfg(target_os = "windows")]
  {
    cmd.creation_flags(CREATE_NO_WINDOW);
  }

  let output = cmd.arg("--version")
    .output()
    .ok()?;
  let output = String::from_utf8(output.stdout).ok()?;
  let version_text = output.lines().nth(0)?;
  Compiler::new(setup, path, version_text)
}
