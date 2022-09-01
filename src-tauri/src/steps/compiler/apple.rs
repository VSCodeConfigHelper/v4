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

#![cfg(target_os = "macos")]

use std::{path::Path, process::Command};
use anyhow::{anyhow, Result};
use crate::utils::ToString;
use super::llvm::path_to_exe;
use super::common::test_compiler;
use super::{CompilerSetup, Compiler, CompilerType};

fn scan() -> Vec<Compiler> {
  if Path::new("/Library/Developer/CommandLineTools").exists() {
    if let Ok(path) = which::which("clang++") {
      if let Some(compiler) = test_compiler(&path.to_string(), None, &SETUP) {
        return vec![compiler];
      }
    }
  }
  vec![]
}

fn install() -> Result<()> {
  let proc = Command::new("xcode-select").args(["--install"]).output()?;
  if proc.status.success() {
    Ok(())
  } else {
    Err(anyhow!("{}", String::from_utf8_lossy(&proc.stderr)))?
  }
}

pub static SETUP: CompilerSetup = CompilerSetup {
  id: super::Id::Apple,
  name: "Apple Clang",
  description: "Apple Clang compiler with Xcode",
  how_to_install: r"安装 Xcode 命令行工具。请按照弹窗中的说明完成安装。",

  scan: scan,
  verify: None,
  install: Some(install),

  ty: CompilerType::LLVM,
  path_to_exe: path_to_exe,
};
