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

use std::{fs, io};
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;

use super::TaskArgs;

#[cfg(target_os = "windows")]
mod scripts {
  pub static PAUSE_CONSOLE_SCRIPT_NAME: &str = "pause-console.ps1";
  pub static PAUSE_CONSOLE_SCRIPT: &str = include_str!("../scripts/pause-console.ps1");

  pub static CHECK_ASCII_SCRIPT_NAME: &str = "check-ascii.ps1";
  pub static CHECK_ASCII_SCRIPT: &str = include_str!("../scripts/check-ascii.ps1");
}

#[cfg(target_os = "macos")]
mod scripts {
  pub static PAUSE_CONSOLE_SCRIPT_NAME: &str = "pause-console.rb";
  pub static PAUSE_CONSOLE_SCRIPT: &str = include_str!("../scripts/pause-console.rb");

  pub static PAUSE_CONSOLE_LAUNCHER_SCRIPT_NAME: &str = "pause-console-launcher.sh";
  pub static PAUSE_CONSOLE_LAUNCHER_SCRIPT: &str =
    include_str!("../scripts/pause-console-launcher.sh");
}

#[cfg(target_os = "linux")]
mod scripts {
  pub static PAUSE_CONSOLE_SCRIPT_NAME: &str = "pause-console.sh";
  pub static PAUSE_CONSOLE_SCRIPT: &str = include_str!("../scripts/pause-console.sh");
}
pub use scripts::*;
use serde_json::json;

pub fn script_path() -> Option<PathBuf> {
  dirs::data_dir().map(|p| p.join("vscch"))
}

fn save_script(filename: &str, content: &str) -> Result<()> {
  let path = script_path().ok_or(io::Error::new(io::ErrorKind::Other, "failed to get script path"))?;
  fs::create_dir_all(&path)?;
  fs::write(&path.join(filename), content)?;
  Ok(())
}

pub fn create_pauser(_: &TaskArgs) -> Result<()> {
  #[cfg(target_os = "macos")]
  save_script(
    PAUSE_CONSOLE_LAUNCHER_SCRIPT_NAME,
    PAUSE_CONSOLE_LAUNCHER_SCRIPT,
  )?;

  save_script(PAUSE_CONSOLE_SCRIPT_NAME, PAUSE_CONSOLE_SCRIPT)
}

#[cfg(target_os = "windows")]
pub fn create_checker(_: &TaskArgs) -> Result<()> {
  save_script(CHECK_ASCII_SCRIPT_NAME, CHECK_ASCII_SCRIPT)
}

pub fn create_keybinding(_: &TaskArgs) -> Result<()> {
  let key = "f6";
  let command = "workbench.action.tasks.runTask";
  let args = "run and pause";

  let filepath = dirs::config_dir()
    .ok_or(io::Error::new(io::ErrorKind::Other, "Config dir not found"))?
    .join("Code")
    .join("User")
    .join("keybindings.json");
  fs::create_dir_all(filepath.parent().unwrap())?;
  let mut result = vec![];
  if filepath.exists() {
    let file = fs::File::open(&filepath)?;
    let reader = BufReader::new(file);
    let content: Vec<serde_json::Value> = serde_json::from_reader(reader)?;
    for i in &content {
      let this_key = i["key"]
        .as_str()
        .expect("'key' of keybinding is not string");
      if this_key == key {
        // Warning for overwriting
        println!("Warning: overwriting keybinding {}", key);
      } else {
        result.push(i.clone());
      }
    }
  }
  result.push(json!({
    "key": key,
    "command": command,
    "args": args
  }));
  let result = serde_json::Value::Array(result);
  fs::write(filepath, serde_json::to_string_pretty(&result)?)?;
  Ok(())
}
