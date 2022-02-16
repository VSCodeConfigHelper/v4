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
use std::path::{Path, PathBuf};

use serde_json::json;

use super::run::*;
use super::TaskArgs;
use crate::{
  steps::compiler::{mingw::check_bin, Compiler},
  tasks::run::script_path,
};

fn compiler_executable(compiler: &Compiler, c: bool) -> Result<String, &'static str> {
  fn join(path: &str, name: &str) -> String {
    Path::new(path).join(name).to_str().unwrap().to_string()
  }
  match compiler.setup.as_str() {
    "gcc-mingw" => {
      let path = check_bin(&compiler.path).unwrap();
      if c {
        Ok(join(&path, "gcc.exe"))
      } else {
        Ok(join(&path, "g++.exe"))
      }
    }
    "llvm-mingw" => {
      let path = check_bin(&compiler.path).unwrap();
      if c {
        Ok(join(&path, "clang.exe"))
      } else {
        Ok(join(&path, "clang++.exe"))
      }
    }
    "msvc" => {
      let version_txt = Path::new(&compiler.path)
        .join("VC\\Auxiliary\\Build\\Microsoft.VCToolsVersion.default.txt");
      if !version_txt.exists() {
        return Err("无法找到 MSVC 版本文件");
      }
      let version = fs::read(version_txt).map_err(|_| "无法读取 MSVC 版本文件")?;
      let version = String::from_utf8(version).unwrap();
      Path::new(&compiler.path)
        .join("VC\\Tools\\MSVC")
        .join(version)
        .join("bin\\HostX64\\x64\\cl.exe")
        .to_str()
        .map(str::to_string)
        .ok_or("Path -> String failed")
    }
    _ => Err("Unknown compiler setup"),
  }
}

#[cfg(target_os = "windows")]
mod os_spec {
  pub static EXT: &str = "exe";
  pub static PATH_SLASH: &str = "\\";
}

#[cfg(not(target_os = "windows"))]
mod os_spec {
  pub static EXT: &str = "out";
  pub static PATH_SLASH: &str = "/";
}
use os_spec::*;

fn single_file_build_task(args: &TaskArgs) -> Result<serde_json::Value, &'static str> {
  Ok(json!({
    "type": "process",
    "label": "single file build",
    "command": compiler_executable(&args.compiler, args.options.language == "C")?,
    "args": args.options.args.iter().chain(&mut vec![
      "-g".to_string(),
      "${file}".to_string(),
      format!("${{fileDirname}}{}${{fileBasenameNoExtension}}.{}", EXT, PATH_SLASH)
    ].iter()).collect::<Vec<&String>>(),
    "group": {
      "kind": "build",
      "isDefault": true
    },
    "presentation": {
      "reveal": "silent",
      "focus": false,
      "echo": false,
      "showReuseMessage": false,
      "panel": "shared",
      "clear": true
    },
    "problemMatcher": "$gcc"
  }))
}

fn pause_task() -> Result<serde_json::Value, &'static str> {
  let script_path = script_path().unwrap();
  struct Process<'a> {
    command: &'a str,
    args: Vec<&'a str>,
  }

  #[cfg(target_os = "windows")]
  let mut process = Process {
    command: "START",
    args: vec![
      "C:\\Windows\\system32\\WindowsPowerShell\\v1.0\\powershell.exe",
      "-ExecutionPolicy",
      "ByPass",
      "-NoProfile",
      "-File",
    ],
  };

  #[cfg(target_os = "macos")]
  let mut process = Process {
    command: script_path
      .join(PAUSE_CONSOLE_SCRIPT_LAUNCHER_NAME)
      .to_str()
      .unwrap(),
    args: vec![],
  };

  #[cfg(target_os = "linux")]
  let mut process = Process {
    command: "x-terminal-emulator",
    args: vec!["-e"],
  };
  let pause_script_path = script_path.join(PAUSE_CONSOLE_SCRIPT_NAME);
  process.args.push(pause_script_path.to_str().unwrap());

  Ok(json!({
    "type": "process",
    "label": "run and pause",
    "command": process.command,
    "dependsOn": "single file build",
    "args": process.args,
    "presentation": {
      "reveal": "never",
      "focus": false,
      "echo": false,
      "showReuseMessage": false,
      "panel": "shared",
      "clear": true
    },
    "problemMatcher": []
  }))
}

fn ascii_check_task(args: &TaskArgs) -> Result<serde_json::Value, &'static str> {
  Ok(json!({
    "type": "process",
    "label": "ascii check",
    "command": "C:\\Windows\\system32\\WindowsPowerShell\\v1.0\\powershell.exe",
    "dependsOn": "single file build",
    "args": [
      "-ExecutionPolicy",
      "ByPass",
      "-NoProfile",
      "-File",
      script_path().unwrap().join(CHECK_ASCII_SCRIPT_NAME).to_str().unwrap(),
      "${fileDirname}\\${fileBasenameNoExtension}.exe"
    ],
    "presentation": {
      "reveal": "never",
      "focus": false,
      "echo": false,
      "showReuseMessage": false,
      "panel": "shared",
      "clear": true
    },
    "problemMatcher": []
  }))
}

pub fn tasks_json(args: &TaskArgs) -> Result<(), &'static str> {
  let mut task_list = vec![single_file_build_task(args)?];
  if !args.options.compatible_mode {
    task_list.push(pause_task()?);
  }
  if args.options.ascii_check {
    task_list.push(ascii_check_task(args)?);
  }
  let mut options = json!({});

  #[cfg(target_os = "windows")]
  {
    options.as_object_mut().unwrap().append(
      json!({
        "shell": {
          "executable": "C:\\Windows\\System32\\cmd.exe",
          "args": [ "/C" ]
        }
      })
      .as_object_mut()
      .unwrap(),
    );
    if ["llvm-mingw", "gcc-mingw"].contains(&args.compiler.setup.as_str()) {
      let path = check_bin(&args.compiler.path).unwrap();
      options.as_object_mut().unwrap().append(
        json!({
          "env": {
            "Path": format!("{};${{env:Path}}",path)
          }
        })
        .as_object_mut()
        .unwrap(),
      );
    }
  }

  let json = json!({
    "version": "2.0.0",
    "tasks": task_list,
    "options": options
  });

  let path = Path::new(&args.workspace)
    .join(".vscode")
    .join("tasks.json");
  fs::write(path, json.to_string()).map_err(|_| "Failed to write tasks.json")
}

pub fn launch_json(args: &TaskArgs) -> Result<(), &'static str> {
  Err("")
}

pub fn c_cpp_properties_json(args: &TaskArgs) -> Result<(), &'static str> {
  Err("")
}
