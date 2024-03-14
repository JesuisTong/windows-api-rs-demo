use windows::{
  core::{w, PCWSTR},
  Win32::System::Registry::{
    RegGetValueW, RegOpenKeyExW, HKEY, HKEY_CLASSES_ROOT, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
    KEY_ALL_ACCESS, KEY_READ, REG_SZ, REG_VALUE_TYPE, RRF_RT_REG_BINARY, RRF_RT_REG_DWORD,
    RRF_RT_REG_EXPAND_SZ, RRF_RT_REG_MULTI_SZ, RRF_RT_REG_NONE, RRF_RT_REG_QWORD, RRF_RT_REG_SZ,
  },
};

pub enum HkeyMap {
  HKCU,
  HKLM,
  HKCR,
}

pub enum RegType {
  RegNone,
  RegSz,
  RegExpandSz,
  RegMultiSz,
  RegBinary,
  RegDword,
  RegQword,
}

/**
 * Read registry value
 */
pub fn read_registry<T>(
  reg_key_root: HkeyMap,
  reg_path: &str,
  reg_key_name: &str,
  reg_key_value_type: RegType,
) -> Result<Option<T>, Error> {
  let reg_key_root = match reg_key_root {
    HkeyMap::HKCU => HKEY_CURRENT_USER,
    HkeyMap::HKLM => HKEY_LOCAL_MACHINE,
    HkeyMap::HKCR => HKEY_CLASSES_ROOT,
    _ => HKEY_LOCAL_MACHINE,
  };

  let dwflags = match reg_key_value_type {
    RegType::RegNone => RRF_RT_REG_NONE,
    RegType::RegSz => RRF_RT_REG_SZ,
    RegType::RegExpandSz => RRF_RT_REG_EXPAND_SZ,
    RegType::RegMultiSz => RRF_RT_REG_MULTI_SZ,
    RegType::RegBinary => RRF_RT_REG_BINARY,
    RegType::RegDword => RRF_RT_REG_DWORD,
    RegType::RegQword => RRF_RT_REG_QWORD,
  };

  let result = unsafe {
    let mut hkey = HKEY::default();
    let mut pdwtype: REG_VALUE_TYPE = Default::default();
    let lpsubkey = PCWSTR(reg_path.as_ptr() as *const u16);
    let lpvalue = PCWSTR(reg_key_name.as_ptr() as *const u16);
    RegOpenKeyExW(reg_key_root, lpsubkey, 0, KEY_READ, &mut hkey as *mut _)?;
    match dwflags {
      RRF_RT_REG_SZ | RRF_RT_REG_EXPAND_SZ | RRF_RT_REG_MULTI_SZ => {
        let mut pvdata = [0u16; 1024 * 4];
        let mut pcbdata = 1024 * 4 * std::mem::size_of_val(&pvdata[0]) as u32;
        let ret = RegGetValueW(
          hkey,
          None,
          lpvalue,
          dwflags,
          Some(&mut pdwtype),
          Some(pvdata.as_mut_ptr() as *mut _),
          Some(&mut pcbdata),
        );
        if ret.is_err() {
          return Ok(None);
        }
        pvdata.
      }
    }
    RegGetValueW(
      hkey,
      None,
      lpvalue,
      dwflags,
      Some(&mut pdwtype),
      Some(pvdata.as_mut_ptr() as *mut _),
      Some(&mut pcbdata),
    )?;
    // let mut pdwtype = REG_VALUE_TYPE(0);
    // let mut pvdata;
    // let mut pcbdata;
    // RegGetValueW(
    //   hkey,
    //   PCWSTR(reg_path.as_ptr() as *const u16),
    //   PCWSTR(value_name.as_ptr() as *const u16),
    //   dw_flags,
    //   &mut pdwtype,
    //   &mut pvdata,
    //   &mut pcbdata,
    // )
  };
  // if result != 0 {
  //   panic!("Failed to open registry key");
  // }
  0
}
