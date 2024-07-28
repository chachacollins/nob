use std::path::Path;

use notify::{RecursiveMode, Watcher};

pub fn file_watcher(args: &String) {
    println!("watching current directory for changes");
    println!("watching current directory for changes");
    loop {
        let mut watcher =
            notify::recommended_watcher(|res: Result<notify::Event, notify::Error>| match res {
                Ok(event) => match event.kind {
                    notify::EventKind::Create(_) => println!("File created"),
                    notify::EventKind::Modify(_) => println!("File modified"),
                    notify::EventKind::Remove(_) => println!("File removed"),
                    _ => (),
                },
                Err(e) => println!("watch error: {:?}", e),
            })
            .unwrap();
        let _ = watcher.watch(Path::new(args), RecursiveMode::Recursive);
    }
}
