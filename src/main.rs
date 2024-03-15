use windows::{HkeyMap, RegType};

mod windows;

fn main() {
  let r = windows::read_registry(HkeyMap::HKCU, "SOFTWARE\\Tencent\\valorant.live", "DisplayName", RegType::RegSz);

  if let Ok(Some(v)) = r {
    match v {

    }
  } else {
    println!("error: {:?}", r);
  }
  // match q {
  //     Ok(v) => println!("gene: {:?}", v),
  //     Err(e) => println!("gene error: {:?}", e),
  // };
}

