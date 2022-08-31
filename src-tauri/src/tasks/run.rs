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

use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use log::{debug, trace, warn};
use serde_json::json;

use super::TaskArgs;

pub fn create_checker(_: &TaskArgs) -> Result<()> {
  let path = dirs::data_dir()
    .ok_or(anyhow!("找不到用于存放脚本的路径。"))?
    .join("vscch");
  fs::create_dir_all(&path)?;
  let filepath = path.join("check-ascii.ps1");
  fs::write(&filepath, include_str!("../scripts/check-ascii.ps1"))?;
  Ok(())
}

pub fn checker_path() -> Result<PathBuf> {
  Ok(
    dirs::data_dir()
      .ok_or(anyhow!("找不到用于存放脚本的路径。"))?
      .join("vscch/check-ascii.ps1"),
  )
}

pub fn create_keybinding(args: &TaskArgs) -> Result<()> {
  let key = &args.run_hotkey;
  let command = "workbench.action.tasks.runTask";
  let args = "run and pause";

  let filepath = dirs::config_dir()
    .ok_or(anyhow!("找不到配置文件存放的路径。"))?
    .join("Code")
    .join("User")
    .join("keybindings.json");
  fs::create_dir_all(filepath.parent().unwrap())?;
  let mut result = vec![];
  if filepath.exists() {
    let content = fs::read_to_string(&filepath)?;
    // JSON5 是 vscode JSON with comment 的超集。直接用 serde_json 可能出错
    let content: Vec<serde_json::Value> = json5::from_str(&content)?;
    for i in &content {
      let this_key = i["key"].as_str().ok_or(anyhow!(
        "keybindings.json 中的 \"key\" 字段应为 string 类型。"
      ))?;
      if this_key == key {
        // Warning for overwriting
        warn!("快捷键 {} 已有配置，将被覆盖。", key);
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
