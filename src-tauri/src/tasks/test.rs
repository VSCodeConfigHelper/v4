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

use std::fs::File;
use std::path::Path;
use super::TaskArgs;

pub fn generate(args: &TaskArgs) -> Result<(), &'static str> {
  let ext = if args.options.language == "C" {
    "c"
  } else {
    "cpp"
  };
  let mut i = 0;
  let path = loop {
    let path = Path::new(&args.workspace).join(if i == 0 {
      format!("helloworld.{}", ext)
    } else {
      format!("helloworld({}).{}", i, ext)
    });
    if !path.exists() {
      break path;
    }
    i += 1;
  };
  let mut file = File::create(&path).map_err(|_| "Failed to create file")?;
  Err("Not implemented")
}
