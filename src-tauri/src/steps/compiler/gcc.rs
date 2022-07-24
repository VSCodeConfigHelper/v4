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

#![cfg(not(windows))]

use std::path::{Path, PathBuf};

use super::common::test_compiler;
use super::{verparse, Compiler, CompilerSetup};
use crate::utils::ToString;

fn scan() -> Vec<Compiler> {
  match which::which("g++") {
    Ok(path) => match verify(&path.to_string()) {
      Ok(compiler) => vec![compiler],
      Err(_) => vec![],
    },
    Err(_) => vec![],
  }
}

fn verify(path: &str) -> Result<Compiler, &'static str> {
  let path = which::which(path).map_err(|_| "找不到文件")?;
  let compiler = test_compiler(path.to_str().unwrap(), None, &SETUP).ok_or("无法解析编译器版本")?;
  Ok(compiler)
}

fn path_to_exe(path: &str, is_c: bool) -> PathBuf {
  let path = Path::new(path);
  let basename = path.file_name().unwrap().to_str().unwrap();
  let basename = if is_c {
    basename.replace("g++", "gcc")
  } else {
    basename.replace("gcc", "g++")
  };
  path.parent().unwrap().join(basename)
}

pub static ID: &'static str = "gcc";

pub static SETUP: CompilerSetup = CompilerSetup {
  id: ID,
  name: "GCC",
  description: "GNU Compiler Collection",
  how_to_install: r"使用包管理器下载 GCC。比如，键入命令 <code>sudo apt install g++</code>。",

  scan: scan,
  verify: Some(verify),
  install: None,

  verparser: verparse::gcc,
  path_to_exe: path_to_exe,
};
