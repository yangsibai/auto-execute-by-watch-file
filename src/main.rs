use std::env;
use std::time::SystemTime;
use std::fs;
use std::{thread, time};
use std::process::Command;

fn main() {
    let file_to_watch = env::args().nth(1).expect("Please specifc the file to watch.");
    let cmd_to_execute = env::args().nth(2).expect("Please specifc the command to execute.");
    println!("{} {}", file_to_watch, cmd_to_execute);

    let mut initial_time = get_modified_time(&file_to_watch);

    let sleep_time = time::Duration::from_secs(5);

    loop {
        let current_time = get_modified_time(&file_to_watch);
        if current_time != initial_time {
            println!("execute the command {}", cmd_to_execute);
            execute(&cmd_to_execute);
            initial_time = current_time;
        }
        thread::sleep(sleep_time);
    }
}


fn get_modified_time(path: &str) -> SystemTime {
    fs::metadata(path).unwrap().modified().unwrap()
}


fn execute(cmd: &str) {
    let v: Vec<&str> = cmd.split(' ').collect();
    let output = Command::new(v[0]).args(&v[1..]).output().expect("Fail to execute process");
    println!("{:?}", output);
}
