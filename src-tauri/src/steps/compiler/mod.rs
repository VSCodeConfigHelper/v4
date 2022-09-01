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

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use std::ops::Deref;
use std::path::PathBuf;
use std::fmt;

pub mod stdchoose;
mod verparse;

pub mod apple;
mod common;
pub mod gcc;
pub mod llvm;
pub mod mingw;
pub mod msvc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Compiler {
  pub setup: Id,
  pub path: String,
  pub version: String,
  package_string: String,
}

impl Compiler {
  pub fn new(setup: &CompilerSetup, path: &str, version_text: &str) -> Option<Compiler> {
    let (version, package_string) = match setup.ty {
      CompilerType::GCC => verparse::gcc(version_text).ok()?,
      CompilerType::LLVM => verparse::clang(version_text).ok()?,
      CompilerType::MSVC => return None,
    };
    let compiler = Compiler {
      setup: setup.id,
      path: path.to_string(),
      version: version.to_string(),
      package_string: package_string.to_string(),
    };
    Some(compiler)
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum Id {
  #[serde(rename = "msvc")]
  MSVC,
  #[serde(rename = "gcc-mingw")]
  MinGW,
  #[serde(rename = "llvm-mingw")]
  LLVMMinGW,
  #[serde(rename = "gcc")]
  GCC,
  #[serde(rename = "llvm")]
  LLVM,
  #[serde(rename = "apple")]
  Apple,
}

impl fmt::Display for Id {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", to_variant_name(self).unwrap())
  }
}

impl Deref for Id {
  type Target = &'static CompilerSetup;
  fn deref(&self) -> &&'static CompilerSetup {
    ENABLED_SETUPS.iter().find(|s| s.id == *self).unwrap()
  }
}

#[derive(PartialEq)]
pub enum CompilerType {
  GCC,
  LLVM,
  MSVC,
}

pub struct CompilerSetup {
  pub id: Id,
  pub name: &'static str,
  pub description: &'static str,
  pub how_to_install: &'static str,

  pub scan: fn() -> Vec<Compiler>,
  pub verify: Option<fn(&str) -> Result<Compiler, &'static str>>,
  pub install: Option<fn() -> Result<()>>,

  pub ty: CompilerType,
  pub path_to_exe: fn(path: &str, is_c: bool) -> PathBuf,
}

impl CompilerSetup {
  pub fn is_mingw(&self) -> bool {
    self.id == Id::MinGW || self.id == Id::LLVMMinGW
  }

  pub fn is_msvc(&self) -> bool {
    self.id == Id::MSVC
  }
}

#[cfg(target_os = "windows")]
pub static ENABLED_SETUPS: &[&CompilerSetup] =
  &[&mingw::GCC_SETUP, &msvc::SETUP, &mingw::LLVM_SETUP];

#[cfg(target_os = "macos")]
pub static ENABLED_SETUPS: &[&CompilerSetup] = &[&apple::SETUP];

#[cfg(target_os = "linux")]
pub static ENABLED_SETUPS: &[&CompilerSetup] = &[&gcc::SETUP, &llvm::SETUP];
