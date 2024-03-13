#![deny(clippy::all)]

use napi_derive::napi;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn get_process_exsits(process_name: String) -> bool {
  platform::get_process_id_by_name(&process_name) != 0
}
