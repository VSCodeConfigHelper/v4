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

#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::{path::Path, process::Command};

use log::debug;

use super::{Compiler, CompilerSetup};
#[cfg(windows)]
use crate::utils::winapi::{ansi_buffer_to_string, CREATE_NO_WINDOW};

pub fn test_compiler(
  path: &str,
  name: Option<&'static str>,
  setup: &'static CompilerSetup,
) -> Option<Compiler> {
  let mut compiler = Path::new(path).to_path_buf();
  if let Some(name) = name {
    compiler = compiler.join(name);
  }
  if !compiler.exists() {
    return None;
  }
  debug!("测试编译器: {:?}（类型 {}）", &compiler, &setup.id);

  let mut cmd = Command::new(compiler);

  #[cfg(windows)]
  cmd.creation_flags(CREATE_NO_WINDOW);

  let output = cmd.arg("--version").output().ok()?;
  let output = match String::from_utf8(output.stdout) {
    Ok(str) => str,
    Err(e) => {
      debug!("编译器返回非 UTF-8 输出");
      #[cfg(not(windows))]
      return None;
      #[cfg(windows)]
      ansi_buffer_to_string(&e.into_bytes()).ok()?
    }
  };
  debug!("编译器版本：{}", output);
  let version_text = output.lines().nth(0)?;
  Compiler::new(setup, path, version_text)
}
