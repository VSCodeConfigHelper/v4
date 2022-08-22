use anyhow::{anyhow, Result};
use clap::{AppSettings, ArgEnum, CommandFactory, Parser};
use std::str::FromStr;

use crate::log;
use crate::steps::compiler::{CompilerSetup, ENABLED_SETUPS};
use crate::tasks;

#[derive(Parser)]
#[clap(
  global_setting(AppSettings::NoAutoVersion),
  global_setting(AppSettings::NoAutoHelp),
  author, version, about, long_about = None
)]
pub struct CliArgs {
  /// 显示此帮助信息并退出
  #[clap(short, long, exclusive = true)]
  pub help: bool,

  /// 显示程序版本信息并退出
  #[clap(short = 'V', long, exclusive = true)]
  pub version: bool,

  #[clap(flatten)]
  pub verbose: clap_verbosity_flag::Verbosity,

  /// 日志路径
  #[clap(long)]
  pub log_path: Option<String>,

  /// 关闭命令行交互操作，总是假设选择“是”
  #[clap(short = 'y', long)]
  pub assume_yes: bool,

  /// 使用图形界面。当不提供任何命令行参数时，将优先运行 GUI
  #[clap(short = 'g', long)]
  pub gui: bool,

  /// 使用命令行界面（此选项没有其它作用）
  #[clap(long, conflicts_with = "gui")]
  cli: bool,

  /// 指定 VS Code 安装路径。若不提供，则尝试自动检测
  #[clap(long)]
  pub vscode: Option<String>,

  /// 指定编译器类型（见后）
  #[clap(short, long)]
  pub setup: Option<&'static CompilerSetup>,

  /// 指定编译器路径。若不提供，则尝试寻找已安装的编译器
  #[clap(short, long)]
  pub compiler: Option<String>,

  /// 指定工作文件夹路径。使用 CLI 时必须提供
  #[clap(short, long)]
  pub workspace: Option<String>,

  /// 指定配置目标语言
  #[clap(short, long, arg_enum, default_value = "cpp")]
  pub language: Language,

  /// 指定语言标准。若不提供，则工具根据编译器版本选取
  #[clap(long, possible_values = ["c++98", "c++11", "c++14", "c++17", "c++20", "c++23", "c89", "c99", "c11", "c17"])]
  pub standard: Option<String>,

  /// 指定编译选项
  #[clap(short, long)]
  pub args: Vec<String>,

  /// 指定运行快捷键
  #[clap(long, default_value = "f6")]
  pub run_hotkey: String,

  /// 启用兼容模式
  #[clap(long)]
  pub compat: bool,

  /// 卸载多余的 VS Code 扩展
  #[clap(long)]
  pub remove_extensions: bool,

  /// 在调试前进行文件名中非 ASCII 字符的检查。仅 Windows 可用
  #[clap(long)]
  pub ascii_check: bool,

  /// 不将编译器添加到 Path 环境变量。仅 Windows 可用
  #[clap(long)]
  pub no_set_env: bool,

  /// 生成指向工作区文件夹的桌面快捷方式。仅 Windows 可用
  #[clap(long)]
  pub desktop_shortcut: bool,

  /// 配置完成后打开 VS Code
  #[clap(short, long)]
  pub open_vscode: bool,

  /// 强制生成测试文件
  #[clap(long, conflicts_with = "no-test")]
  pub test: bool,

  /// 不生成测试文件
  #[clap(long)]
  pub no_test: bool,

  /// 不发送统计数据
  #[clap(long)]
  pub no_stats: bool,

  /// 跳过扩展管理步骤。仅当扩展管理无法正确运行时使用此选项
  #[clap(long)]
  pub skip_ext_manage: bool,
}

#[derive(Clone, PartialEq, ArgEnum)]
pub enum Language {
  #[clap(name = "cpp")]
  Cpp,
  #[clap(name = "c")]
  C,
}

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
  for (i, setup) in ENABLED_SETUPS.iter().enumerate() {
    println!(
      "    \x1b[32m{:10}\x1b[0m - {}{} \x1b[38;5;242m[{}]\x1b[0m",
      setup.id,
      setup.name,
      if i == 0 { " (默认)" } else { "" },
      setup.description,
    );
  }
}

pub fn parse() -> Result<CliArgs> {
  let args = CliArgs::try_parse()?;
  log::setup(args.log_path.as_ref(), args.verbose.log_level_filter())?;
  tasks::statistics::set(!args.no_stats);
  if args.skip_ext_manage {
    tasks::extension::disable();
  }
  Ok(args)
}

pub fn early_exit(args: &CliArgs) -> bool {
  if args.help {
    CliArgs::command().print_help().unwrap();
    print_setup_help();
    return true;
  } else if args.version {
    print!("{}", CliArgs::command().render_version());
    println!(
      r"Copyright (C) 2022 Guyutongxue
本程序是自由软件；你可以再分发之和/或依照由自由软件基金会发布的
GNU 通用公共许可证修改之，无论是版本 3 许可证，还是（按你的决定）
任何以后版都可以。本程序不含任何保障。"
    );
    return true;
  }
  return false;
}
