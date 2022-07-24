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

use std::fs;

pub fn setup(verbose: bool) -> Result<()> {
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
        .chain(fs::File::create("vscch.log")?),
    )
    .chain(
      fern::Dispatch::new()
        .level(if verbose {
          log::LevelFilter::Info
        } else {
          log::LevelFilter::Warn
        })
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
