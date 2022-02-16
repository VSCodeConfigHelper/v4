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

use super::TaskArgs;
use std::fs;
use std::path::{Path, PathBuf};

fn c_comment(s: &str) -> String {
  format!("/* {} */", s)
}

fn cpp_comment(s: &str) -> String {
  format!("// {}", s)
}

static C_HELLOWORLD: &str = r#"
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    /* 在标准输出中打印 "Hello, world!" */
    printf("Hello, world!\n");
    return EXIT_SUCCESS;
}
"#;

static CPP_HELLOWORLD: &str = r#"
#include <iostream>

int main() {
    // 在标准输出中打印 "Hello, world!"
    std::cout << "Hello, world!" << std::endl;
}
"#;

#[cfg(target_os = "macos")]
mod key {
  pub static CTRL_CMD: &str = "⌘";
  pub static CTRL: &str = "⌃";
  pub static SHIFT: &str = "⇧";
  pub static SEP: &str = " ";
}

#[cfg(not(target_os = "macos"))]
mod key {
  pub static CTRL_CMD: &str = "Ctrl";
  pub static CTRL: &str = "Ctrl";
  pub static SHIFT: &str = "Shift";
  pub static SEP: &str = " + ";
}

use key::*;

pub fn filepath(args: &TaskArgs) -> PathBuf {
  let ext = if args.options.language == "C" { "c" } else { "cpp" };
  let mut i = 0;
  loop {
    let path = Path::new(&args.workspace).join(if i == 0 {
      format!("helloworld.{}", ext)
    } else {
      format!("helloworld({}).{}", i, ext)
    });
    if !path.exists() {
      return path;
    }
    i += 1;
  }
}

pub fn generate(args: &TaskArgs) -> Result<(), &'static str> {
  let c = args.options.language == "C";
  let ext = if c { "c" } else { "cpp" };
  macro_rules! cmt {
    ( $( $x:expr ),* ) => {
      &(if c { c_comment } else { cpp_comment })(&format!($( $x ),*))
    };
  }
  let helloworld = if c { &C_HELLOWORLD } else { &CPP_HELLOWORLD };

  let path = filepath(args);

  let run_key = if args.options.compatible_mode {
    format!("{}{}F5", CTRL, SEP)
  } else {
    "F6".to_string()
  };

  fs::write(
    path,
    &[
      cmt!("VS Code C/C++ 测试代码 \"Hello World\""),
      cmt!("由 VSCodeConfigHelper v{} 生成", env!("CARGO_PKG_VERSION")),
      "",
      cmt!(
        "您可以在当前文件夹（工作文件夹）下新建 .{} 源文件编写代码。",
        ext
      ),
      "",
      cmt!("按下 {} 编译运行", run_key),
      cmt!("按下 F5 编译调试。"),
      cmt!("按下 {}{}{}{}B 编译", CTRL_CMD, SEP, SHIFT, SEP),
      helloworld,
      cmt!("此文件编译运行将输出 \"Hello, world!\"。"),
      cmt!("按下 {} 后，你将在弹出的终端窗口中看到这一行字。", run_key),
      cmt!(
        "{}",
        if args.compiler.setup == "gcc-mingw" {
          "!! 重要提示：请您在编写代码前，确认文件名不含中文或特殊字符。 !!"
        } else {
          ""
        }
      ),
      cmt!("若遇到问题，请联系开发者 guyutongxue@163.com。"),
    ]
    .join("\n"),
  )
  .map_err(|_| "Failed to write file")
}
