#![cfg(target_os = "macos")]

use std::{path::Path, process::Command};
use anyhow::Result;
use crate::utils::ToString;
use super::{CompilerSetup, Compiler, common::test_compiler, verparse};

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
  Command::new("xcode-select --install").spawn()?;
  Ok(())
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
