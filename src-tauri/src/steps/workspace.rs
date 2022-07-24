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

use std::path::Path;

pub fn path_available(path: &str) -> Result<(), &'static str> {
  if cfg!(windows) {
    if path.chars().all(|c| c.is_ascii()) {
      Ok(())
    } else {
      Err("路径应为 ASCII，即不能包含中文或特殊字符等")
    }
  } else {
    Ok(())
  }
}

pub fn exists(path: &str) -> bool {
  Path::new(path).join(".vscode").exists()
}
