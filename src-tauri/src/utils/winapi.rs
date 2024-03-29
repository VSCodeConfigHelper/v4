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
use std::slice;

use anyhow::{anyhow, Result};

use windows::core::{Interface, GUID, PCWSTR, PWSTR};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Globalization::{
  GetACP, MultiByteToWideChar, CP_ACP, MULTI_BYTE_TO_WIDE_CHAR_FLAGS,
};
use windows::Win32::System::Com::{
  CoCreateInstance, CoInitialize, CoTaskMemFree, CoUninitialize, IPersistFile, CLSCTX_INPROC_SERVER,
};
use windows::Win32::System::Console;
use windows::Win32::System::Environment::ExpandEnvironmentStringsW;
use windows::Win32::UI::Shell::{IShellLinkW, SHGetKnownFolderPath, ShellLink, KNOWN_FOLDER_FLAG};

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
  pcwstr! { let src; }

  // Get buffer size
  let size = unsafe { ExpandEnvironmentStringsW(src, None) };

  let mut buf = Vec::with_capacity(size as usize);
  unsafe {
    buf.set_len(size as usize);
    if ExpandEnvironmentStringsW(src, Some(&mut buf)) == 0 {
      return Err(std::io::Error::last_os_error())?;
    };
  }
  let mut result = OsString::from_wide(&buf).into_string().unwrap();
  result.pop();
  Ok(result)
}

pub fn get_known_folder_path(id: *const GUID) -> Result<String> {
  let slice;
  unsafe {
    let PWSTR(path) = SHGetKnownFolderPath(id, KNOWN_FOLDER_FLAG(0), None)?;
    let mut char_ptr = path;
    let mut len = 0;
    while char_ptr.read() != 0 {
      char_ptr = char_ptr.offset(1);
      len += 1;
    }
    slice = slice::from_raw_parts(path, len);
    CoTaskMemFree(Some(path as *const c_void));
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
    CoInitialize(None)?;
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
  // https://github.com/rust-lang/rust/issues/100884
  unsafe {
    Console::SetStdHandle(Console::STD_INPUT_HANDLE, HANDLE(0)).as_bool()
      && Console::SetStdHandle(Console::STD_OUTPUT_HANDLE, HANDLE(0)).as_bool()
      && Console::SetStdHandle(Console::STD_ERROR_HANDLE, HANDLE(0)).as_bool()
      && Console::FreeConsole().as_bool()
  }
}

pub fn alloc_console() -> bool {
  unsafe { Console::FreeConsole().as_bool() && Console::AllocConsole().as_bool() }
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

pub fn ansi_buffer_to_string(buf: &Vec<u8>) -> Result<String> {
  let size = unsafe { MultiByteToWideChar(CP_ACP, MULTI_BYTE_TO_WIDE_CHAR_FLAGS(0), buf, None) };
  if size == 0 {
    return Err(std::io::Error::last_os_error())?;
  }
  let mut new_buf = Vec::with_capacity(size as usize);
  unsafe {
    new_buf.set_len(size as usize);
    if MultiByteToWideChar(
      CP_ACP,
      MULTI_BYTE_TO_WIDE_CHAR_FLAGS(0),
      buf,
      Some(&mut new_buf),
    ) == 0
    {
      return Err(std::io::Error::last_os_error())?;
    }
  }
  let mut result = OsString::from_wide(&new_buf).into_string().unwrap();
  result.pop();
  Ok(result)
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
  fn test_ansi_to_string() {
    let out = std::process::Command::new("cmd").arg("/?").output().unwrap().stdout;
    ansi_buffer_to_string(&out).unwrap();
  }
}
