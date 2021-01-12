use std::env;
use std::fs::File;
use std::io::prelude::*;
use libc::{atexit, rename, remove, signal,sighandler_t,c_int,c_void, SIGTERM};


#[link_section = ".countdown_section"]
#[no_mangle]
pub static VAR1: u32 = 100;
pub static DISK_ID:&'static str = "123456789";

extern "C" fn update_self() {
    println!("hi there");
}
extern fn handler(_: c_int) {
    println!("hi you");
}

fn get_handler() -> sighandler_t {
    handler as extern fn(c_int) as *mut c_void as sighandler_t
}


fn main() {
    let args:Vec<String> = env::args().collect();
    let self_file = &args[0];

    unsafe { atexit(update_self) };
    unsafe {signal(SIGTERM, get_handler())};

    let mut fs = File::open(self_file).unwrap();
    // let mut contents = Vec::with_capacity(0x100000*20);
    let mut contents = Vec::with_capacity(0x100000*20);
    let mut new_file = args[0].to_string();
    new_file.push_str(".bk");
    let mut new_fs = File::create(new_file).unwrap();
    if let std::io::Result::Ok(len) = fs.read_to_end(&mut contents){
        println!("{}", len);
        print!("\n");
        contents[len - 1] = 0xff;
    new_fs.write_all(&contents).unwrap();
    }

}
