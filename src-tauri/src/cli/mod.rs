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

use ::log::{debug, error, info, warn};
use anyhow::{anyhow, Result};

use crate::cli::args::Language;
use crate::gui::gui;
use crate::log;
use crate::steps::options::Options;
use crate::tasks;
#[cfg(windows)]
use crate::utils::winapi;
use crate::utils::ToString;
use args::CliArgs;

mod args;
mod prompt;

#[cfg(windows)]
fn has_webview2_installed() -> bool {
  use crate::utils::winreg;
  if let Some(v) = winreg::get(
    winreg::HKEY_LOCAL_MACHINE,
    r#"SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"#,
    "pv",
  ) {
    if v != "" && v != "0.0.0.0" {
      return true;
    }
  }
  if let Some(v) = winreg::get(
    winreg::HKEY_CURRENT_USER,
    r#"Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"#,
    "pv",
  ) {
    if v != "" && v != "0.0.0.0" {
      return true;
    }
  }
  return false;
}

#[macro_export]
macro_rules! out {
  ($pattern:expr $(, $arg:expr)*) => (
    println!(concat!("\x1b[1m[OUT] ", $pattern, "\x1b[0m") $(, $arg)*)
  )
}

pub fn run() {
  let no_args = std::env::args().len() <= 1;

  let args = match args::parse() {
    Ok(args) => args,
    Err(e) => {
      if no_args {
        native_dialog::MessageDialog::new()
          .set_title("早期错误")
          .set_text(&format!("{:?}", e))
          .show_alert()
          .unwrap();
      } else {
        println!("[\x1b[31mERROR\x1b[0m] 早期错误：{:?}", e);
        #[cfg(windows)]
        {
          out!("按任意键退出...");
          winapi::getch();
        }
      }
      std::process::exit(1)
    }
  };

  if no_args {
    #[cfg(windows)]
    if !has_webview2_installed() {
      warn!("WebView 2 未安装。退化到命令行界面。");
      out!("检测到您的计算机上尚未安装 WebView 2；将使用命令行界面替代。建议您前往 https://go.microsoft.com/fwlink/p/?LinkId=2124703 下载 WebView 2 以获得更好体验。");
      cli_handled(args);
      return;
    }
    gui_handled();
    return;
  }

  if args.gui {
    gui_handled()
  } else {
    cli_handled(args)
  }
}

fn cli_handled(args: CliArgs) {
  match cli(args) {
    Ok(_) => (),
    Err(e) => {
      error!("{:?}", e);
      out!(
        "如果你认为该错误是 bug，请将 {} 文件发送到 guyutongxue@163.com。",
        log::get_log_path().to_string()
      )
    }
  }
  #[cfg(windows)]
  {
    out!("按任意键退出...");
    winapi::getch();
  }
}

fn gui_handled() {
  #[cfg(windows)]
  winapi::free_console();
  match gui() {
    Ok(_) => (),
    Err(e) => {
      if let Some(id) = tasks::statistics::send_error(&e) {
        native_dialog::MessageDialog::new()
          .set_title("程序已报告错误")
          .set_text(&format!(
            "{}\n您可以将代码 “{}” 发送至 guyutongxue@163.com，开发者会尽快帮您解决问题。",
            e.to_string(),
            &id[0..6]
          ))
          .set_type(native_dialog::MessageType::Error)
          .show_alert()
          .unwrap();
      }
    }
  }
}

fn cli(mut args: CliArgs) -> Result<()> {
  if args::early_exit(&args) {
    return Ok(());
  };

  if !cfg!(windows) {
    fn nonsupport_check(name: &'static str, flag: &mut bool) {
      if *flag {
        warn!("{} 选项在此操作系统上不支持，已忽略。", name);
        *flag = false;
      }
    }
    nonsupport_check("--ascii-check", &mut args.ascii_check);
    nonsupport_check("--no-set-env", &mut args.no_set_env);
    nonsupport_check("--desktop-shortcut", &mut args.desktop_shortcut);
  }

  info!("检查 VS Code 安装...");
  let vscode = prompt::vscode(args.vscode, args.assume_yes)?;
  info!("VS Code 安装在 {}，", vscode);

  let setup = prompt::setup(args.setup, args.assume_yes)?;

  info!("验证 {} 类型编译器...", setup.id);
  let compiler = prompt::compiler(setup, args.compiler, args.assume_yes)?;
  info!("编译器路径为 {}。", compiler.path);

  info!("检查工作区路径...");
  let workspace = prompt::workspace(args.workspace, args.assume_yes)?;
  info!("工作区路径为 {}。", workspace);

  let language = if args.language == Language::Cpp {
    "C++"
  } else {
    "C"
  };
  let add_to_path = !args.no_set_env;
  let test = if args.test {
    Some(true)
  } else if args.no_test {
    Some(false)
  } else {
    None
  };

  let task_init_args = tasks::TaskInitArgs {
    vscode: vscode,
    workspace: workspace,
    compiler: compiler,
    options: Options {
      language: language.into(),
      args: args.args,
      standard: args.standard,
      run_hotkey: args.run_hotkey,
      compatible_mode: args.compat,
      remove_extensions: args.remove_extensions,
      ascii_check: args.ascii_check,
      add_to_path: add_to_path,
      desktop_shortcut: args.desktop_shortcut,
      test: test,
      open_vscode: args.open_vscode,
      collect_data: !args.no_stats,
    },
  };
  debug!("task_init_args: {:?}", task_init_args);

  info!("正在初始化任务列表...");
  let task_list = tasks::list(task_init_args);
  debug!(
    "任务列表：{:?}",
    task_list.iter().map(|t| t.0).collect::<Vec<_>>()
  );

  for (name, action) in task_list {
    info!("正在执行任务 {}...", name);
    action()?;
    info!("任务 {} 执行完毕。", name);
  }
  Ok(())
}
