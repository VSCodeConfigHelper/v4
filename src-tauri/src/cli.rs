use std::process::Command;

use clap::{Parser,CommandFactory,AppSettings};

use crate::utils::winapi;
use crate::gui::gui;

#[derive(Parser, Debug)]
#[clap(
  global_setting(AppSettings::NoAutoVersion),
  global_setting(AppSettings::NoAutoHelp),
  author, version, about, long_about = None
)]
pub struct CliArgs {
  /// 显示此帮助信息并退出
  #[clap(short, long)]
  pub help: bool,

  /// 显示程序版本信息并退出
  #[clap(short = 'V', long)]
  pub version: bool,

  /// 显示详细的输出信息
  #[clap(short, long)]
  pub verbose: bool,

  /// 关闭命令行交互操作，总是假设选择“是”
  #[clap(short = 'y', long)]
  pub assume_yes: bool,

  /// 使用 GUI 进行配置。当不提供任何命令行参数时，此选项将被默认使用
  #[clap(long)]
  pub use_gui: bool,

  /// 指定 VS Code 安装路径
  #[clap(long)]
  pub vscode: Option<String>,

  /// 指定编译器路径
  #[clap(long)]
  pub compiler: Option<String>,
}

struct PauseGuard;

impl PauseGuard {
  fn new() -> PauseGuard {
    #[cfg(target_os = "windows")]
    {
      winapi::alloc_console();
    }
    PauseGuard
  }
}

impl Drop for PauseGuard {
  fn drop(&mut self) {
    #[cfg(target_os = "windows")]
    {
      println!("");
      let _ = Command::new("C:\\Windows\\System32\\cmd.exe")
        .args(["/C", "PAUSE"])
        .status();
    }
  }
}

pub fn parse_args() {
  if std::env::args().len() <= 1 {
    gui();
    return;
  }

  let _pause_guard = PauseGuard::new();

  let args = match CliArgs::try_parse() {
    Err(e) => {
      println!("{}", e);
      return;
    }
    Ok(args) => args,
  };

  if args.help {
    CliArgs::command().print_help().unwrap();
    return;
  }
  if args.version {
    print!("{}", CliArgs::command().render_version());
    println!(r"Copyright (C) 2022 Guyutongxue
本程序是自由软件；你可以再分发之和/或依照由自由软件基金会发布的
GNU 通用公共许可证修改之，无论是版本 3 许可证，还是（按你的决定）
任何以后版都可以。本程序不含任何保障。");
    return;
  }

  if args.use_gui {
    gui();
  } else {
    cli(&args);
  }
}

fn cli(args: &CliArgs) {
  
}
