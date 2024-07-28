use std::path::Path;

use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};

pub fn file_watcher() {
    loop {
        let mut watcher = notify::recommended_watcher(|res| match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        })
        .unwrap();
        watcher.watch(Path::new("."), RecursiveMode::Recursive);
    }
}
