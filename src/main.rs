use watchr_filesystem::{Event, FileWatcher};

use std::path::PathBuf;

fn main() {
    // Watch the destination folder in the current directory for changes
    let watcher = FileWatcher::new(vec![PathBuf::from("./destination")]);

    // Run the watcher
    let _ = watcher.watch(log_event);
}

fn log_event(event: Event) {
    println!("Event type: {:?}, Path: {:?}", event.kind, event.paths[0]);
}
