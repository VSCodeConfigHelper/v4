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

use anyhow::{anyhow, Result};
use log::debug;
use serde_json::json;

use super::run;
use super::TaskArgs;
use crate::steps::compiler::CompilerType;
#[cfg(target_os = "macos")]
use crate::utils::sysctl;
use crate::utils::ToString;

pub static EXT: &str = if cfg!(windows) { "exe" } else { "out" };
pub static PATH_SLASH: &str = if cfg!(windows) { "\\" } else { "/" };
pub static PATH_SEPARATOR: &str = if cfg!(windows) { ";" } else { ":" };

fn single_file_build_task(args: &TaskArgs) -> Result<serde_json::Value> {
  let debug = if args.setup.is_msvc() { "/Zi" } else { "-g" };
  let output = if args.setup.is_msvc() { "/Fe:" } else { "-o" };
  let mut c_args = vec![
    debug.to_string(),
    "${file}".to_string(),
    output.to_string(),
    format!(
      "${{fileDirname}}{}${{fileBasenameNoExtension}}.{}",
      PATH_SLASH, EXT
    ),
  ];
  if args.setup.is_msvc() {
    c_args.push("/EHsc".to_string());
    if !args.args.iter().any(|a| a.starts_with("/execution-charset")) {
      c_args.push("/execution-charset:utf-8".to_string());
    }
    c_args.push("/source-charset:utf-8".to_string());
  }
  c_args.extend(args.args.clone());
  let task_args = if args.setup.is_msvc() {
    // 参见 vcvars64.bat 附近的注释
    // quoting 设置为 escape 对于 cmd.exe 没有效果
    // https://github.com/microsoft/vscode/blob/0f9d0328ebe1eccd28e4de11ece14f4b0db3e818/src/vs/workbench/contrib/tasks/browser/terminalTaskSystem.ts#L1507
    let args = c_args
      .iter()
      .map(|s| json!(s))
      .chain(std::iter::once(json!({
        "value": "\"",
        "quoting": "escape"
      })));
    json!(args.collect::<Vec<_>>())
  } else {
    json!(c_args)
  };

  let mut compiler_cmd = args.compiler_path.to_string();
  if args.setup.is_msvc() {
    compiler_cmd = "cl.exe".to_string();
  }

  Ok(json!({
    "type": if args.setup.is_msvc() { "shell" } else { "process" },
    "label": "single file build",
    "command": compiler_cmd,
    "args": task_args,
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
    "problemMatcher": if args.setup.is_msvc() { "$msCompile" } else { "$gcc" }
  }))
}

