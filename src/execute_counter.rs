// use lazy_static;
use libc::{atexit, c_int, c_void, remove, rename, sighandler_t, signal, SIGTERM};
use std::{env, str};
use std::fs::File;
use std::io::prelude::*;
use std::path:: PathBuf;
#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

// #[link_section = ".countdown_section"]
// #[no_mangle]
// pub static VAR1: u32 = 100;
// pub static DISK_ID: &'static str = "123456789";

#[used]
#[link_section = ".custom_data"]
static REMEAIN_TIMES: &'static [u8] = b"Self downcounter remain times:9999";
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
       
        if cfg!(target_os = "linux"){
            file.set_permissions(std::fs::Permissions::from_mode(0o755));
        }

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
