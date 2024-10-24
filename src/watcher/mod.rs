use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use std::env;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn file_watcher(args: &String) -> Result<()> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!(
        " watching file: {:?}, in current directory: {:?}",
        args, current_dir
    );

    let (tx, rx) = channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(move |res| {
        tx.send(res).unwrap();
    })?;

    watcher.watch(Path::new(args), RecursiveMode::Recursive)?;
    loop {
        match rx.recv_timeout(Duration::from_secs(2)) {
            Ok(event) => match event {
                Ok(event) => {
                    if let EventKind::Modify(_) = event.kind {
                        crate::run::run_code().expect("unable to run code");
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            },
            _ => {}
        }
    }
}
