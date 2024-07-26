use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args[1].as_str() {
        "hello" => println!("hello"),
        "goodbye" => println!("goodbye"),
        _ => println!("unknown command"),
    }
}
