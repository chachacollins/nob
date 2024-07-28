use std::{
    fs::{self, metadata},
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
        printf("hello world");
    }
    "#;
    let bash_boiler_plate = r#"
    gcc -o main main.c 
    chmod +x main
    ./main

    "#;

    let _create_main_file = fs::write(main_path, boiler_plate).map_err(|x| {
        eprintln!("{}", x);
        process::exit(1);
    });
    let _create_bash_file = fs::write(&bash_path, bash_boiler_plate).map_err(|x| {
        eprintln!("{}", x);
        process::exit(1);
    });
    let metadata = metadata(&bash_path).unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o040755);
    let _ = fs::set_permissions(&bash_path, permissions).map_err(|x| {
        eprintln!("{}", x);
        process::exit(1);
    });
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
