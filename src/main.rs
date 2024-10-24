use std::env;

use init::init_prog;
mod init;
mod run;
mod watcher;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        help();
        return;
    }
    match args[1].as_str() {
        "init" => init_prog(),
        "run" => run::run_code().expect("Could not run code"),
        "watch" => {
            if args.len() < 3 {
                println!("Too few arguements passed to watch");
            }
            watcher::file_watcher(&args[2]).expect("File watcher failed");
        }
        _ => {
            println!("unknown command");
            help();
        }
    }
}

fn help() {
    println!("Commands are init, watch and run")
}
