mod windows;

fn main() {
    println!("Hello, {}!", windows::world());
    let bol = windows::get_process_id_by_name("notepad.exe");
    println!("bol: {}", bol);
}
