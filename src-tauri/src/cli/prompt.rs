use anyhow::{anyhow, Result};
use log::{info, warn};
use requestty::{prompt_one, Question};

use crate::out;
use crate::steps::compiler::{Compiler, CompilerSetup, ENABLED_SETUPS};
use crate::steps::vscode;
use crate::steps::workspace;

pub fn vscode(arg: Option<String>, y: bool) -> Result<String> {
  if let Some(origin) = arg {
    match vscode::verify(&origin) {
      Err(e) => Err(anyhow!("VS Code 验证失败：{}", e)),
      Ok(_) => Ok(origin),
    }
  } else {
    let scanned = vscode::scan();
    if y {
      match scanned {
        Some(v) => Ok(v),
        None => Err(anyhow!("未找到已安装的 VS Code")),
      }
    } else {
      let mut builder = Question::input("vscode")
        .message("VS Code 路径：")
        .validate_on_key(|s, _| vscode::verify(s).is_ok())
        .validate(|s, _| vscode::verify(s).map_err(|s| format!("验证失败：{}", s)));
      if let Some(dft) = vscode::scan() {
        builder = builder.default(dft);
      }
      Ok(prompt_one(builder.build())?.as_string().unwrap().into())
    }
  }
}

pub fn setup(arg: Option<&'static CompilerSetup>, y: bool) -> Result<&'static CompilerSetup> {
  if y {
    match arg {
      Some(v) => Ok(v),
      None => {
        info!("未传入 -s，默认使用 {} 类型编译器", ENABLED_SETUPS[0].id);
        Ok(ENABLED_SETUPS[0])
      }
    }
  } else {
    let q = Question::select("setup")
      .message("选择编译器类型")
      .choices(ENABLED_SETUPS.iter().map(|s| s.name))
      .build();
    let i = prompt_one(q)?.as_list_item().unwrap().index;
    Ok(ENABLED_SETUPS[i])
  }
}

pub fn compiler(setup: &'static CompilerSetup, arg: Option<String>, y: bool) -> Result<Compiler> {
  if let Some(path) = arg {
    match setup.verify {
      Some(verify) => verify(&path).map_err(|str| anyhow!("验证编译器 {} 失败：{}", path, str)),
      None => Err(anyhow!(
        "类型 {} 的编译器不支持自定义路径。请更改编译器类型，或移除 -c 选项。",
        setup.id
      )),
    }
  } else {
    loop {
      let mut compilers = (setup.scan)();
      break if y {
        match compilers.len() {
          0 => Err(anyhow!("找不到已安装的 {}。", setup.name)),
          1 => Ok(compilers.swap_remove(0)),
          _ => {
            warn!(
              "由于开启了 -y，在多个编译器路径中自动选择 {}",
              compilers[0].path
            );
            Ok(compilers.swap_remove(0))
          }
        }
      } else {
        let mut builder = Question::select("compiler");

        // The "rescan" choice's index
        let mut rescan_idx = 0;

        match compilers.len() {
          0 => builder = builder.message("未检测到可用的编译器。您打算…"),
          1 => {
            return Ok(compilers.swap_remove(0));
          }
          _ => {
            builder = builder
              .message("检测到以下编译器：")
              .choices(
                compilers
                  .iter()
                  .map(|s| format!("{} ({})", s.version, s.path)),
              )
              .default_separator();
            rescan_idx += compilers.len() + 1;
          }
        }

        let install_idx = rescan_idx + 1;
        let new_idx = rescan_idx + 2;

        let mut actions = vec!["重新扫描"];
        match setup.install {
          Some(_) => actions.push("获取新的编译器..."),
          None => actions.push("查看获取方法"),
        }
        match setup.verify {
          Some(_) => actions.push("填写新的编译器路径..."),
          None => {}
        }

        let chosen = prompt_one(builder.choices(actions).build())?
          .as_list_item()
          .unwrap()
          .index;

        match chosen {
          x if x == rescan_idx => continue,
          x if x == install_idx => {
            let how_to = setup
              .how_to_install
              .replace("<code>", "\x1b[34m")
              .replace("</code>", "\x1b[39m");
            out!("请{}", how_to);
            if let Some(install) = setup.install {
              install()?;
            }
            continue;
          }
          x if x == new_idx => {
            let verify = setup.verify.unwrap();
            let question = Question::input("compiler_path")
              .message("输入编译器路径：")
              .validate_on_key(|s, _| verify(s).is_ok())
              .validate(|s, _| {
                verify(s).map_err(|s| format!("验证失败：{}", s))?;
                Ok(())
              })
              .build();
            let prompt = prompt_one(question)?;
            let path = prompt.as_string().unwrap();
            let compiler = verify(path).unwrap();
            Ok(compiler)
          }
          _ => Ok(compilers.swap_remove(chosen)),
        }
      };
    }
  }
}

pub fn workspace(arg: Option<String>, y: bool) -> Result<String> {
  if let Some(origin) = arg {
    match workspace::path_available(&origin) {
      Err(e) => Err(anyhow!("工作文件夹不可用：{}", e)),
      Ok(_) => Ok(origin),
    }
  } else if y {
    Err(anyhow!("由于未从命令行指定工作文件夹，且用户输入被禁用；程序无法继续。请指定 -w 选项，或者关闭 -y 开关。"))
  } else {

    let question = Question::input("vscode")
      .message("工作文件夹路径：")
      .validate_on_key(|s, _| workspace::path_available(s).is_ok())
      .validate(|s, _| workspace::path_available(s).map_err(|s| format!("该文件夹不可用：{}", s)))
      .build();
    Ok(prompt_one(question)?.as_string().unwrap().into())
  }
}
