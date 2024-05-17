use windows::{
  core::{Error, PCWSTR},
  Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
      FindWindowW, SetWindowPos, ShowWindow, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SW_RESTORE,
      SW_SHOW,
    },
  },
};
use windows_registry::*;

pub enum HkeyMap {
  HKCU,
  HKLM,
  HKCR,
}

#[derive(Debug)]
pub enum RegValueResult {
  Int(i32),
  Str(String),
  VecStr(Vec<String>),
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
) -> Result<Option<RegValueResult>> {
  let reg_key_root = match reg_key_root {
    HkeyMap::HKCU => CURRENT_USER,
    HkeyMap::HKLM => LOCAL_MACHINE,
    HkeyMap::HKCR => CLASSES_ROOT,
  };

  let key = reg_key_root.open(reg_path)?;

  let value = key.get_value(reg_key_name)?;

  match value {
    Value::U32(num) => Ok(Some(RegValueResult::Int(num as i32))),
    Value::U64(num) => Ok(Some(RegValueResult::Int(num as i32))),
    Value::Bytes(bytes) => Ok(Some(RegValueResult::VecU8(bytes))),
    Value::MultiString(vec_str) => Ok(Some(RegValueResult::VecStr(vec_str))),
    Value::String(str) => Ok(Some(RegValueResult::Str(str))),
    Value::Unknown(unknown) => {
      println!("Unknown: {:?}", unknown);
      Ok(Some(RegValueResult::Null))
    }
  }
}

pub fn write_registry(
  reg_key_root: HkeyMap,
  reg_path: &str,
  reg_key_name: &str,
  reg_key_value: RegValueResult,
) -> Result<()> {
  let reg_key_root = match reg_key_root {
    HkeyMap::HKCU => CURRENT_USER,
    HkeyMap::HKLM => LOCAL_MACHINE,
    HkeyMap::HKCR => CLASSES_ROOT,
  };

  let key = reg_key_root.create(reg_path)?;

  match reg_key_value {
    RegValueResult::Str(str) => {
      key.set_string(reg_key_name, &str)?;
    }
    RegValueResult::Int(num) => {
      key.set_u32(reg_key_name, num as u32)?;
    }
    RegValueResult::VecStr(vec_str) => {
      let v = vec_str.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
      key.set_multi_string(reg_key_name, &v)?;
    }
    RegValueResult::VecU8(vec_u8) => {
      key.set_bytes(reg_key_name, &vec_u8)?;
    }
    RegValueResult::Null => {
      // noop
    }
  };

  Ok(())
}

pub fn delete_registry(reg_key_root: HkeyMap, reg_path: &str, reg_key_name: &str) -> Result<()> {
  let reg_key_root = match reg_key_root {
    HkeyMap::HKCU => CURRENT_USER,
    HkeyMap::HKLM => LOCAL_MACHINE,
    HkeyMap::HKCR => CLASSES_ROOT,
  };

  let key = reg_key_root.create(reg_path)?;

  key.remove_value(reg_key_name)?;

  Ok(())
}

/**
 * 强制窗体显示
 * TODO: 窗体可以响应点击事件
 */
pub fn show_window_force(window_title: &str) -> std::result::Result<(), Error> {
  unsafe {
    let (pcwstr, _) = window_title.into_pcwstr();
    let hwnd = FindWindowW(None, pcwstr);
    println!("hwnd: {:?}", hwnd);
    SetWindowPos(
      hwnd,
      HWND(-1),
      0,
      0,
      0,
      0,
      SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
    )?;
    let _ = ShowWindow(hwnd, SW_RESTORE);
    let _ = ShowWindow(hwnd, SW_RESTORE);
    let _ = ShowWindow(hwnd, SW_SHOW);
    let _ = ShowWindow(hwnd, SW_SHOW);
    SetWindowPos(
      hwnd,
      HWND(-2),
      0,
      0,
      0,
      0,
      SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
    )?;
  }
  Ok(())
}
