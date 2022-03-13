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

#![cfg(target_os = "windows")]

use std::alloc::{alloc, dealloc, Layout};
use std::ffi::{c_void, OsStr, OsString};
use std::io;
use std::os::windows::prelude::*;
use std::ptr;
use std::slice;

use windows::core::{Interface, GUID};
use windows::Win32::Foundation::PWSTR;
use windows::Win32::Globalization::GetACP;
use windows::Win32::System::Com::{
  CoCreateInstance, CoInitialize, CoTaskMemFree, CoUninitialize, IPersistFile, CLSCTX_INPROC_SERVER,
};
use windows::Win32::System::Environment::ExpandEnvironmentStringsW;
use windows::Win32::UI::Shell::{IShellLinkW, SHGetKnownFolderPath, ShellLink};

pub static CREATE_NO_WINDOW: u32 = windows::Win32::System::Threading::CREATE_NO_WINDOW.0;

pub fn expand_environment_strings(src: &str) -> io::Result<String> {
  let src = OsStr::new(src);

  // Get buffer size
  let size = unsafe { ExpandEnvironmentStringsW(src, PWSTR(ptr::null_mut()), 0) };
  let layout = Layout::array::<u16>(size as usize).map_err(|_| {
    io::Error::new(
      io::ErrorKind::Other,
      "Overflow during calculate alloc layout",
    )
  })?;
  let result: String;

  unsafe {
    let buf = alloc(layout) as *mut u16;
    let len = match ExpandEnvironmentStringsW(src, PWSTR(buf), size) {
      0 => return Err(io::Error::last_os_error()),
      x => x,
    };
    result = OsString::from_wide(slice::from_raw_parts(buf, (len - 1) as usize))
      .into_string()
      .unwrap();
    dealloc(buf as *mut u8, layout);
  }
  Ok(result)
}

pub fn get_known_folder_path(id: *const GUID) -> io::Result<String> {
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
  os_string.into_string().map_err(|_| {
    io::Error::new(
      io::ErrorKind::Other,
      "Failed to convert wide string to string",
    )
  })
}

pub fn get_acp() -> u32 {
  unsafe { GetACP() }
}

pub fn create_lnk(lnk: &str, target: &str, desc: &str, args: &str) -> io::Result<()> {
  let lnk = OsStr::new(lnk);
  let target = OsStr::new(target);
  let desc = OsStr::new(desc);
  let args = OsStr::new(args);
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

pub fn alloc_console() -> bool {
  unsafe { windows::Win32::System::Console::AllocConsole() }.0 != 0
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
}
