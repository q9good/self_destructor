// use lazy_static;

use std::fs::File;

use std::io::prelude::*;

#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

use std::path::PathBuf;

use std::{env, process::Command, str};

// #[link_section = ".countdown_section"]

// #[no_mangle]

// pub static VAR1: u32 = 100;

// pub static DISK_ID: &'static str = "123456789";

#[used]
#[link_section = ".custom_data"]

static REMEAIN_TIMES: &'static [u8] = b"Self downcounter remain times:0003";

pub struct ExecutionCounter {
    execute_name: String,

    execute_dir: PathBuf,

    execute_content: Vec<u8>,

    counter_str: Vec<u8>,

    counter_offset: Option<usize>,

    counter_times: Option<i32>,
}

impl ExecutionCounter {
    pub fn new() -> Self {
        ExecutionCounter {
            execute_name: String::new(),

            execute_dir: PathBuf::new(),

            execute_content: Vec::new(),

            counter_str: Vec::new(),

            counter_offset: None,

            counter_times: None,
        }
    }

    pub fn init(&mut self) {
        // get the path and name of executable file

        self.execute_dir = env::current_dir().unwrap();

        self.execute_name = env::args().nth(0).unwrap();

        // read executable file content

        let mut fs = File::open(&self.execute_name).unwrap();

        fs.read_to_end(&mut self.execute_content).unwrap();

        // search the string containing counter info in file

        self.counter_str = (&REMEAIN_TIMES[0..REMEAIN_TIMES.len() - 4]).to_vec();

        let pos = self.search_pattern();

        if let Some(offset) = pos {
            self.counter_offset = Some(offset + REMEAIN_TIMES.len() - 4);

            self.counter_times = self.parse_remain_times();
        } else {
            self.counter_offset = None;

            self.counter_times = None;
        }
    }

    fn search_pattern(&self) -> Option<usize> {
        (0..self.execute_content.len() - self.counter_str.len() + 1)
            .filter(|&i| {
                self.execute_content[i..i + self.counter_str.len()] == self.counter_str[..]
            })
            .next()
    }

    pub fn parse_remain_times(&self) -> Option<i32> {
        if self.counter_offset == None {
            return None;
        }

        let pos = self.counter_offset.unwrap();

        let counter_bytes = &self.execute_content[pos..pos + 4];

        let num = str::from_utf8(counter_bytes)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        Some(num)
    }

    fn update_remain_times(&mut self, new_val: i32) {
        if self.counter_offset == None {
            return;
        }

        let pos = self.counter_offset.unwrap();

        let counter_bytes = &mut self.execute_content[pos..pos + 4];

        let new_string = format!("{:04}", new_val);

        let counter_bytes1 = &new_string.as_bytes().to_vec();

        counter_bytes[0] = counter_bytes1[0];

        counter_bytes[1] = counter_bytes1[1];

        counter_bytes[2] = counter_bytes1[2];

        counter_bytes[3] = counter_bytes1[3];

        // let num = str::from_utf8(counter_bytes).unwrap().parse::<i32>().unwrap();
    }

    pub fn subtract(&mut self, step: i32) {
        if let Some(counter) = self.counter_times {
            self.update_remain_times(counter - step);

            self.counter_times = Some(counter - step);
        }
    }

    pub fn get_remain_times(&self) -> Option<i32> {
        self.counter_times
    }

    pub fn add(&mut self, step: i32) {
        if let Some(counter) = self.counter_times {
            self.update_remain_times(counter + step);

            self.counter_times = Some(counter + step);
        }
    }

    fn remove_old_exectue_file(&self) {
        let mut oldfile = self.execute_dir.clone();

        oldfile.push(&self.execute_name);

        if cfg!(target_os = "linux") {
            std::fs::remove_file(oldfile).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        } else {
            // use std::os::windows::process::CommandExt;

            let mut back_file = self.execute_dir.clone();

            let mut suffix = self.execute_name.clone();

            suffix.push_str(".bk");

            back_file.push(&suffix);

            let idx = self.execute_name.rfind('\\').unwrap();

            let _cmd = format!(
                "ping localhost -n 1 > nul & del {0} & rename {1} {2}",
                oldfile.to_str().unwrap(),
                back_file.to_str().unwrap(),
                &self.execute_name[idx + 1..]
            );

            let wind_cmd = r#"&_cmd"#;

            Command::new("cmd")
                .args(&["/C", wind_cmd])
                // .creation_flags(0x08000000)
                .spawn()
                .expect("failed to delete old executable");
        }
    }

    fn save_new_execute_file(&self) {
        let mut newfile = self.execute_dir.clone();

        if cfg!(target_os = "linux") {
            if let Some(pos) = self.execute_name.find('/') {
                newfile.push(&self.execute_name[pos + 1..]);
            } else {
                newfile.push(&self.execute_name);
            }
        } else {
            let mut back_file = self.execute_name.clone();

            back_file.push_str(".bk");

            newfile.push(&back_file);
        }

        let mut file = std::fs::File::create(newfile).expect("create failed");

        if cfg!(target_os = "linux") {

            // file.set_permissions(std::fs::Permissions::from_mode(0o755));
        }

        file.write_all(&self.execute_content);
    }

    pub fn update_execute(&self) {
        match self.counter_times {
            Some(0) => self.remove_old_exectue_file(),

            Some(_) => {
                if cfg!(target_os = "linux") {
                    self.remove_old_exectue_file();

                    self.save_new_execute_file();
                } else {
                    self.save_new_execute_file();

                    self.remove_old_exectue_file();
                }
            }

            None => (),
        }
    }
}

impl Drop for ExecutionCounter {
    fn drop(&mut self) {
        self.update_execute();
    }
}

impl Default for ExecutionCounter {
    fn default() -> Self {
        ExecutionCounter::new()
    }
}
