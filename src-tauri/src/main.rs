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

#![windows_subsystem = "windows"]

use anyhow::Error;

mod cli;
mod gui;
mod log;
mod steps;
mod tasks;
mod utils;

fn handle_error(e: Error) -> ! {
  if log::is_enabled() {
    if let Some(id) = tasks::statistics::send_error(&e) {
      native_dialog::MessageDialog::new()
        .set_title("程序已报告错误")
        .set_text(&format!("{}\n您可以将代码 “{}” 发送至 guyutongxue@163.com，开发者会尽快帮您解决问题。\n（使用 --no-stats 选项以关闭此弹窗。）", e.to_string(), &id[0..6]))
        .set_type(native_dialog::MessageType::Error)
        .show_alert()
        .unwrap();
    }
  } else {
    if std::env::args().len() <= 1 {
      native_dialog::MessageDialog::new()
        .set_title("日志未就绪前出现错误")
        .set_text(&format!("{:?}", e))
        .set_type(native_dialog::MessageType::Error)
        .show_alert()
        .unwrap();
    } else {
      eprintln!("日志未就绪前出现错误：{:?}", e);
    }
  }
  std::process::exit(1);
}

fn main() {
  std::env::set_var("RUST_BACKTRACE", "1");
  if let Err(e) = cli::parse_args() {
    handle_error(e);
  }
}
