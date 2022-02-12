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

use serde::Serialize;

#[derive(Serialize)]
pub struct Compiler {
  kind: &'static str,
  path: String,
  version: String,
  package_string: String,
  version_text: String
}
use super::compiler_setup::CompilerSetup;

impl Compiler {
  pub fn new(setup: &CompilerSetup, path: &str, version_text: &str) -> Compiler {
    let (version, package_string) = (setup.verparser)(version_text);
    Compiler {
      kind: setup.id,
      path: path.to_string(),
      version: version.to_string(),
      package_string: package_string.to_string(),
      version_text: version_text.to_string()
    }
  }
}
