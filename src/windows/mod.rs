use napi::bindgen_prelude::FromNapiValue;
use windows::{
  core::{w, Error, HRESULT, HSTRING, PCWSTR},
  Win32::System::Registry::{
    RegGetValueW, RegOpenKeyExW, HKEY, HKEY_CLASSES_ROOT, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
    KEY_ALL_ACCESS, KEY_READ, REG_SZ, REG_VALUE_TYPE, RRF_RT_REG_BINARY, RRF_RT_REG_DWORD,
    RRF_RT_REG_EXPAND_SZ, RRF_RT_REG_MULTI_SZ, RRF_RT_REG_QWORD, RRF_RT_REG_SZ,
  },
};

const BUFFRE_SIZE: usize = 1024;

pub enum HkeyMap {
  HKCU,
  HKLM,
  HKCR,
}

impl FromNapiValue for HkeyMap {

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
    None,
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

  let result: Result<Option<RegValueResult>, Error> = unsafe {
    let mut hkey = HKEY::default();
    let mut pdwtype: REG_VALUE_TYPE = Default::default();
    let (lpsubkey, _) = reg_path.into_pcwstr();
    let (lpvalue, _) = reg_key_name.into_pcwstr();

    RegOpenKeyExW(reg_key_root, lpsubkey, 0, KEY_READ, &mut hkey as *mut _)?;

    match dwflags {
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
        Ok(Some(RegValueResult::Str(String::from_utf16_lossy(&pvdata[..len].to_vec()))))
      },
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
        Ok(Some(RegValueResult::VecU8(pvdata[..pcbdata as usize].to_vec())))
      },
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
      },
      _ => Err(Error::new(HRESULT(0), HSTRING::default())),
    }
  };

  result
}
