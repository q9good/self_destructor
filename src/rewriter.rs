// use lazy_static;
use libc::{atexit, c_int, c_void, remove, rename, sighandler_t, signal, SIGTERM};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// #[link_section = ".countdown_section"]
// #[no_mangle]
// pub static VAR1: u32 = 100;
// pub static DISK_ID: &'static str = "123456789";

const REMEAIN_TIMES: &'static [u8] = b"There are 136729 times leftover";

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
    let mut oldfile =env::current_dir()?;
    oldfile.push(&ENV_PARAM[0]);
    println!("{:?}", oldfile);
    // unsafe{libc::remove(ENV_PARAM[0].as_ptr()  as *const i8);}
    // unsafe{libc::remove(del.as_ptr()  as *const i8);}
    std::fs::remove_file(oldfile).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    Ok(())
}

pub fn write_new(){
   let mut newfile = std::env::current_dir().expect(" ");
    newfile.push(&ENV_PARAM[0]); 
    let mut file = std::fs::File::create(newfile).expect("create failed");
    let mut content = OLD_FILE_CONTENTS.clone();
    // content[0] = 0xff;
    file.write_all(&content);
}

pub fn search_pattern()->usize{
    let pattern = REMEAIN_TIMES.to_vec();
    let ret = find3(&OLD_FILE_CONTENTS, &pattern).unwrap();
    ret
}

fn find3(haystack: &Vec<u8>, needle: &Vec<u8>) -> Option<usize> {
    (0..haystack.len()-needle.len()+1)
        .filter(|&i| haystack[i..i+needle.len()] == needle[..]).next()
}