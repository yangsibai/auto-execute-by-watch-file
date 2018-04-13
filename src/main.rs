use std::env;
use std::time::SystemTime;
use std::fs;
use std::{thread, time};
use std::process::Command;

fn main() {
    let fileToWatch = env::args().nth(1).expect("Please specifc the file to watch.");
    let commandToExecute = env::args().nth(2).expect("Please specifc the command to execute.");
    println!("{} {}", fileToWatch, commandToExecute);

    let mut initialTime = getModifiedTime(&fileToWatch);

    let sleepTime = time::Duration::from_secs(5);

    while true {
        let currentTime = getModifiedTime(&fileToWatch);
        if currentTime != initialTime {
            println!("execute the command {}", commandToExecute);
            execute(&commandToExecute);
            initialTime = currentTime;
        }
        thread::sleep(sleepTime);
    }
}


fn getModifiedTime(path: &str) -> SystemTime {
    fs::metadata(path).unwrap().modified().unwrap()
}


fn execute(cmd: &str) {
    let output = Command::new(cmd).output().expect("Fail to execute process");
    println!("{:?}", output);
}
