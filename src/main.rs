use self_destructor::execute_counter::*;




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
    println!("{:?}", counter.get_remain_times());
    counter.subtract(1);
    let new = counter.parse_remain_times();
    println!("{:?}", new);
}
