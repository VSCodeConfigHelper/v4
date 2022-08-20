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

use std::str::FromStr;

#[allow(unused_imports)]
use ::log::{debug, error, info, warn};
use anyhow::{anyhow, Result};
use clap::{AppSettings, ArgEnum, CommandFactory, Parser};
use std::io::Write;

use crate::gui::gui;
use crate::log;
use crate::steps::compiler::{CompilerSetup, ENABLED_SETUPS};
use crate::steps::options::Options;
use crate::steps::{vscode, workspace};
use crate::tasks;

#[cfg(windows)]
use crate::utils::winapi;
use crate::utils::ToString;

#[derive(Parser)]
#[clap(
  global_setting(AppSettings::NoAutoVersion),
  global_setting(AppSettings::NoAutoHelp),
  author, version, about, long_about = None
)]
struct CliArgs {
  /// 显示此帮助信息并退出
  #[clap(short, long, exclusive = true)]
  help: bool,

  /// 显示程序版本信息并退出
  #[clap(short = 'V', long, exclusive = true)]
  version: bool,

  #[clap(flatten)]
  verbose: clap_verbosity_flag::Verbosity,

  /// 日志路径
  #[clap(long)]
  log_path: Option<String>,

  /// 关闭命令行交互操作，总是假设选择“是”
  #[clap(short = 'y', long)]
  assume_yes: bool,

  /// 使用图形界面。当不提供任何命令行参数时，将优先运行 GUI
  #[clap(short = 'g', long)]
  gui: bool,

  /// 使用命令行界面（此选项没有其它作用）
  #[clap(long)]
  cli: bool,

  /// 指定 VS Code 安装路径。若不提供，则尝试自动检测
  #[clap(long)]
  vscode: Option<String>,

  /// 指定编译器类型（见后）
  #[clap(short, long, default_value = DEFAULT_SETUP_ID)]
  setup: &'static CompilerSetup,

  /// 指定编译器路径。若不提供，则尝试寻找已安装的编译器
  #[clap(short, long)]
  compiler: Option<String>,

  /// 指定工作文件夹路径。使用 CLI 时必须提供
  #[clap(short, long)]
  workspace: Option<String>,

  /// 指定配置目标语言
  #[clap(short, long, arg_enum, default_value = "cpp")]
  language: Language,

  /// 指定语言标准。若不提供，则工具根据编译器版本选取
  #[clap(long, possible_values = ["c++98", "c++11", "c++14", "c++17", "c++20", "c++23", "c89", "c99", "c11", "c17"])]
  standard: Option<String>,

  /// 指定编译选项
  #[clap(short, long)]
  args: Vec<String>,

  /// 启用兼容模式
  #[clap(long)]
  compat: bool,

  /// 卸载多余的 VS Code 扩展
  #[clap(long)]
  remove_extensions: bool,

  /// 在调试前进行文件名中非 ASCII 字符的检查。仅 Windows 可用
  #[clap(long)]
  ascii_check: bool,

  /// 不将编译器添加到 Path 环境变量。仅 Windows 可用
  #[clap(long)]
  no_set_env: bool,

  /// 生成指向工作区文件夹的桌面快捷方式。仅 Windows 可用
  #[clap(long)]
  desktop_shortcut: bool,

  /// 配置完成后打开 VS Code
  #[clap(short, long)]
  open_vscode: bool,

  /// 强制生成测试文件
  #[clap(long, conflicts_with = "no-test")]
  test: bool,

  /// 不生成测试文件
  #[clap(long)]
  no_test: bool,

  /// 不发送统计数据
  #[clap(long)]
  no_stats: bool,
}

#[derive(Clone, PartialEq, ArgEnum)]
enum Language {
  #[clap(name = "cpp")]
  Cpp,
  #[clap(name = "c")]
  C,
}

static DEFAULT_SETUP_ID: &'static str = ENABLED_SETUPS[0].id;

impl FromStr for &'static CompilerSetup {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    ENABLED_SETUPS
      .iter()
      .find(|setup| setup.id == s)
      .cloned()
      .ok_or_else(|| anyhow!("Unknown compiler setup: {}", s))
  }
}

fn print_setup_help() {
  println!("\n\x1b[33mSETUPS:\x1b[0m");
  for setup in ENABLED_SETUPS {
    println!("    \x1b[32m{:10}\x1b[0m - {}", setup.id, setup.name);
  }
}

fn parse_args() -> Result<CliArgs> {
  let args = CliArgs::try_parse()?;
  log::setup(args.log_path.as_ref(), args.verbose.log_level_filter())?;
  tasks::statistics::set(!args.no_stats);
  Ok(args)
}

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

  let args = match parse_args() {
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

  if args.help {
    CliArgs::command().print_help().unwrap();
    print_setup_help();
    return Ok(());
  } else if args.version {
    print!("{}", CliArgs::command().render_version());
    println!(
      r"Copyright (C) 2022 Guyutongxue
本程序是自由软件；你可以再分发之和/或依照由自由软件基金会发布的
GNU 通用公共许可证修改之，无论是版本 3 许可证，还是（按你的决定）
任何以后版都可以。本程序不含任何保障。"
    );
    return Ok(());
  }

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

  info!("验证 VS Code 安装...");
  let vscode = match args.vscode {
    Some(vscode) => {
      if let Err(e) = vscode::verify(&vscode) {
        return Err(anyhow!("VS Code 验证失败：{}", e));
      }
      vscode
    }
    None => {
      if args.assume_yes {
        match vscode::scan() {
          Some(v) => v,
          None => return Err(anyhow!("未找到 VS Code 安装。")),
        }
      } else {
        let mut builder = requestty::Question::input("vscode")
          .message("VS Code 路径：")
          .validate_on_key(|s, _| vscode::verify(s).is_ok())
          .validate(|s, _| vscode::verify(s).map_err(|s| format!("验证失败：{}", s)));
        if let Some(dft) = vscode::scan() {
          builder = builder.default(dft);
        }
        requestty::prompt_one(builder.build())?
          .as_string()
          .unwrap()
          .into()
      }
    }
  };

  info!("VS Code 安装在 {}，", vscode);

  info!("验证工作区路径...");
  let workspace = args
    .workspace
    .ok_or(anyhow!("未在命令行中指定工作区文件夹。"))?;
  if let Err(e) = workspace::path_available(&workspace) {
    return Err(anyhow!("工作文件夹验证失败: {}", e));
  }
  info!("工作区路径为 {}。", workspace);

  info!("验证 {} 类型编译器...", args.setup.id);
  let compiler = match args.compiler.as_ref() {
    // 验证命令行传入的编译器
    Some(compiler) => match args.setup.verify {
      Some(verify) => {
        verify(&compiler).map_err(|str| anyhow!("验证编译器 {} 失败：{}", &compiler, str))?
      }
      None => Err(anyhow!("该编译器类型不支持自定义。"))?,
    },
    // 寻找已安装的编译器
    None => {
      let compilers = (args.setup.scan)();
      debug!("找到的编译器有：{:?}", compilers);
      if compilers.len() == 0 {
        Err(anyhow!("未在命令行传入编译器，也找不到已安装的编译器。"))?
      } else {
        // TODO: selection
        compilers.into_iter().nth(0).unwrap()
      }
    }
  };
  info!("编译器为 {}。", compiler.path);

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
