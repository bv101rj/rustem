
use std::process::Command;

fn main{

Command::new("bash")
    .arg("-c")
    .arg("mkdir testing")
    .output()
    .expect("failed stoopid")

};
