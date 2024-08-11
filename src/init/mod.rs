use std::{
    fs::{self, metadata, read_dir},
    os::unix::fs::PermissionsExt,
    process,
};

pub fn init_prog() {
    create_proj();
    println!("Finished scafolding C project");
}

fn create_proj() {
    let main_path = "./main.c".to_string();
    let bash_path = "./build.sh".to_string();
    let boiler_plate = r#"
#include <stdio.h>

int main(int argc, char** argv){
    printf("hello world\n");
    return 0;
}
    "#;
    let bash_boiler_plate = r#"
gcc -Wall -o main main.c 
chmod +x main
./main

    "#;

    let _create_main_file = fs::write(&main_path, boiler_plate).map_err(|x| {
        eprintln!("{}", x);
        process::exit(1);
    });
    let _create_bash_file = fs::write(&bash_path, bash_boiler_plate).map_err(|x| {
        eprintln!("{}", x);
        process::exit(1);
    });

    permission_exec(bash_path);
    permission_exec(main_path);
    let _git_init = git_init();
}

fn git_init() {
    let _git = process::Command::new("git")
        .arg("init")
        .output()
        .map_err(|x| {
            eprint!("{}", x);
            process::exit(1);
        });
    println!("Initialised git at root of the project");
}

fn permission_exec(path: String) {
    let metadata = metadata(&path).unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o040755);
    let _ = fs::set_permissions(&path, permissions).map_err(|x| {
        eprintln!("{}", x);
        process::exit(1);
    });
}
fn check_if_init() {
    let current_dir_path = ".".to_string();
    let dir_entries = read_dir(current_dir_path).map_err(|x| {
        eprintln!("{}", x);
        process::exit(1);
    });
    if let Ok(entries) = dir_entries {
        for entry in entries {
            let entry = entry.map_err(|x| {
                eprintln!("{}", x);
                process::exit(1);
            });
            // println!("{:?}", entry.path());
        }
    }
}
