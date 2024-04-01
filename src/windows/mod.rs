use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

use windows::{
  core::{Error, HRESULT, HSTRING, PCWSTR},
  Win32::{
    Foundation::HWND,
    System::Registry::{
      RegCloseKey, RegDeleteKeyValueW, RegGetValueW, RegOpenKeyExW, RegSetValueExW, HKEY, HKEY_CLASSES_ROOT, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS, KEY_READ, KEY_WRITE, REG_BINARY, REG_DWORD, REG_EXPAND_SZ, REG_MULTI_SZ, REG_SZ, REG_VALUE_TYPE, RRF_RT_REG_BINARY, RRF_RT_REG_DWORD, RRF_RT_REG_EXPAND_SZ, RRF_RT_REG_MULTI_SZ, RRF_RT_REG_SZ
    },
    UI::WindowsAndMessaging::{
      FindWindowW, SetWindowPos, ShowWindow, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SW_RESTORE,
      SW_SHOW,
    },
  },
};

const BUFFRE_SIZE: usize = 1024;

pub enum HkeyMap {
  HKCU,
  HKLM,
  HKCR,
}

pub enum RegType {
  RegSz,
  RegExpandSz,
  RegMultiSz,
  RegBinary,
  RegDword,
}

#[derive(Debug)]
pub enum RegValueResult {
  Int(i32),
  Str(String),
  VecU8(Vec<u8>),
  Null,
}

trait IntoPCWSTR {
  fn into_pcwstr(self) -> (PCWSTR, Vec<u16>);
}

impl IntoPCWSTR for &str {
  fn into_pcwstr(self) -> (PCWSTR, Vec<u16>) {
    let mut encoded = self.encode_utf16().chain([0u16]).collect::<Vec<u16>>();

    (PCWSTR(encoded.as_mut_ptr()), encoded)
  }
}

/**
 * Read registry value
 */
pub fn read_registry(
  reg_key_root: HkeyMap,
  reg_path: &str,
  reg_key_name: &str,
  reg_key_value_type: RegType,
) -> Result<Option<RegValueResult>, Error> {
  let reg_key_root = match reg_key_root {
    HkeyMap::HKCU => HKEY_CURRENT_USER,
    HkeyMap::HKLM => HKEY_LOCAL_MACHINE,
    HkeyMap::HKCR => HKEY_CLASSES_ROOT,
  };

  let dwflags = match reg_key_value_type {
    RegType::RegSz => RRF_RT_REG_SZ,
    RegType::RegExpandSz => RRF_RT_REG_EXPAND_SZ,
    RegType::RegMultiSz => RRF_RT_REG_MULTI_SZ,
    RegType::RegBinary => RRF_RT_REG_BINARY,
    RegType::RegDword => RRF_RT_REG_DWORD,
  };

  unsafe {
    let mut hkey = HKEY::default();
    let mut pdwtype: REG_VALUE_TYPE = Default::default();
    let (lpsubkey, _) = reg_path.into_pcwstr();
    let (lpvalue, _) = reg_key_name.into_pcwstr();

    RegOpenKeyExW(reg_key_root, lpsubkey, 0, KEY_READ, &mut hkey as *mut _)?;

    let res = match dwflags {
      RRF_RT_REG_SZ | RRF_RT_REG_EXPAND_SZ | RRF_RT_REG_MULTI_SZ => {
        let mut pvdata = [0u16; BUFFRE_SIZE * 4];
        let mut pcbdata = (BUFFRE_SIZE * 4 * std::mem::size_of_val(&pvdata[0])) as u32;
        RegGetValueW(
          hkey,
          None,
          lpvalue,
          dwflags,
          Some(&mut pdwtype),
          Some(pvdata.as_mut_ptr() as *mut _),
          Some(&mut pcbdata),
        )?;
        let len = (pcbdata as usize - 1) / 2;
        Ok(Some(RegValueResult::Str(String::from_utf16_lossy(
          &pvdata[..len].to_vec(),
        ))))
      }
      RRF_RT_REG_BINARY => {
        let mut pvdata = [0u8; BUFFRE_SIZE];
        let mut pcbdata = (BUFFRE_SIZE * std::mem::size_of_val(&pvdata[0])) as u32;
        RegGetValueW(
          hkey,
          None,
          lpvalue,
          dwflags,
          Some(&mut pdwtype),
          Some(pvdata.as_mut_ptr() as *mut _),
          Some(&mut pcbdata),
        )?;
        Ok(Some(RegValueResult::VecU8(
          pvdata[..pcbdata as usize].to_vec(),
        )))
      }
      RRF_RT_REG_DWORD => {
        let mut pvdata = [0u8; 4];
        let mut pcbdata = pvdata.len() as u32;
        RegGetValueW(
          hkey,
          None,
          lpvalue,
          dwflags,
          Some(&mut pdwtype),
          Some(pvdata.as_mut_ptr() as *mut _),
          Some(&mut pcbdata),
        )?;
        Ok(Some(RegValueResult::Int(i32::from_le_bytes(pvdata))))
      }
      _ => Err(Error::new(HRESULT(0), HSTRING::default())),
    };

    RegCloseKey(hkey)?;

    res
  }
}

