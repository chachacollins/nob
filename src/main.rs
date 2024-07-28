use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        help();
    }
    match args[1].as_str() {
        "init" => println!("hello"),
        "run" => println!("goodbye"),
        "watch" => println!("goodbye"),
        _ => {
            println!("unknown command");
            help();
        }
    }
}

fn help() {
    println!("Commands are init, watch and run")
}
