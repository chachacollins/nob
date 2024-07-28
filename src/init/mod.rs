use std::{fs, process};

pub fn init_prog() {
    create_proj()
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
    let _create_main_file = fs::write(main_path, boiler_plate).map_err(|x| {
        eprintln!("{}", x);
        process::exit(1);
    });
    let _create_bash_file = fs::write(bash_path, boiler_plate).map_err(|x| {
        eprintln!("{}", x);
        process::exit(1);
    });
}
