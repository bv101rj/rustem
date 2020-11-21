use std::process::Command;

fn main(){

let output = if cfg!(target_os="windows"){
    Command::new("cmd")
        .args(&["/C", "echo hello"])
        .output()
        .expect("failed to execute")
} else {
    Command::new("sh")
        .arg("-c")
        .arg("echo poop > dung.txt")
        .output()
        .expect("failed to execute")
};

let hello = output.stdout;

}
