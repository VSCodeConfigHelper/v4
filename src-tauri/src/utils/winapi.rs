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

#![cfg(windows)]

use std::ffi::{c_void, OsString};
use std::os::windows::prelude::*;
use std::ptr;
use std::slice;

use anyhow::{anyhow, Result};

use windows::core::{Interface, GUID, PCWSTR, PWSTR};
use windows::Win32::Globalization::GetACP;
use windows::Win32::System::Com::{
  CoCreateInstance, CoInitialize, CoTaskMemFree, CoUninitialize, IPersistFile, CLSCTX_INPROC_SERVER,
};
use windows::Win32::System::Console;
use windows::Win32::System::Environment::ExpandEnvironmentStringsW;
use windows::Win32::System::Threading::{
  CreateProcessW, PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTUPINFOW,
};
use windows::Win32::UI::Shell::{IShellLinkW, SHGetKnownFolderPath, ShellLink};

pub static CREATE_NO_WINDOW: u32 = windows::Win32::System::Threading::CREATE_NO_WINDOW.0;

fn to_vec(s: &str) -> Vec<u16> {
  s.encode_utf16().chain(std::iter::once(0)).collect()
}

macro_rules! pcwstr {
  ( $( let $id:ident; )* ) => {
    $(
      let vec = to_vec($id);
      let $id = PCWSTR(vec.as_ptr());
    )*
  }
}

pub fn expand_environment_strings(src: &str) -> Result<String> {
  pcwstr! {let src;}

  // Get buffer size
  let size = unsafe { ExpandEnvironmentStringsW(src, &mut []) };
  let mut result: String;

  unsafe {
    let mut buf = vec![0; size as usize];
    if ExpandEnvironmentStringsW(src, &mut buf) == 0 {
      return Err(std::io::Error::last_os_error())?;
    };
    result = OsString::from_wide(&buf).into_string().unwrap();
    result.pop();
  }
  Ok(result)
}

pub fn get_known_folder_path(id: *const GUID) -> Result<String> {
  let slice;
  unsafe {
    let PWSTR(path) = SHGetKnownFolderPath(id, 0, None)?;
    let mut char_ptr = path;
    let mut len = 0;
    while char_ptr.read() != 0 {
      char_ptr = char_ptr.offset(1);
      len += 1;
    }
    slice = slice::from_raw_parts(path, len);
    CoTaskMemFree(path as *const c_void);
  };

  let os_string = OsString::from_wide(slice);
  os_string
    .into_string()
    .map_err(|_| anyhow!("Failed to convert wide string to string"))
}

pub fn get_acp() -> u32 {
  unsafe { GetACP() }
}

pub fn create_lnk(lnk: &str, target: &str, desc: &str, args: &str) -> Result<()> {
  pcwstr! {
    let lnk;
    let target;
    let desc;
    let args;
  }
  unsafe {
    CoInitialize(ptr::null())?;
    let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
    shell_link.SetPath(target)?;
    shell_link.SetDescription(desc)?;
    shell_link.SetArguments(args)?;
    let persist_file: IPersistFile = Interface::cast(&shell_link)?;
    persist_file.Save(lnk, true)?;
    CoUninitialize();
  }
  Ok(())
}

pub fn free_console() -> bool {
  unsafe { Console::FreeConsole().as_bool() }
}

pub fn alloc_console() -> bool {
  free_console() && unsafe { Console::AllocConsole().as_bool() }
}

pub fn enable_virtual_terminal() -> bool {
  unsafe {
    let handle = Console::GetStdHandle(Console::STD_OUTPUT_HANDLE).unwrap();
    let mut mode = Console::CONSOLE_MODE(0);
    if !Console::GetConsoleMode(handle, &mut mode).as_bool() {
      return false;
    }
    if !Console::SetConsoleMode(handle, mode | Console::ENABLE_VIRTUAL_TERMINAL_PROCESSING)
      .as_bool()
    {
      return false;
    }
  }
  return true;
}

extern "C" {
  fn _getch() -> i32;
}

pub fn getch() {
  unsafe {
    _getch();
  }
}
pub fn create_process(exe: &str, args: Vec<&str>) -> Result<()> {
  let args = exe.to_string()
    + " "
    + &args
      .iter()
      .map(|s| format!("\"{}\"", s))
      .collect::<Vec<_>>()
      .join(" ");
  let mut args: Vec<_> = to_vec(&args);
  let mut si: STARTUPINFOW = Default::default();
  si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
  let mut pi: PROCESS_INFORMATION = Default::default();

  let r;
  unsafe {
    r = CreateProcessW(
      PCWSTR(std::ptr::null()),
      PWSTR(args.as_mut_ptr()),
      std::ptr::null(),
      std::ptr::null(),
      false,
      PROCESS_CREATION_FLAGS(0),
      std::ptr::null(),
      PCWSTR(std::ptr::null()),
      &si,
      &mut pi,
    );
  }
  if r.as_bool() {
    return Ok(());
  } else {
    return Err(std::io::Error::last_os_error())?;
  }
}

#[cfg(test)]
mod tests {
  use windows::Win32::UI::Shell::FOLDERID_ProgramFilesX86;

  use super::*;

  #[test]
  fn test_expand_environment_strings() {
    assert_eq!(
      expand_environment_strings("%SystemDrive%").unwrap(),
      "C:".to_string()
    );
    assert_eq!(
      expand_environment_strings("%SystemRoot%\\system32\\cmd.exe").unwrap(),
      "C:\\WINDOWS\\system32\\cmd.exe".to_string()
    );
  }

  #[test]
  fn test_get_known_folder_path() {
    assert_eq!(
      get_known_folder_path(&FOLDERID_ProgramFilesX86).unwrap(),
      "C:\\Program Files (x86)".to_string()
    );
  }

  #[test]
  fn test_create_process() {
    create_process(r"cmd.exe", vec!["/K", "echo", "hello"]).unwrap();
  }
}
