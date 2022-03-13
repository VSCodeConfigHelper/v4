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

use std::io;

use anyhow::Result;
use regex::Regex;

pub type Parser = fn(&str) -> Result<(&str, &str)>;

pub fn gcc(version_text: &str) ->Result<(&str, &str)>{
  let re = Regex::new(r"^\S*g(cc|\+\+)\S* \((.*)\) (.+)$").unwrap();
  match re.captures(version_text) {
    Some(caps) => Ok((caps.get(3).unwrap().as_str(), caps.get(2).unwrap().as_str())),
    None => Err(io::Error::new(io::ErrorKind::Other, "gcc version parse error"))?,
  }
}

pub fn clang(version_text: &str) -> Result<(&str, &str)> {
  let re = Regex::new(r"(.* )?clang version (.+)( \(.*\))?$").unwrap();
  match re.captures(version_text) {
    Some(caps) => Ok((caps.get(2).unwrap().as_str(), caps.get(3).map(|m| m.as_str()).unwrap_or(""))),
    None => Err(io::Error::new(io::ErrorKind::Other, "clang version parse error"))?,
  }
}
