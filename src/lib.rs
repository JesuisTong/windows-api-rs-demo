#![deny(clippy::all)]
#![allow(clippy::upper_case_acronyms, clippy::enum_variant_names)]
use napi::{
  assert_type_of,
  bindgen_prelude::{Buffer, FromNapiValue, Null, ToNapiValue},
  JsBuffer, JsNumber, JsObject, JsString, JsUnknown, NapiRaw, NapiValue, Result, ValueType,
};
use napi_derive::napi;
use regex::Regex;
use sysinfo::{ProcessRefreshKind, System};

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod windows;

#[napi]
pub fn get_process_exists(process_name: String, exact: bool) -> bool {
  let mut sys = System::new();
  sys.refresh_processes_specifics(ProcessRefreshKind::new());
  if exact {
    sys.processes_by_exact_name(&process_name).count() != 0
  } else {
    let str = format!(r"(?i){}", &process_name);
    let reg = Regex::new(&str).unwrap();

    sys
      .processes()
      .values()
      .filter(move |val| reg.is_match(val.name()))
      .count()
      != 0
  }
}

#[napi]
pub fn kill_process(process_name: String) {
  let mut sys = System::new();
  sys.refresh_processes_specifics(ProcessRefreshKind::new());
  sys.processes_by_exact_name(&process_name).for_each(|p| {
    p.kill();
  });
}

#[cfg(target_os = "windows")]
impl FromNapiValue for windows::HkeyMap {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> Result<Self> {
    let num = JsNumber::from_napi_value(env, napi_val)?;
    let num = num.get_int32()?;
    match num {
      0 => Ok(windows::HkeyMap::HKCU),
      1 => Ok(windows::HkeyMap::HKLM),
      2 => Ok(windows::HkeyMap::HKCR),
      _ => Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("invalid Hkey {num}"),
      )),
    }
  }
}

#[cfg(target_os = "windows")]
impl FromNapiValue for windows::RegType {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> Result<Self> {
    let num = JsNumber::from_napi_value(env, napi_val)?;
    let num = num.get_int32()?;
    match num {
      0 => Ok(windows::RegType::RegSz),
      1 => Ok(windows::RegType::RegExpandSz),
      // 2 => Ok(windows::RegType::RegMultiSz),
      3 => Ok(windows::RegType::RegBinary),
      4 => Ok(windows::RegType::RegDword),
      _ => Err(napi::Error::new(
        napi::Status::InvalidArg,
        format!("invalid RegType {num}"),
      )),
    }
  }
}

#[cfg(target_os = "windows")]
impl FromNapiValue for windows::RegValueResult {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> Result<Self> {
    let is_i32 = assert_type_of!(env, napi_val, ValueType::Number);
    let is_str = assert_type_of!(env, napi_val, ValueType::String);
    let is_obj = assert_type_of!(env, napi_val, ValueType::Object);
    let is_unknown = assert_type_of!(env, napi_val, ValueType::Unknown);
    let is_null = assert_type_of!(env, napi_val, ValueType::Null);

    if let Ok(()) = is_i32 {
      let num = JsNumber::from_napi_value(env, napi_val)?;
      return Ok(windows::RegValueResult::Int(num.get_int32()?));
    }
    if let Ok(()) = is_str {
      let str = JsString::from_napi_value(env, napi_val)?;
      return Ok(windows::RegValueResult::Str(
        str.into_utf8()?.as_str()?.to_string(),
      ));
    }
    if let Ok(()) = is_null {
      return Ok(windows::RegValueResult::Null);
    }
    if let Ok(()) = is_obj {
      let obj = JsObject::from_napi_value(env, napi_val)?;
      let bool = obj.is_buffer()?;
      if bool {
        let v = JsBuffer::from_raw(env, obj.raw())?;
        return Ok(windows::RegValueResult::VecU8(v.into_value()?.to_vec()));
      }
      return Err(napi::Error::new(
        napi::Status::InvalidArg,
        "invalid RegValueResult",
      ));
    }
    if let Ok(()) = is_unknown {
      let unknown = JsUnknown::from_napi_value(env, napi_val)?;
      match unknown.get_type()? {
        napi::ValueType::Object => {
          let v = unknown.cast::<napi::JsBuffer>();
          return Ok(windows::RegValueResult::VecU8(v.into_value()?.to_vec()));
        }
        _ => {
          return Err(napi::Error::new(
            napi::Status::InvalidArg,
            "invalid RegValueResult",
          ));
        }
      }
    }
    Err(napi::Error::new(
      napi::Status::InvalidArg,
      "invalid RegValueResult",
    ))
  }
}

#[cfg(target_os = "windows")]
impl ToNapiValue for windows::RegValueResult {
  unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> Result<napi::sys::napi_value> {
    match val {
      windows::RegValueResult::Int(num) => unsafe { ToNapiValue::to_napi_value(env, num) },
      windows::RegValueResult::Str(str) => unsafe { ToNapiValue::to_napi_value(env, str) },
      windows::RegValueResult::VecU8(vec_u8) => unsafe {
        Buffer::to_napi_value(env, Buffer::from(vec_u8))
      },
      windows::RegValueResult::Null => unsafe { Null::to_napi_value(env, Null) },
    }
  }
}

#[cfg(target_os = "windows")]
#[napi(ts_return_type = "unknown")]
pub fn read_registry(
  #[napi(ts_arg_type = "0|1|2")] reg_key_root: windows::HkeyMap,
  reg_path: String,
  reg_key_name: String,
  #[napi(ts_arg_type = "0|1|2|3|4")] reg_key_value_type: windows::RegType,
) -> Result<windows::RegValueResult> {
  match windows::read_registry(reg_key_root, &reg_path, &reg_key_name, reg_key_value_type) {
    Ok(result) => {
      if let Some(result) = result {
        return Ok(result);
      }
      Ok(windows::RegValueResult::Null)
    }
    Err(err) => {
      eprintln!("read_registry error: {:?}", err.message().to_string());
      Ok(windows::RegValueResult::Null)
    }
  }
}

#[cfg(target_os = "windows")]
#[napi]
pub fn write_registry(
  #[napi(ts_arg_type = "0|1|2")] reg_key_root: windows::HkeyMap,
  reg_path: String,
  reg_key_name: String,
  #[napi(ts_arg_type = "0|1|2|3|4")] _reg_key_value_type: windows::RegType,
  #[napi(ts_arg_type = "unknown")] reg_key_value: windows::RegValueResult,
) -> Result<()> {
  windows::write_registry(
    reg_key_root,
    &reg_path,
    &reg_key_name,
    reg_key_value,
  )
  .map_err(|err| {
    napi::Error::new(
      napi::Status::FunctionExpected,
      format!("write_registry error: {:?}", err.message().to_string()),
    )
  })
}

#[cfg(target_os = "windows")]
#[napi]
pub fn delete_registry(
  #[napi(ts_arg_type = "0|1|2")] reg_key_root: windows::HkeyMap,
  reg_path: String,
  reg_key_name: String,
) -> Result<()> {
  windows::delete_registry(reg_key_root, &reg_path, &reg_key_name).map_err(|err| {
    napi::Error::new(
      napi::Status::FunctionExpected,
      format!("delete_registry error: {:?}", err.message().to_string()),
    )
  })
}

#[cfg(target_os = "windows")]
#[napi]
pub fn show_window_by_title(window_title: String) -> Result<()> {
  windows::show_window_force(&window_title).map_err(|err| {
    napi::Error::new(
      napi::Status::FunctionExpected,
      format!(
        "show_window_by_title error: {:?}",
        err.message().to_string()
      ),
    )
  })
}
