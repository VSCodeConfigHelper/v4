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
use std::ffi::{OsStr, OsString};
use std::io;
use std::os::windows::prelude::*;
use std::ptr;
use std::slice;

use winapi::ctypes::c_void;
use winapi::shared::ntdef::*;
use winapi::shared::winerror::S_OK;
use winapi::um::combaseapi::CoTaskMemFree;
pub use winapi::um::knownfolders::*;
use winapi::um::objbase::CoInitialize;
use winapi::um::processenv::ExpandEnvironmentStringsW;
use winapi::um::shlobj::SHGetKnownFolderPath;
use winapi::um::shtypes::REFKNOWNFOLDERID;
pub use winapi::um::winbase::CREATE_NO_WINDOW;
use winapi::um::winnls::GetACP;
use winapi::um::shobjidl_core::IShellLinkW;

pub fn expand_environment_strings(src: &str) -> io::Result<String> {
  // Make src null-terminated UTF-16
  let mut src: Vec<u16> = OsStr::new(src).encode_wide().collect();
  src.push(0);

  // Get buffer size
  let size = unsafe { ExpandEnvironmentStringsW(src.as_ptr(), ptr::null_mut(), 0) };
  let layout = Layout::array::<u16>(size as usize).map_err(|_| {
    io::Error::new(
      io::ErrorKind::Other,
      "Overflow during calculate alloc layout",
    )
  })?;
  let result: String;

  unsafe {
    let buf = alloc(layout) as *mut u16;
    let len = match ExpandEnvironmentStringsW(src.as_ptr(), buf, size) {
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

pub fn get_known_folder_path(id: REFKNOWNFOLDERID) -> io::Result<String> {
  struct KnownFolderPath(PWSTR);
  impl Drop for KnownFolderPath {
    fn drop(&mut self) {
      unsafe {
        CoTaskMemFree(self.0 as *mut c_void);
      }
    }
  }

  let slice = unsafe {
    let mut path = KnownFolderPath(ptr::null_mut());
    match SHGetKnownFolderPath(id, 0, ptr::null_mut(), &mut path.0) {
      S_OK => {
        let mut wide_string = path.0;
        let mut len = 0;
        while wide_string.read() != 0 {
          wide_string = wide_string.offset(1);
          len += 1;
        }
        slice::from_raw_parts(path.0, len)
      }
      _ => return Err(io::Error::last_os_error()),
    }
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

pub fn create_lnk(lnk: &str, target: &str, desc: &str, args: &str) -> bool {
  unsafe {
    let result = CoInitialize(ptr::null_mut());
    if result <= 0 { return false; }
    let shell_link: *mut IShellLinkW = ptr::null_mut();
    // FUCK YOU, winapi missing defs. use windows-rs instead.
    //let result = CoCreateInstance(&CLSID_ShellLink, ptr::null_mut(), CLSCTX_INPROC_SERVER, IID_ISHellLink, shell_link);
  }
  false
}

#[cfg(test)]
mod tests {
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
