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

#![cfg(windows)]

use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, str};

use anyhow::Result;
use log::{debug, error, warn};
use serde::Deserialize;
use serde_json;

use super::{Compiler, CompilerSetup, CompilerType};
use crate::utils::winapi::{get_known_folder_path, CREATE_NO_WINDOW};
use crate::utils::ToString;
use windows::Win32::UI::Shell::{
  FOLDERID_LocalAppData, FOLDERID_ProgramData, FOLDERID_ProgramFilesX86,
};

static POWERSHELL: &str = "C:\\Windows\\system32\\WindowsPowerShell\\v1.0\\powershell.exe";
static VSWHERE_DOWNLOAD_LINK: &str =
  "http://v4.vscch.tk/proxy/?target=https%3A%2F%2Fgithub.com%2Fmicrosoft%2Fvswhere%2Freleases%2Fdownload%2F3.0.3%2Fvswhere.exe";

fn get_vswhere() -> Option<PathBuf> {
  let choco_path = get_known_folder_path(&FOLDERID_ProgramData)
    .ok()
    .map(|p| Path::new(&p).join(r"chocolatey\bin\vswhere.exe"))
    .filter(|p| p.exists());
  if choco_path.is_some() {
    return choco_path;
  }
  let vs_path = get_known_folder_path(&FOLDERID_ProgramFilesX86)
    .ok()
    .map(|p| Path::new(&p).join(r"Microsoft Visual Studio\Installer\vswhere.exe"))
    .filter(|p| p.exists());
  if vs_path.is_some() {
    return vs_path;
  }
  let tmp_path = get_known_folder_path(&FOLDERID_LocalAppData)
    .ok()
    .map(|p| Path::new(&p).join(r"Temp\vswhere.exe"))?;
  if tmp_path.exists() {
    return Some(tmp_path);
  }
  Command::new(POWERSHELL)
    .creation_flags(CREATE_NO_WINDOW)
    .args([
      "-Command",
      &(format!(
        "Invoke-WebRequest -Uri {} -OutFile {}",
        VSWHERE_DOWNLOAD_LINK,
        tmp_path.to_string()
      )),
    ])
    .output()
    .ok()
    .map(|_| tmp_path)
}

fn scan() -> Vec<Compiler> {
  let vswhere = get_vswhere();
  if vswhere.is_none() {
    warn!("找不到 vswhere.exe。");
    return vec![];
  }
  let vswhere = vswhere.unwrap();
  debug!("vswhere.exe 路径：{:?}", vswhere);

  #[derive(Deserialize)]
  #[serde(rename_all = "camelCase")]
  struct InstallInfo {
    installation_path: String,
    installation_version: String,
    display_name: String,
  }

  let list: Option<Vec<InstallInfo>> = Command::new(vswhere)
    .creation_flags(CREATE_NO_WINDOW)
    .args(&[
      "-products",
      "*",
      "-requires",
      "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
      "-format",
      "json",
      "-utf8",
    ])
    .output()
    .ok()
    .and_then(|o| {
      // 受 https://github.com/microsoft/vswhere/issues/262 影响，某些 vswhere 可能存在编码错误
      let s = String::from_utf8_lossy(&o.stdout).into_owned();
      debug!("vswhere.exe 输出：{}", s);
      serde_json::from_str(&s).ok()
    });
  if list.is_none() {
    error!("vswhere.exe 运行失败或无法解析其输出。");
    return vec![];
  }
  list
    .unwrap()
    .into_iter()
    .map(|info| Compiler {
      setup: super::Id::MSVC,
      version: info.installation_version,
      path: info.installation_path,
      package_string: info.display_name,
    })
    .collect()
}

fn install() -> Result<()> {
  open::that("https://aka.ms/vs/17/release/vs_BuildTools.exe")?;
  Ok(())
}

pub fn path_to_exe(path: &str, _: bool) -> PathBuf {
  let version_txt =
    Path::new(&path).join("VC\\Auxiliary\\Build\\Microsoft.VCToolsVersion.default.txt");
  let version = fs::read(version_txt).unwrap();
  let version = String::from_utf8(version).unwrap();
  Path::new(path)
    .join("VC\\Tools\\MSVC")
    .join(version.trim())
    .join("bin\\HostX64\\x64\\cl.exe")
}

pub static SETUP: CompilerSetup = CompilerSetup {
  id: super::Id::MSVC,
  name: "VC++ 生成工具",
  description: "Microsoft Visual C++",
  how_to_install: r"下载 VC++ 生成工具安装器。运行安装器，按照提示完成安装。",

  scan: scan,
  verify: None,
  install: Some(install),

  ty: CompilerType::MSVC,
  path_to_exe: path_to_exe,
};
