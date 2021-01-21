// use lazy_static;
use libc::{atexit, c_int, c_void, remove, rename, sighandler_t, signal, SIGTERM};
use std::{env, str};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;

// #[link_section = ".countdown_section"]
// #[no_mangle]
// pub static VAR1: u32 = 100;
// pub static DISK_ID: &'static str = "123456789";

const REMEAIN_TIMES: &'static [u8] = b"Self downcounter remain times:9999";
pub struct ExecutionCounter{
    pub execute_name:String,
    pub execute_dir:PathBuf,
    pub execute_content: Vec<u8>,
    pub counter_str: Vec<u8>,
    pub counter_offset:Option<usize>,
    pub counter_times: Option<i32>,
}

impl ExecutionCounter {
    pub fn new()->Self{
        ExecutionCounter{
        execute_name:String::new(),
        execute_dir:PathBuf::new(),
        execute_content:Vec::new(),
        counter_str:Vec::new(),
        counter_offset:None,
        counter_times:None,
        }
    }

    pub fn init(&mut self){
        // get the path and name of executable file
        self.execute_dir = env::current_dir().unwrap();
        self.execute_name = env::args().nth(0).unwrap();

        // read executable file content 
        let mut fs = File::open(&self.execute_name).unwrap();
        fs.read_to_end(&mut self.execute_content).unwrap();

        // search the string containing counter info in file
        self.counter_str = (&REMEAIN_TIMES[0..REMEAIN_TIMES.len()-4]).to_vec();
        let  pos= self.search_pattern();
        if let Some(offset) = pos{
            self.counter_offset = Some(offset + REMEAIN_TIMES.len() - 4);
            self.counter_times = self.parse_remain_times();
        }else{
            self.counter_offset = None;
            self.counter_times = None;
        }
    }

    fn search_pattern(&self)-> Option<usize>{
        (0..self.execute_content.len()-self.counter_str.len()+1)
        .filter(|&i| self.execute_content[i..i+self.counter_str.len()] == self.counter_str[..]).next() 
    }

    pub fn parse_remain_times(&self)->Option<i32>{
        if self.counter_offset == None{
            return None;
        }
        let pos = self.counter_offset.unwrap() ;
        let counter_bytes = &self.execute_content[pos..pos+4];
        let num = str::from_utf8(counter_bytes).unwrap().parse::<i32>().unwrap();
        Some(num)
    }

    pub fn update_remain_times(&mut self, new_val: i32){
        if self.counter_offset == None{
            return;
        }
        let pos = self.counter_offset.unwrap() ;
        let mut counter_bytes = &mut self.execute_content[pos..pos+4];
        let new_string = new_val.to_string();
        let counter_bytes1 = &new_string.as_bytes().to_vec();
        // println!("{:?}",counter_bytes1);
        counter_bytes[0] = counter_bytes1[0];
        counter_bytes[1] = counter_bytes1[1];
        counter_bytes[2] = counter_bytes1[2];
        counter_bytes[3] = counter_bytes1[3];
        // let num = str::from_utf8(counter_bytes).unwrap().parse::<i32>().unwrap();
    }

    pub fn subtract(&mut self, step: i32){
        if let Some(counter) = self.counter_times{
            self.counter_times = Some(counter - step);
            self.update_remain_times(counter - step);
        }
    }

    pub fn remove_old_exectue_file(&mut self){
        let mut oldfile = self.execute_dir.clone();
        oldfile.push(&self.execute_name[..]);
        std::fs::remove_file(oldfile).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        }); 
    }

    pub fn save_new_execute_file(&self){
        let mut newfile = self.execute_dir.clone();
        newfile.push(&self.execute_name[..]);
        let mut file = std::fs::File::create(newfile).expect("create failed");
       
        // set execute permissions
        // let metadata = file.metadata().unwrap();
        // let mut permissions = metadata.permissions();
        // permissions.set_mode(0x777);
        file.set_permissions(std::fs::Permissions::from_mode(0o655));

        file.write_all(&self.execute_content);
    }
}

impl Drop for ExecutionCounter{
    fn drop(&mut self) {
        println!("> Dropping!");
        self.remove_old_exectue_file();
        self.save_new_execute_file();
    }
}

lazy_static! {
    static ref ENV_PARAM: Vec<String> = env::args().collect();
    static ref OLD_FILE_execute_contentS: Vec<u8> = {
        let mut execute_contents = Vec::with_capacity(0x100000 * 20);
        let mut fs = File::open(&ENV_PARAM[0]).unwrap();
        fs.read_to_end(&mut execute_contents).unwrap();
        execute_contents
    };
}

pub fn display_old_file()
{
    println!("{:?}", *OLD_FILE_execute_contentS);
}

pub fn get_old_file_length()->u32{
    OLD_FILE_execute_contentS.len() as u32
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
    let mut execute_content = OLD_FILE_execute_contentS.clone();
    // execute_content[0] = 0xff;
    file.write_all(&execute_content);
}

pub fn search_pattern()->usize{
    let pattern = REMEAIN_TIMES.to_vec();
    let ret = find3(&OLD_FILE_execute_contentS, &pattern).unwrap();
    ret
}

fn find3(haystack: &Vec<u8>, needle: &Vec<u8>) -> Option<usize> {
    (0..haystack.len()-needle.len()+1)
        .filter(|&i| haystack[i..i+needle.len()] == needle[..]).next()
}