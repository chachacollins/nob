use std::path::Path;

use notify::{RecursiveMode, Watcher};
use std::env;

use crate::run::run_code;

pub fn file_watcher(args: &String) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("{:?}", current_dir);
    loop {
        let mut watcher =
            notify::recommended_watcher(|res: Result<notify::Event, notify::Error>| match res {
                Ok(event) => match event.kind {
                    notify::EventKind::Create(_) => println!("file created"),

                    notify::EventKind::Modify(_) => run_code(),

                    notify::EventKind::Remove(_) => println!("file removed"),
                    _ => (),
                },
                Err(e) => println!("watch error: {:?}", e),
            })
            .unwrap();
        let _ = watcher.watch(Path::new(args), RecursiveMode::Recursive);
    }
}
