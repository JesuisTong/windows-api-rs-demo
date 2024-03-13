use windows::Win32::{
  Foundation::{CloseHandle, MAX_PATH}, Globalization::{MultiByteToWideChar, CP_ACP, MULTI_BYTE_TO_WIDE_CHAR_FLAGS}, System::{Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS
  }, WindowsProgramming::uaw_wcsicmp}
};


fn str2wstring(s: &str) -> Vec<u16> {
  s.encode_utf16().chain(Some(0)).collect()
}

fn wstring_c_str(s: Vec<u16>) -> *const u16 {
  s.as_ptr()
}



pub fn get_process_id_by_name(process_name: &str) -> u32 {
  unsafe {
    let handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if let Ok(snapshot) = handle {
      let mut entry = PROCESSENTRY32 {
        dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
        ..Default::default()
      };
      let cptr = wstring_c_str(str2wstring(process_name));
      if let Ok(_) = Process32First(snapshot, &mut entry) {
        loop {
          let mut origin_ptr = [0u16; MAX_PATH as usize];
          let lpwidecharstr: Option<&mut [u16]> = Some(&mut origin_ptr);
          print!("entry.szExeFile: {:?}", &entry.szExeFile.len());
          let w_process_name = MultiByteToWideChar(CP_ACP, MULTI_BYTE_TO_WIDE_CHAR_FLAGS(0u32), &entry.szExeFile, lpwidecharstr);
          println!("w_process_name: {:?}", w_process_name);
          // w_process_name.
          if uaw_wcsicmp( cptr, cptr) == 0 {
            return entry.th32ProcessID;
          }
          if Process32Next(snapshot, &mut entry).is_err() {
            break;
          }
        }
      };
      // CloseHandle(snapshot);
    }
  }
  0u32
}

pub fn world() -> String {
  "world".to_string()
}
