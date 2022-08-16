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
use ::log::{debug, info, warn};
use anyhow::{anyhow, Result};
use clap::{AppSettings, ArgEnum, CommandFactory, Parser};

use crate::gui::gui;
use crate::log;
use crate::steps::compiler::{CompilerSetup, ENABLED_SETUPS};
use crate::steps::options::Options;
use crate::steps::{vscode, workspace};
use crate::tasks;
use crate::tasks::TaskInitArgs;

#[cfg(windows)]
use crate::utils::winapi;

#[derive(Parser)]
#[clap(
  global_setting(AppSettings::NoAutoVersion),
  global_setting(AppSettings::NoAutoHelp),
  author, version, about, long_about = None
)]
struct CliArgs {
  /// 显示此帮助信息并退出
  #[clap(short, long)]
  help: bool,

  /// 显示程序版本信息并退出
  #[clap(short = 'V', long)]
  version: bool,

  /// 显示详细的输出信息
  #[clap(short, long)]
  verbose: bool,

  /// 日志路径
  #[clap(long)]
  log_path: Option<String>,

  /// 关闭命令行交互操作，总是假设选择“是”
  #[clap(short = 'y', long)]
  assume_yes: bool,

  /// 使用 GUI 进行配置。当不提供任何命令行参数时，此选项将被默认使用
  #[clap(short = 'g', long)]
  use_gui: bool,

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

pub fn parse_args() -> Result<()> {
  if std::env::args().len() <= 1 {
    log::setup(None, false)?;
    return gui();
  }

  #[cfg(windows)]
  {
    winapi::attach_console();
  }

  let args = match CliArgs::try_parse() {
    Err(e) => {
      println!("{}", e);
      return Err(anyhow!("命令行解析错误。"));
    }
    Ok(args) => args,
  };

  log::setup(args.log_path.as_ref(), args.verbose)?;

  if args.help {
    CliArgs::command().print_help().unwrap();
    print_setup_help();
    return Ok(());
  }
  if args.version {
    print!("{}", CliArgs::command().render_version());
    println!(
      r"Copyright (C) 2022 Guyutongxue
本程序是自由软件；你可以再分发之和/或依照由自由软件基金会发布的
GNU 通用公共许可证修改之，无论是版本 3 许可证，还是（按你的决定）
任何以后版都可以。本程序不含任何保障。"
    );
    return Ok(());
  }
  tasks::statistics::set(!args.no_stats);

  if args.use_gui {
    gui()
  } else {
    cli(args)
  }
}

fn cli(mut args: CliArgs) -> Result<()> {
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
  let vscode = args
    .vscode
    .or_else(|| vscode::scan())
    .ok_or(anyhow!("No vscode found"))?;
  if let Err(e) = vscode::verify(&vscode) {
    return Err(anyhow!("VS Code 验证失败: {}", e));
  }
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
  
  let task_init_args = TaskInitArgs {
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
