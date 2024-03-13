#![deny(clippy::all)]
use napi_derive::napi;
use sysinfo::{ProcessRefreshKind, System};

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
