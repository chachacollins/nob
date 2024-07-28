use std::env;

use init::init_prog;
mod init;
mod watcher;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        help();
    }
    match args[1].as_str() {
        "init" => init_prog(),
        "run" => println!("goodbye"),
        "watch" => watcher::file_watcher(&args[2]),
        _ => {
            println!("unknown command");
            help();
        }
    }
}

fn help() {
    println!("Commands are init, watch and run")
}