pub fn write_registry(
  reg_key_root: HkeyMap,
  reg_path: &str,
  reg_key_name: &str,
  reg_key_value_type: RegType,
  reg_key_value: RegValueResult,
) -> Result<(), Error> {
  let reg_key_root = match reg_key_root {
    HkeyMap::HKCU => HKEY_CURRENT_USER,
    HkeyMap::HKLM => HKEY_LOCAL_MACHINE,
    HkeyMap::HKCR => HKEY_CLASSES_ROOT,
  };

  let dw_type = match reg_key_value_type {
    RegType::RegSz => REG_SZ,
    RegType::RegExpandSz => REG_EXPAND_SZ,
    RegType::RegMultiSz => REG_MULTI_SZ,
    RegType::RegBinary => REG_BINARY,
    RegType::RegDword => REG_DWORD,
  };

  unsafe {
    let mut hkey = HKEY::default();
    let (lpsubkey, _) = reg_path.into_pcwstr();
    let (lpvaluename, _) = reg_key_name.into_pcwstr();

    RegOpenKeyExW(reg_key_root, lpsubkey, 0, KEY_WRITE, &mut hkey as *mut _)?;

    let res = match dw_type {
      REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
        if let RegValueResult::Str(val) = reg_key_value {
          let val = OsStr::new(&val)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect::<Vec<_>>();
          let val = val.align_to::<u8>().1;
          RegSetValueExW(hkey, lpvaluename, 0u32, dw_type, Some(val))?;
          Ok(())
        } else {
          return Err(Error::new(HRESULT(1), HSTRING::default()));
        }
      }
      REG_BINARY => {
        if let RegValueResult::VecU8(val) = reg_key_value {
          RegSetValueExW(hkey, lpvaluename, 0u32, dw_type, Some(val.as_ref()))?;
          Ok(())
        } else {
          return Err(Error::new(HRESULT(3), HSTRING::default()));
        }
      }
      REG_DWORD => {
        if let RegValueResult::Int(val) = reg_key_value {
          RegSetValueExW(
            hkey,
            lpvaluename,
            0u32,
            dw_type,
            Some(val.to_ne_bytes().as_ref()),
          )?;
          Ok(())
        } else {
          return Err(Error::new(HRESULT(4), HSTRING::default()));
        }
      }
      _ => Err(Error::new(HRESULT(0), HSTRING::default())),
    };

    RegCloseKey(hkey)?;

    res
  }
}

pub fn delete_registry(
  reg_key_root: HkeyMap,
  reg_path: &str,
  reg_key_name: &str,
) -> Result<(), Error> {
  let reg_key_root = match reg_key_root {
    HkeyMap::HKCU => HKEY_CURRENT_USER,
    HkeyMap::HKLM => HKEY_LOCAL_MACHINE,
    HkeyMap::HKCR => HKEY_CLASSES_ROOT,
  };

  unsafe {
    let mut hkey = HKEY::default();
    let (lpsubkey, _) = reg_path.into_pcwstr();
    let (lpvaluename, _) = reg_key_name.into_pcwstr();

    RegOpenKeyExW(reg_key_root, lpsubkey, 0, KEY_ALL_ACCESS, &mut hkey as *mut _)?;

    RegDeleteKeyValueW(hkey, lpsubkey, lpvaluename)?;

    RegCloseKey(hkey)?;

    Ok(())
  }
}

/**
 * 强制窗体显示
 * TODO: 窗体可以响应点击事件
 */
pub fn show_window_force(window_title: &str) {
  unsafe {
    let (pcwstr, _) = window_title.into_pcwstr();
    let hwnd = FindWindowW(None, pcwstr);
    println!("hwnd: {:?}", hwnd);
    let _ = SetWindowPos(
      hwnd,
      HWND(-1),
      0,
      0,
      0,
      0,
      SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
    )
    .is_ok();
    ShowWindow(hwnd, SW_RESTORE);
    ShowWindow(hwnd, SW_RESTORE);
    ShowWindow(hwnd, SW_SHOW);
    ShowWindow(hwnd, SW_SHOW);
    let _ = SetWindowPos(
      hwnd,
      HWND(-2),
      0,
      0,
      0,
      0,
      SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
    )
    .is_ok();
  }
}
