#![deny(clippy::all)]
use napi_derive::napi;
use napi::Env;
use sysinfo::{ProcessRefreshKind, System};

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod windows;

#[napi]
pub fn get_process_exists(process_name: String) -> bool {
  let mut sys = System::new();
  sys.refresh_processes_specifics(ProcessRefreshKind::new());
  sys.processes_by_exact_name(&process_name).count() != 0
}

// redefine https://github.com/napi-rs/napi-rs/issues/1463
#[napi]
pub enum HkeyMap {
    HKCU,
    HKLM,
    HKCR,
}

// redefine https://github.com/napi-rs/napi-rs/issues/1463
#[napi]
pub enum RegType {
    RegSz,
    RegExpandSz,
    RegMultiSz,
    RegBinary,
    RegDword,
}

#[cfg(target_os = "windows")]
#[napi]
pub fn read_registry(
  env: Env,
  #[napi(ts_arg_type = "HkeyMap")]
  reg_key_root: i32,
  reg_path: String,
  reg_key_name: String,
  #[napi(ts_arg_type = "RegType")]
  reg_key_value_type: i32
) -> napi::Result<napi::JsUnknown> {
  let reg_key_root = match reg_key_root {
    0 => windows::HkeyMap::HKCU,
    1 => windows::HkeyMap::HKLM,
    2 => windows::HkeyMap::HKCR,
    _ => windows::HkeyMap::HKLM,
  };
  let reg_key_value_type = match reg_key_value_type {
    0 => windows::RegType::RegSz,
    1 => windows::RegType::RegExpandSz,
    2 => windows::RegType::RegMultiSz,
    3 => windows::RegType::RegBinary,
    4 => windows::RegType::RegDword,
    _ => windows::RegType::RegSz,
  };

  match windows::read_registry(reg_key_root, &reg_path, &reg_key_name, reg_key_value_type) {
    Ok(value) => {
      if let Some(value) = value {
        match value {
          windows::RegValueResult::Int(val) => env.create_int32(val).map(|v| v.into_unknown()),
          windows::RegValueResult::Str(val) => env.create_string(&val).map(|v| v.into_unknown()),
          windows::RegValueResult::VecU8(val) => env.create_buffer_with_data(val).map(|v| v.into_unknown()),
        }
      } else {
        env.get_null().map(|v| v.into_unknown())
      }
    },
    Err(err) => {
      println!("Error: {:?}", err.message());
      env.get_null().map(|v| v.into_unknown())
    },
  }
}

#[cfg(target_os = "windows")]
#[napi]
pub fn write_registry(
  #[napi(ts_arg_type = "HkeyMap")]
  reg_key_root: i32,
  reg_path: String,
  reg_key_name: String,
  #[napi(ts_arg_type = "RegType")]
  reg_key_value_type: i32,
  #[napi(ts_arg_type = "unknown")]
  reg_key_value: napi::JsUnknown
) -> napi::Result<()> {
  let reg_key_root = match reg_key_root {
    0 => windows::HkeyMap::HKCU,
    1 => windows::HkeyMap::HKLM,
    2 => windows::HkeyMap::HKCR,
    _ => windows::HkeyMap::HKLM,
  };
  let reg_key_value_type = match reg_key_value_type {
    0 => windows::RegType::RegSz,
    1 => windows::RegType::RegExpandSz,
    2 => windows::RegType::RegMultiSz,
    3 => windows::RegType::RegBinary,
    4 => windows::RegType::RegDword,
    _ => windows::RegType::RegSz,
  };

  unsafe {
    match reg_key_value.get_type()? {
      napi::ValueType::Number => {
        let val = reg_key_value.cast::<napi::JsNumber>().get_int32()?;
        windows::write_registry(reg_key_root, &reg_path, &reg_key_name, reg_key_value_type, windows::RegValueResult::Int(val)).map_err(|err| {
          napi::Error::from_reason(format!("write registry number error: {:?}", err.message().to_string_lossy()))
        })
      },
      napi::ValueType::String => {
        let val = reg_key_value.cast::<napi::JsString>().into_utf8()?;
        windows::write_registry(reg_key_root, &reg_path, &reg_key_name, reg_key_value_type, windows::RegValueResult::Str(val.as_str()?.to_string())).map_err(|err| {
          napi::Error::from_reason(format!("write registry string error: {:?}", err.message().to_string_lossy()))
        })
      },
      napi::ValueType::Object => {
        if let Ok(bol) = reg_key_value.is_buffer() {
          if bol {
            let buffer = reg_key_value.cast::<napi::JsBuffer>();
            windows::write_registry(reg_key_root, &reg_path, &reg_key_name, reg_key_value_type, windows::RegValueResult::VecU8(buffer.into_value()?.to_vec())).map_err(|err| {
              napi::Error::from_reason(format!("write registry buffer error: {:?}", err.message().to_string_lossy()))
            })
          } else {
            Err(napi::Error::from_reason("write registry buffer is not buffer object"))
          }
        } else {
          Err(napi::Error::from_reason("write registry buffer is not buffer object"))
        }
      }
      _ => {
        Err(napi::Error::from_reason("unknown error"))
      }
    }
  }
}

#[cfg(target_os = "windows")]
#[napi]
pub fn show_window_by_title(window_title: String) {
  windows::show_window_force(&window_title);
}
