#![deny(clippy::all)]
use napi_derive::napi;
use sysinfo::{ProcessRefreshKind, System};

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod windows;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn get_process_exsits(process_name: String) -> bool {
  let mut sys = System::new();
  sys.refresh_processes_specifics(ProcessRefreshKind::new());
  sys.processes_by_exact_name(&process_name).count() != 0
}

#[cfg(target_os = "windows")]
#[napi]
pub fn read_registry(reg_key_root: windows::HkeyMap, reg_path: String, reg_key_name: String, reg_key_value_type: windows::RegType) {

  windows::read_registry(reg_key_root, &reg_path, &reg_key_name, reg_key_value_type);
  windows::read_registry()
}
