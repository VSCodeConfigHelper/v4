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

#![cfg(target_os = "windows")]

use super::verparse;
use super::Compiler;
use super::CompilerSetup;

fn scan() -> Vec<Compiler> {
  vec![]
}

fn install() -> bool {
  open::that("https://aka.ms/vs/17/release/vs_BuildTools.exe").is_ok()
}

pub static SETUP: CompilerSetup = CompilerSetup {
  id: "msvc",
  name: "VC++ 生成工具",
  description: "Microsoft Visual C++",
  how_to_install: r"下载。运行安装器，按照提示完成安装。",

  scan: scan,
  verify: None,
  install: Some(install),
  verparser: verparse::gcc
};
