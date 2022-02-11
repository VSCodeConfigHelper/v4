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

pub use super::compiler::Compiler;

pub struct CompilerSetup {
  pub name: &'static str,
  pub description: &'static str,
  pub scan: fn() -> Vec<Compiler>,
  pub validate: fn(&str) -> Option<Compiler>
}

pub mod gcc_mingw;
pub mod llvm_mingw;
pub mod msvc;

#[cfg(target_os = "windows")]
pub static ENABLED_SETUPS: &'static [&'static CompilerSetup] = &[
  &gcc_mingw::SETUP,
  &llvm_mingw::SETUP,
  &msvc::SETUP
];

#[cfg(target_os = "macos")]
pub static ENABLED_SETUPS: &'static [&'static CompilerSetup] = &[];

#[cfg(target_os = "linux")]
pub static ENABLED_SETUPS: &'static [&'static CompilerSetup] = &[];
