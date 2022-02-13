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

use regex::Regex;

pub type Parser = fn(&str) -> (&str, &str);

pub fn gcc(version_text: &str) -> (&str, &str) {
  let re = Regex::new(r"^g(cc|\+\+)(\.exe)? \((.+)\) (.*)$").unwrap();
  match re.captures(version_text) {
    Some(caps) => (caps.get(4).unwrap().as_str(), caps.get(3).unwrap().as_str()),
    None => ("unknown", version_text),
  }
}
