use std::process;

use std::env;

pub fn run_code() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Compiled project at Current directory: {:?}", current_dir);

    let result = process::Command::new("zsh")
        .arg("-C")
        .arg("./build.sh")
        .stdout(process::Stdio::inherit())
        .stdin(process::Stdio::inherit())
        .output()
        .map_err(|x| {
            eprintln!("Error: {}", x);
            process::exit(1);
        });

    if let Err(e) = result {
        eprintln!("Failed to execute build.sh: {:?}", e);
        process::exit(1);
    }
}
