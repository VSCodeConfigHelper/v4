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

use anyhow::Result;
use fern::colors::ColoredLevelConfig;
use log::info;
use once_cell::sync::Lazy;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

static DEFAULT_LOG_PATH: Lazy<PathBuf> = Lazy::new(|| {
  dirs::data_dir()
    .map(|s| s.join("vscch"))
    .and_then(|s| {
      fs::create_dir_all(&s).ok()?;
      Some(s)
    })
    .unwrap_or(PathBuf::from(""))
    .join(format!("vscch_{}.log", chrono::Local::now().format("%Y%m%d%H%M%S")))
});
static LOG_PATH: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Default::default());

pub fn get_log_path() -> PathBuf {
  if let Some(p) = LOG_PATH.lock().unwrap().as_ref() {
    p.clone()
  } else {
    DEFAULT_LOG_PATH.clone()
  }
}

pub fn setup(path: Option<&String>, log_level: log::LevelFilter) -> Result<()> {
  *LOG_PATH.lock().unwrap() = path.map(|s| PathBuf::from(s));
  fern::Dispatch::new()
    .chain(
      fern::Dispatch::new()
        .level(log::LevelFilter::Trace)
        .format(|out, message, record| {
          out.finish(format_args!(
            "{}[{}][{}] {}",
            chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
            record.target(),
            record.level(),
            message
          ))
        })
        .chain(fs::File::create(get_log_path())?),
    )
    .chain(
      fern::Dispatch::new()
        .level(log_level)
        .format(|out, message, record| {
          out.finish(format_args!(
            "[{}] {}",
            ColoredLevelConfig::new().color(record.level()),
            message
          ))
        })
        .chain(std::io::stdout()),
    )
    .apply()?;

  info!("版本 v{}", env!("CARGO_PKG_VERSION"));
  info!("操作系统 {}", os_info::get());
  info!("处理器 {}", std::env::consts::ARCH);
  Ok(())
}
