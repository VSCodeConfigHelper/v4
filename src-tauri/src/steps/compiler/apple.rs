#![cfg(target_os = "macos")]

use super::{common::test_compiler, verparse, Compiler, CompilerSetup};
use crate::utils::ToString;
use anyhow::{anyhow, Result};
use std::{path::Path, process::Command};

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

pub static ID: &'static str = "apple";

pub static SETUP: CompilerSetup = CompilerSetup {
  id: ID,
  name: "Apple Clang",
  description: "Apple Clang compiler with Xcode",
  how_to_install: r"安装 Xcode 命令行工具。请按照弹窗中的说明完成安装。",

  scan: scan,
  verify: None,
  install: Some(install),

  verparser: verparse::clang,
  path_to_exe: super::llvm::path_to_exe,
};
