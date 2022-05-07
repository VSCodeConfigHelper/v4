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

use version_compare::Version;

pub fn gcc(ver: &str) -> (&'static str, &'static str) {
  let ver = Version::from(ver);
  if ver < Version::from("4.8") {
    return ("c++98", "c99");
  } else if ver < Version::from("5.0") {
    return ("c++11", "c11");
  } else if ver < Version::from("8.0") {
    return ("c++14", "c11");
  } else if ver < Version::from("10.0") {
    return ("c++17", "c17");
  } else if ver < Version::from("11.0") {
    return ("c++20", "c17");
  } else {
    return ("c++23", "c17");
  }
}

// https://clang.llvm.org/cxx_status.html
pub fn clang(ver: &str) -> (&'static str, &'static str) {
  let ver = Version::from(ver);
  if ver < Version::from("4.3") {
    return ("c++98", "c99");
  } else if ver < Version::from("4.4") {
    return ("c++11", "c11");
  } else if ver < Version::from("5.0") {
    return ("c++14", "c11");
  } else if ver < Version::from("10.0") {
    return ("c++17", "c17");
  } else {
    return ("c++20", "c17");
  }
}

