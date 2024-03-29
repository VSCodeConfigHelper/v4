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

use anyhow::{anyhow, Result};
use log::{debug, trace, warn};
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;

#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;

use super::TaskArgs;
#[cfg(windows)]
use crate::utils::winapi::CREATE_NO_WINDOW;

static ENABLED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));

pub fn disable() {
  *ENABLED.lock().unwrap() = false;
}

struct ExtensionManager {
  enabled: bool,
  path: PathBuf,
  installed: Vec<String>,
}

impl ExtensionManager {
  fn get(args: &TaskArgs) -> &Mutex<Self> {
    static INSTANCE: OnceCell<Mutex<ExtensionManager>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
      let path = args.vscode.clone();
      debug!("初始化扩展管理器，路径：{:?}", path);
      let mut instance = ExtensionManager {
        enabled: *ENABLED.lock().unwrap(),
        path: path,
        installed: vec![],
      };
      let _ = instance.update();
      Mutex::new(instance)
    })
  }

  #[allow(unused)]
  fn run(&self, args: &[&str]) -> Result<String> {
    let (suc, output) = self.run_lossy(args)?;
    if suc {
      Ok(output)
    } else {
      Err(anyhow!("安装扩展出现错误：{}", output))
    }
  }

  /// 返回：(是否成功, stdout)
  /// 因为有时这个命令的返回值不是 0，不知道为什么，总之目前先忽略它的返回值
  fn run_lossy(&self, args: &[&str]) -> Result<(bool, String)> {
    if !self.enabled {
      warn!("由于启用了 --skip-ext-manage，扩展管理命令 {:?} 被跳过。请手动管理扩展以保证配置结果正确。", args);
      return Ok((true, "".into()));
    }

    let mut command = Command::new(&self.path);
    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    let output = command.args(args).output()?;
    let stdout = String::from_utf8(output.stdout)?;
    trace!("Run code with args {:?}, got output: {:?}", args, stdout);
    let suc = output.status.success();
    Ok((suc, stdout))
  }

  fn update(&mut self) -> Result<()> {
    let (_, output) = self.run_lossy(&["--list-extensions"])?;
    self.installed = output.lines().map(|line| line.to_string()).collect();
    debug!("已安装的扩展有：{:?}", &self.installed);
    Ok(())
  }

  fn install(&mut self, id: &str) -> Result<()> {
    debug!("安装扩展 {}...", id);
    if self.installed.contains(&id.to_string()) {
      debug!("扩展 {} 已经安装，跳过。", id);
      return Ok(());
    }
    self.run_lossy(&["--install-extension", id])?;
    self.installed.push(id.to_string());
    debug!("扩展 {} 安装成功。", id);
    Ok(())
  }

  fn uninstall(&mut self, id: &str) -> Result<()> {
    debug!("卸载扩展 {}...", id);
    if !self.installed.contains(&id.to_string()) {
      debug!("扩展 {} 未安装，跳过。", id);
      return Ok(());
    }
    self.run_lossy(&["--uninstall-extension", id])?;
    self.installed.retain(|installed| installed != id);
    debug!("扩展 {} 卸载成功。", id);
    Ok(())
  }
}

static C_CPP_ID: &str = "ms-vscode.cpptools";
static CODE_LLDB_ID: &str = "vadimcn.vscode-lldb";
static PAUSER_ID: &str = "Guyutongxue.pause-console";

pub fn install_pauser(args: &TaskArgs) -> Result<()> {
  let mut m = ExtensionManager::get(args).lock().unwrap();
  m.install(PAUSER_ID)
}

pub fn install_c_cpp(args: &TaskArgs) -> Result<()> {
  let mut m = ExtensionManager::get(args).lock().unwrap();
  m.install(C_CPP_ID)
}

pub fn install_code_lldb(args: &TaskArgs) -> Result<()> {
  let mut m = ExtensionManager::get(args).lock().unwrap();
  m.install(CODE_LLDB_ID)
}

pub fn remove_unrecommended(args: &TaskArgs) -> Result<()> {
  let mut m = ExtensionManager::get(args).lock().unwrap();
  [
    "formulahendry.code-runner",
    "austin.code-gnu-global",
    "danielpinto8zz6.c-cpp-compile-run",
    "mitaki28.vscode-clang",
    "jaycetyle.vscode-gnu-global",
    "franneck94.c-cpp-runner",
    "ajshort.include-autocomplete",
    "xaver.clang-format",
    "jbenden.c-cpp-flylint",
  ]
  .iter()
  .map(|id| m.uninstall(id))
  .collect()
}
