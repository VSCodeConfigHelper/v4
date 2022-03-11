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
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use super::verparse;
use super::Compiler;
use super::CompilerSetup;
use crate::utils::winapi::CREATE_NO_WINDOW;
use crate::utils::winreg;
use crate::utils::ToString;

/// 给定 `{path}`, 按需构造 `{path}\\bin`。
/// 检查其存在且是目录后返回。
pub fn check_bin(path: &str) -> Option<String> {
  let mut path = Path::new(path).to_path_buf();
  if !path.ends_with("bin") {
    path = path.join("bin");
  }
  if path.is_dir() {
    Some(path.to_string())
  } else {
    None
  }
}

/// Get user & machine `Path` environment variables.
fn get_paths() -> HashSet<String> {
  let user_path = winreg::get_user_env("Path").unwrap_or_default();
  let machine_path = winreg::get_machine_env("Path").unwrap_or_default();
  let all_path = user_path + ";" + &machine_path;

  // FIXME: deal with paths contain quotes
  all_path.split(';').map(|p| p.to_string()).collect()
}

fn test_compiler(path: &str, name: &'static str, setup: &'static CompilerSetup) -> Option<Compiler> {
  let compiler = Path::new(path).join(name);
  if !compiler.exists() {
    return None;
  }
  let output = Command::new(compiler)
    .creation_flags(CREATE_NO_WINDOW)
    .arg("--version")
    .output()
    .ok()?;
  let output = String::from_utf8(output.stdout).ok()?;
  let version_text = output.lines().nth(0)?;
  Compiler::new(setup, path, version_text)
}

fn scan_gcc() -> Vec<Compiler> {
  get_paths().iter().filter_map(|p| verify(p, "g++.exe", &GCC_SETUP).ok()).collect()
}

fn scan_clang() -> Vec<Compiler> {
  get_paths().iter().filter_map(|p| verify(p, "clang++.exe", &LLVM_SETUP).ok()).collect()
}

fn verify(path: &str, name: &'static str, setup: &'static CompilerSetup) -> Result<Compiler, &'static str> {
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
    .and_then(|p| test_compiler(&p, name, setup).ok_or(".exe 不是编译器"))
}

fn install_gcc() -> bool {
  open::that("https://gytx.lanzoui.com/iy906s48llc").is_ok()
}

fn install_clang() -> bool {
  open::that("https://github.com/mstorsjo/llvm-mingw/releases").is_ok()
}

fn join(path: &str, name: &str) -> PathBuf {
  Path::new(path).join(name)
}

fn path_to_gcc(path: &str, is_c: bool) -> PathBuf {
  let path = check_bin(path).unwrap();
  if is_c {
    join(&path, "gcc.exe")
  } else {
    join(&path, "g++.exe")
  }
}

fn path_to_clang(path: &str, is_c: bool) -> PathBuf {
  let path = check_bin(path).unwrap();
  if is_c {
    join(&path, "clang.exe")
  } else {
    join(&path, "clang++.exe")
  }
}

pub static GCC_ID: &'static str = "gcc-mingw";

pub static GCC_SETUP: CompilerSetup = CompilerSetup {
  id: GCC_ID,
  name: "MinGW",
  description: "GCC for Windows",
  how_to_install: r"下载 MinGW。下载并解压后，您将得到一个 <code>mingw64</code> 文件夹。建议您将它妥善保存在合适的位置（如 <code>C:\mingw64</code>），并在下方输入其路径。",

  scan: scan_gcc,
  verify: Some(|path| verify(path, "g++.exe", &GCC_SETUP)),
  install: Some(install_gcc),

  verparser: verparse::gcc,
  path_to_exe: path_to_gcc,
};

pub static LLVM_ID: &'static str = "llvm-mingw";

pub static LLVM_SETUP: CompilerSetup = CompilerSetup {
  id: LLVM_ID,
  name: "LLVM MinGW",
  description: "LLVM-based MinGW toolchain",
  how_to_install: r"下载 LLVM-MinGW。下载并解压后，您将得到一个名字类似 <code>llvm-mingw-2021...</code> 的文件夹。建议您将它妥善保存在合适的位置（如 <code>C:\llvm-mingw</code>），并在下方输入其路径。",

  scan: scan_clang,
  verify: Some(|path| verify(path, "clang++.exe", &LLVM_SETUP)),
  install: Some(install_clang),

  verparser: verparse::clang,
  path_to_exe: path_to_clang,
};