fn pause_task(args: &TaskArgs) -> Result<serde_json::Value> {
  let mut env = json!({});
  if args.setup.is_mingw() {
    let path = args.compiler_path.parent().unwrap().to_string();
    env = json!({ "Path": format!("{}{}${{env:Path}}", path, PATH_SEPARATOR) });
  }
  Ok(json!({
    "type": "pause-console",
    "label": "run and pause",
    "command": format!("${{fileDirname}}{}${{fileBasenameNoExtension}}.{}", PATH_SLASH, EXT),
    "dependsOn": "single file build",
    "args": [],
    "options": {
      "env": env,
      "cwd": "${fileDirname}"
    },
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
      run::checker_path()?.to_string(),
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
    task_list.push(pause_task(args)?);
  }
  if args.ascii_check {
    task_list.push(ascii_check_task(args)?);
  }
  let mut options = json!({});

  if cfg!(windows) {
    if args.setup.is_msvc() {
      let vcvars = Path::new(&args.compiler_path)
        .join("..\\..\\..\\..\\..\\..\\..\\Auxiliary\\Build\\vcvars64.bat");
      // MSVC 需要一堆环境变量才能正确运行，
      // 所以加载 vcvars64.bat 到 cmd.exe
      // 然后将 vscode 的 shell 设置为：
      // cmd.exe /S /C "vcvars64.bat && cl.exe ..."
      // 注意到 vcvars64.bat 路径极有可能包含空格，所以需要对引号进行处理
      // 参考 cmd.exe /? 的解释，使用 /C 时有两种引号处理策略
      // 其一是不删除任何引号，但是这要求整个命令行只有一对引号出现在开头
      // vscode 会自动为文件名带空格的参数加引号，所以这时不满足“一对引号”的要求
      // 其二是，若引号出现在最开头，则删除最开头的引号和最结尾的引号
      // 这种删除策略总是可行的，所以我们使用这一种。
      // 在 vcvars64.bat 的开头加两个引号，结尾加一个引号，
      // 另外一个引号加在所有参数的结尾（参见上方 build_task）
      // 从而 cmd.exe 把这两个引号删除，剩余的部分刚好满足需求
      let vcvars = format!("\"\"{}\"", vcvars.to_string());

      options = json!({
        "shell": {
          "executable": "C:\\Windows\\System32\\cmd.exe",
          "args": [
            "/S",
            "/C",
            vcvars,
            "&&"
          ]
        }
      });
    } else if args.setup.is_mingw() {
      let path = args.compiler_path.parent().unwrap().to_string();
      options = json!({
        "env": {
          "Path": format!("{}{}${{env:Path}}", path, PATH_SEPARATOR)
        }
      });
    }
  }

  let json = json!({
    "version": "2.0.0",
    "tasks": task_list,
    "options": options
  });

  debug!("tasks.json: {}", json);

  let path = args.workspace.join(".vscode").join("tasks.json");
  fs::write(path, serde_json::to_string_pretty(&json)?)?;
  Ok(())
}

pub fn launch_json(args: &TaskArgs) -> Result<()> {
  let debugger_name: String = match args.setup.ty {
    CompilerType::GCC => "gdb",
    CompilerType::LLVM => "lldb",
    CompilerType::MSVC => "",
  }
  .into();

  let debugger_ext = if cfg!(windows) { ".exe" } else { "" };

  let debug_type = match args.setup.ty {
    CompilerType::GCC => "cppdbg",
    CompilerType::LLVM => "lldb",
    CompilerType::MSVC => "cppvsdbg",
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

  let mut json = json!({
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
          "PATH": format!("{}{}${{env:PATH}}",
            bin_path.to_string(),
            PATH_SEPARATOR)
        },
        console_settings.0: console_settings.1,
        "MIMode": debugger_name,          // Only used in cppdbg (GDB mode)
        "miDebuggerPath": debugger_path,  // ..
        "preLaunchTask": if args.ascii_check { "ascii check" } else { "single file build" },
        "internalConsoleOptions": "neverOpen"
      }
    ]
  });
  if debug_type == "cppdbg" {
    json["configurations"][0]["setupCommands"] = json!([
      {
        "description": "Enable pretty-printing for gdb",
        "text": "-enable-pretty-printing",
        "ignoreFailures": true
      }
    ]);
  }

  debug!("launch.json: {}", json);

  let path = args.workspace.join(".vscode").join("launch.json");
  fs::write(path, serde_json::to_string_pretty(&json)?)?;
  Ok(())
}

pub fn c_cpp_properties_json(args: &TaskArgs) -> Result<()> {
  let im_compiler = match args.setup.ty {
    CompilerType::GCC => "gcc",
    CompilerType::LLVM => "clang",
    CompilerType::MSVC => "msvc",
  };
  let im_platform = std::env::consts::OS;
  let name = match im_platform {
    "windows" => "Win32",
    "macos" => "Mac",
    "linux" => "Linux",
    _ => return Err(anyhow!("unknown platform")),
  };

  #[cfg(not(target_os = "macos"))]
  let im_arch = "x64";

  #[cfg(target_os = "macos")]
  let im_arch = match sysctl::get_arch()? {
    sysctl::Aarch64 => "arm64",
    sysctl::X64 => "x64",
  };

  let standard_key = if args.is_c {
    "cStandard"
  } else {
    "cppStandard"
  };

  let json = json!({
    "version": 4i32,
    "configurations": [
      {
        "name": name,
        "includePath": [
          "${{workspaceFolder}}/**"
        ],
        "compilerPath": args.compiler_path.to_string(),
        standard_key: args.standard.to_ascii_lowercase(),
        "intelliSenseMode": format!("{}-{}-{}", im_platform, im_compiler, im_arch),
      }
    ]
  });

  debug!("c_cpp_properties.json: {}", json);

  let path = args.workspace.join(".vscode").join("c_cpp_properties.json");
  fs::write(path, serde_json::to_string_pretty(&json)?)?;
  Ok(())
}

pub fn create_folder(args: &TaskArgs) -> Result<()> {
  let path = args.workspace.join(".vscode");
  fs::create_dir_all(&path)?;
  Ok(())
}
