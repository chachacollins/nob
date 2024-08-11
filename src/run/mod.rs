use std::fs::read_dir;
use std::io;
use std::path::Path;
use std::process;

use std::env;

pub fn run_code() -> io::Result<()> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let dir = Path::new(".");
    let bash_path = Path::new("build.sh");

    let build_profile = read_dir(dir)?
        .filter_map(Result::ok)
        .any(|entry| entry.path() == bash_path);
    if build_profile {
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
    }

    println!("Compiled project at Current directory: {:?}", current_dir);
    Ok(())
}
