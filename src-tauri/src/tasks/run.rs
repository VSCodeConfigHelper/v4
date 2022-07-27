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
#[cfg(not(windows))]
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use log::{debug, trace, warn};
use serde_json::json;

use super::TaskArgs;
pub struct Script {
  pub filename: &'static str,
  pub content: &'static str,
}

macro_rules! generate_script {
  ($os: expr, $name: expr) => {
    generate_script!($os, $name, PAUSE_CONSOLE);
  };
  ($os: expr, $filename: expr, $id: ident) => {
    #[cfg(target_os = $os)]
    pub const $id: Script = Script {
      filename: $filename,
      content: include_str!(concat!("../scripts/", $filename)),
    };
  };
}

generate_script!("windows", "pause-console.ps1");
generate_script!("macos", "pause-console.rb");
generate_script!("linux", "pause-console.sh");
generate_script!("windows", "check-ascii.ps1", CHECK_ASCII);
generate_script!("macos", "pause-console-launcher.sh", PAUSE_CONSOLE_LAUNCHER);

pub fn script_path() -> Option<PathBuf> {
  Some(dirs::data_dir()?.join("vscch"))
}

fn save_script(script: &'static Script) -> Result<()> {
  let path = script_path().ok_or(anyhow!("找不到用于存放脚本的路径。"))?;
  fs::create_dir_all(&path)?;
  let filepath = path.join(script.filename);
  fs::write(&filepath, script.content)?;
  #[cfg(not(target_os = "windows"))]
  {
    fs::set_permissions(&filepath, fs::Permissions::from_mode(0o755))?;
  }
  Ok(())
}

pub fn create_pauser(_: &TaskArgs) -> Result<()> {
  #[cfg(target_os = "macos")]
  save_script(&PAUSE_CONSOLE_LAUNCHER)?;
  save_script(&PAUSE_CONSOLE)?;

  #[cfg(target_os = "macos")]
  {
    set_terminal_preferences()?;
  }

  Ok(())
}

#[cfg(target_os = "windows")]
pub fn create_checker(_: &TaskArgs) -> Result<()> {
  save_script(&CHECK_ASCII)
}

#[cfg(not(target_os = "windows"))]
pub fn create_checker(_: &TaskArgs) -> Result<()> {
  panic!("Not supported on this platform")
}

pub fn create_keybinding(_: &TaskArgs) -> Result<()> {
  let key = "f6";
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

#[cfg(target_os = "macos")]
fn set_terminal_preferences() -> Result<()> {
  debug!("写入设置到 com.apple.Terminal.plist 中...");
  let preference_path = dirs::home_dir()
    .ok_or(anyhow!("找不到家目录。"))?
    .join("Library/Preferences/com.apple.Terminal.plist");
  let mut preference = plist::Value::from_file(&preference_path)?;
  trace!("{:?}", preference);
  let preference = preference
    .as_dictionary_mut()
    .ok_or(anyhow!(".plist 不是一个字典。"))?;
  let default_profile_name = preference
    .get("Default Window Settings")
    .and_then(|v| v.as_string())
    .ok_or(anyhow!("Default Window Settings 字段解析错误。"))?
    .to_owned();
  let default_profile = preference
    .get_mut("Window Settings")
    .and_then(|v| v.as_dictionary_mut())
    .and_then(|v| v.get_mut(&default_profile_name))
    .and_then(|v| v.as_dictionary_mut())
    .ok_or(anyhow!("Window Settings 字段解析错误。"))?;
  default_profile.insert("shellExitAction".to_string(), plist::Value::from(0));
  plist::to_file_binary(preference_path, preference)?;
  debug!("写入设置完成。");
  Ok(())
}
