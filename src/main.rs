use libc::{atexit, c_int, c_void,  rename, sighandler_t, signal, SIGTERM};
use std::env;
use std::ffi::CStr;
use std::fs::File;
use std::io::prelude::*;
use self_destructor::rewriter::*;
use std::path::{Path, PathBuf};

// static HOST_FILE: &'static [u8] = include_bytes!("self_destructor");
const TIMES:u32 = 0xcf2e6c9d;

extern "C" fn update_self() {
    println!("hi there");
}
extern "C" fn handler(_: c_int) {
    println!("hi you");
}

fn get_handler() -> sighandler_t {
    handler as extern "C" fn(c_int) as *mut c_void as sighandler_t
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let self_file = &args[0];
    // let mut fs = File::open(self_file).unwrap();
    // // let mut contents = Vec::with_capacity(0x100000*20);
    // let mut contents = Vec::with_capacity(0x100000 * 20);
    // let mut new_file = args[0].to_string();
    // new_file.push_str(".bk");
    // let mut new_fs = File::create(new_file).unwrap();

    // let exit = || new_fs.write_all(&contents).unwrap();

    // unsafe { atexit(update_self) };
    // unsafe { signal(SIGTERM, get_handler()) };

    // let lazy_len = rewriter::get_old_file_length();
    // println!("{}", lazy_len);

    // if let std::io::Result::Ok(len) = fs.read_to_end(&mut contents) {
    //     println!("{}", len);
    //     print!("\n");
    //     contents[len - 1] = 0xff;
    //     // new_fs.write_all(&contents).unwrap();
    // }
    // println!("{:}", rewriter::search_pattern());
    // rewriter::remove_old();
    // rewriter::write_new();
    let mut counter = ExecutionCounter::new(); 
    counter.init();
}
