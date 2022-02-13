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

#![cfg(target_os = "windows")]

use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

use reqwest;
use serde::Deserialize;
use serde_json;
use winapi::um::knownfolders::FOLDERID_LocalAppData;

use super::super::winapi::get_known_folder_path;
use super::super::winapi::{FOLDERID_ProgramData, FOLDERID_ProgramFilesX86};
use super::verparse;
use super::Compiler;
use super::CompilerSetup;

static VSWHERE_DOWNLOAD_LINK: &str =
  "https://guyutongxue.oss-cn-beijing.aliyuncs.com/vswhere/vswhere.exe";

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
    return Some(tmp_path.to_path_buf());
  }
  let mut file = File::create(&tmp_path).ok()?;
  reqwest::blocking::get(VSWHERE_DOWNLOAD_LINK)
    .ok()
    .and_then(|mut res| res.copy_to(&mut file).ok())
    .map(|_| tmp_path.to_path_buf())
}

fn scan() -> Vec<Compiler> {
  let vswhere = get_vswhere();
  if vswhere.is_none() {
    println!("Failed to find vswhere.exe");
    return vec![];
  }
  let vswhere = vswhere.unwrap();

  #[derive(Deserialize)]
  struct InstallInfo {
    #[serde(rename = "installationPath")]
    installation_path: String,
    #[serde(rename = "installationVersion")]
    installation_version: String,
    #[serde(rename = "displayName")]
    display_name: String,
  }

  let list: Option<Vec<InstallInfo>> = Command::new(vswhere)
    .arg("-products")
    .arg("*")
    .arg("-requires")
    .arg("Microsoft.VisualStudio.Component.VC.Tools.x86.x64")
    .arg("-format")
    .arg("json")
    .arg("-utf8")
    .output()
    .ok()
    .and_then(|o| {
      str::from_utf8(&o.stdout)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
    });
  if list.is_none() {
    println!("Failed to parse vswhere.exe output");
    return vec![];
  }
  list
    .unwrap()
    .into_iter()
    .map(|info| Compiler {
      kind: "msvc",
      version: info.installation_version,
      path: info.installation_path,
      package_string: info.display_name,
      version_text: "".to_string(),
    })
    .collect()
}

fn install() -> bool {
  open::that("https://aka.ms/vs/17/release/vs_BuildTools.exe").is_ok()
}

pub static SETUP: CompilerSetup = CompilerSetup {
  id: "msvc",
  name: "VC++ 生成工具",
  description: "Microsoft Visual C++",
  how_to_install: r"下载 VC++ 生成工具安装器。运行安装器，按照提示完成安装。",

  scan: scan,
  verify: None,
  install: Some(install),
  verparser: verparse::gcc,
};
