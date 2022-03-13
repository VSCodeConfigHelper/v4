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
use std::path::Path;

use anyhow::Result;
use serde_json::json;

use super::run::*;
use super::TaskArgs;
use crate::utils::ToString;

#[cfg(target_os = "windows")]
mod os_spec {
  pub static EXT: &str = "exe";
  pub static PATH_SLASH: &str = "\\";
  pub static PATH_SEPARATOR: &str = ";";
}

#[cfg(not(target_os = "windows"))]
mod os_spec {
  pub static EXT: &str = "out";
  pub static PATH_SLASH: &str = "/";
  pub static PATH_SEPARATOR: &str = ":";
}
use os_spec::*;

fn single_file_build_task(args: &TaskArgs) -> Result<serde_json::Value> {
  let debug = if args.compiler_setup.id == "msvc" {
    "/Zi"
  } else {
    "-g"
  };
  let output = if args.compiler_setup.id == "msvc" {
    "/Fe:"
  } else {
    "-o"
  };
  let mut c_args = vec![
    debug.to_string(),
    "${file}".to_string(),
    output.to_string(),
    format!(
      "${{fileDirname}}{}${{fileBasenameNoExtension}}.{}",
      PATH_SLASH, EXT
    ),
  ];
  if args.compiler_setup.id == "msvc" {
    c_args.push("/EHsc".to_string());
    c_args.push("/source-charset:utf-8".to_string());
  }
  c_args.extend(args.args.clone());

  let mut compiler_cmd = args.compiler_path.to_string();
  if args.compiler_setup.id == "msvc" {
    compiler_cmd = "cl.exe".to_string();
  }

  Ok(json!({
    "type": "shell",
    "label": "single file build",
    "command": compiler_cmd,
    "args": c_args,
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

fn pause_task() -> Result<serde_json::Value> {
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
    "type": "shell",
    "label": "run and pause",
    "command": process.command,
    "dependsOn": "single file build",
    "args": process.args.into_iter().map(serde_json::Value::from).chain(vec![
      serde_json::Value::from(format!("${{fileDirname}}{}${{fileBasenameNoExtension}}.{}", PATH_SLASH, EXT)),
    ]).collect::<Vec<_>>(),
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

#[cfg(target_os = "windows")]
fn ascii_check_task(_: &TaskArgs) -> Result<serde_json::Value> {
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
      script_path().unwrap().join(CHECK_ASCII_SCRIPT_NAME).to_string(),
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

pub fn tasks_json(args: &TaskArgs) -> Result<()> {
  let mut task_list = vec![single_file_build_task(args)?];
  if !args.compatible_mode {
    task_list.push(pause_task()?);
  }
  #[cfg(target_os = "windows")]
  if args.ascii_check {
    task_list.push(ascii_check_task(args)?);
  }
  let mut options = json!({});

  #[cfg(target_os = "windows")]
  {
    let mut shell_args = vec!["/C".to_string()];
    if args.compiler_setup.id == "msvc" {
      let cl_path = Path::new(&args.compiler_path)
        .join("..\\..\\..\\..\\..\\..\\..\\..\\Common7\\Tools\\VsDevCmd.bat");
      shell_args.push(format!("\"{}\"", cl_path.to_string()));
      shell_args.push("&&".to_string());
    }

    options.as_object_mut().unwrap().append(
      json!({
        "shell": {
          "executable": "C:\\Windows\\System32\\cmd.exe",
          "args": shell_args
        }
      })
      .as_object_mut()
      .unwrap(),
    );
    if super::mingw_setup(args.compiler_setup) {
      let path = args.compiler_path.parent().unwrap().to_string();
      options.as_object_mut().unwrap().append(
        json!({
          "env": {
            "Path": format!("{}{}${{env:Path}}", path, PATH_SEPARATOR)
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
  fs::write(path, serde_json::to_string_pretty(&json)?)?;
  Ok(())
}

pub fn launch_json(args: &TaskArgs) -> Result<()> {
  let debugger_name: String = if super::llvm_setup(args.compiler_setup) {
    "lldb".into()
  } else {
    "gdb".into()
  };

  #[cfg(target_os = "windows")]
  let debugger_ext = ".exe";

  #[cfg(not(target_os = "windows"))]
  let debugger_ext = "";

  let debug_type = if super::llvm_setup(args.compiler_setup) {
    "lldb"
  } else if args.compiler_setup.id == "msvc" {
    "cppvsdbg"
  } else {
    "cppdbg"
  };

  let bin_path = args.compiler_path.parent().unwrap();
  let debugger_path = bin_path
    .join(format!("{}{}", debugger_name, debugger_ext))
    .to_string();
  let console_settings = if debug_type == "cppdbg" {
    (
      "externalConsole",
      serde_json::to_value(!args.compatible_mode)?,
    )
  } else {
    ("console", serde_json::to_value("externalTerminal")?)
  };

  let json = json!({
    "version": "0.2.0",
    "configurations": [
      {
        "name": "single file debug",
        "type": debug_type,
        "request": "launch",
        "program": format!("${{fileDirname}}{}${{fileBasenameNoExtension}}.{}", PATH_SLASH, EXT),
        "args": [],
        "stopAtEntry": false,
        "cwd": "${fileDirname}",
        "environment": [],
        "env": {
          "Path": format!("{}{}${{env:Path}}",
            bin_path.to_string(),
            PATH_SEPARATOR)
        },
        console_settings.0: console_settings.1,
        "MIMode": debugger_name,          // Only used in GDB mode
        "miDebuggerPath": debugger_path,  // ..
        "preLaunchTask": if args.ascii_check { "ascii check" } else { "single file build" },
        "internalConsosleOptions": "neverOpen"
      }
    ]
  });

  let path = Path::new(&args.workspace)
    .join(".vscode")
    .join("launch.json");
  fs::write(path, serde_json::to_string_pretty(&json)?)?;
  Ok(())
}

pub fn c_cpp_properties_json(args: &TaskArgs) -> Result<()> {
  let intellisense_mode = match args.compiler_setup.id {
    "gcc-mingw" => "windows-gcc-x64",
    "msvc" => "windows-msvc-x64",
    "llvm-mingw" => "windows-clang-x64",
    "gcc" => "linux-gcc-x64",    // TODO: Should be platform specific
    "llvm" => "linux-clang-x64",
    _ => panic!(),
  };

  #[cfg(target_os = "windows")]
  let platform = "Win32";

  #[cfg(target_os = "macos")]
  let platform = "Mac";

  #[cfg(target_os = "linux")]
  let platform = "Linux";

  let standard_key = if args.is_c {
    "cStandard"
  } else {
    "cppStandard"
  };
  // TODO
  let standard = args
    .standard
    .as_ref()
    .map(String::as_str)
    .unwrap_or("c++17");

  let json = json!({
    "version": 4i32,
    "configurations": [
      {
        "name": platform,
        "includePath": [
          "${{workspaceFolder}}/**"
        ],
        "compilerPath": args.compiler_path.to_string(),
        standard_key: standard,
        "intelliSenseMode": intellisense_mode,
      }
    ]
  });

  let path = Path::new(&args.workspace)
    .join(".vscode")
    .join("c_cpp_properties.json");
  fs::write(path, serde_json::to_string_pretty(&json)?)?;
  Ok(())
}

pub fn create_folder(args: &TaskArgs) -> Result<()> {
  let path = Path::new(&args.workspace).join(".vscode");
  fs::create_dir_all(&path)?;
  Ok(())
}
