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

use std::collections::HashSet;
use std::path::Path;
use std::process::Command;

use super::super::reg;
use super::verparse;
use super::Compiler;
use super::CompilerSetup;

/// 给定 `{path}`, 按需构造 `{path}\\bin`。
/// 检查其存在且是目录后返回。
fn check_bin(path: &str) -> Option<String> {
  let mut path = Path::new(path).to_path_buf();
  if !path.ends_with("bin") {
    path = path.join("bin");
  }
  if path.is_dir() {
    path.into_os_string().into_string().ok()
  } else {
    None
  }
}

/// Get user & machine `Path` environment variables.
fn get_paths() -> HashSet<String> {
  let user_path = reg::get_user_env("Path").unwrap_or_default();
  let machine_path = reg::get_machine_env("Path").unwrap_or_default();
  let all_path = user_path + ";" + &machine_path;

  // FIXME: deal with paths contain quotes
  all_path.split(';').map(|p| p.to_string()).collect()
}

fn test_compiler(path: &str, name: &'static str) -> Option<Compiler> {
  let compiler = Path::new(path).join(name);
  if !compiler.exists() {
    return None;
  }
  let output = Command::new(compiler).arg("--version").output().ok()?;
  let output = String::from_utf8(output.stdout).ok()?;
  let version_text = output.lines().nth(0)?;
  Some(Compiler::new(&GCC_SETUP, path, version_text))
}

fn scan() -> Vec<Compiler> {
  get_paths().iter().filter_map(|p| verify(p).ok()).collect()
}

fn verify(path: &str) -> Result<Compiler, &'static str> {
  if path.contains(';') {
    return Err("路径中不能含有分号 ';'");
  }
  if path.chars().any(|c| !c.is_ascii()) {
    return Err("路径应为 ASCII，即不能包含中文或特殊字符等");
  }
  if !Path::new(path).exists() {
    return Err("路径不存在");
  }
  check_bin(path)
    .ok_or("bin 不存在")
    .and_then(|p| test_compiler(&p, "g++.exe").ok_or("g++.exe 不是编译器"))
}

fn install() -> bool {
  open::that("https://gytx.lanzoui.com/iy906s48llc").is_ok()
}

pub static GCC_SETUP: CompilerSetup = CompilerSetup {
  id: "gcc-mingw",
  name: "MinGW",
  description: "GCC for Windows",
  how_to_install: r"下载 MinGW。下载并解压后，您将得到一个 <code>mingw64</code> 文件夹。建议您将它妥善保存在合适的位置（如 <code>C:\mingw64</code>），并在下方输入其路径。",

  scan: scan,
  verify: Some(verify),
  install: Some(install),

  verparser: verparse::gcc,
};
