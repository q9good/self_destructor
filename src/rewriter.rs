// use lazy_static;
use libc::{atexit, c_int, c_void, remove, rename, sighandler_t, signal, SIGTERM};
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[link_section = ".countdown_section"]
#[no_mangle]
pub static VAR1: u32 = 100;
pub static DISK_ID: &'static str = "123456789";

lazy_static! {
    static ref ENV_PARAM: Vec<String> = env::args().collect();
    static ref OLD_FILE_CONTENTS: Vec<u8> = {
        let mut contents = Vec::with_capacity(0x100000 * 20);
        let mut fs = File::open(&ENV_PARAM[0]).unwrap();
        fs.read_to_end(&mut contents).unwrap();
        contents
    };
}

pub fn display_old_file()
{
    println!("{:?}", *OLD_FILE_CONTENTS);
}

pub fn get_old_file_length()->u32{
    OLD_FILE_CONTENTS.len() as u32
}

pub fn remove_old()->std::io::Result<()>{
    println!("remove");
    let del = "~/workspace/rust/self_destructor/target/debug/1.txt";
    // unsafe{libc::remove(ENV_PARAM[0].as_ptr()  as *const i8);}
    // unsafe{libc::remove(del.as_ptr()  as *const i8);}
    std::fs::remove_file(del)?;
    Ok(())
}