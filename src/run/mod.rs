use std::fs;
use std::io;
use std::process;

use std::env;

pub fn run_code() -> io::Result<()> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let _build_profile = fs::read_to_string("./build.sh").expect("failed to find build profile");
    println!("Compiled project at Current directory: {:?}", current_dir);

    let result = process::Command::new("zsh")
        .arg("-C")
        .arg("./build.sh")
        .stdout(process::Stdio::inherit())
        .stdin(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .output()
        .map_err(|x| {
            eprintln!("Error: {}", x);
            process::exit(1);
        });

    if let Err(e) = result {
        eprintln!("Failed to execute build.sh: {:?}", e);
        process::exit(1);
    }
    Ok(())
}
