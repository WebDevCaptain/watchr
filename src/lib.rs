//! Watchr
//!
//! A simple file watcher that watches a list of paths and calls a callback when any of them change

use anyhow::Result;
use notify::{self, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use tokio::sync::mpsc;

pub use notify::Event;

/// A simple file watcher for monitoring file-system events (changes)
pub struct FileWatcher {
    paths: Vec<PathBuf>,
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self { paths }
    }

    /// Starts watching the provided paths and triggers the callback when any of them change
    #[tokio::main(flavor = "current_thread")]
    pub async fn watch<F>(self, mut callback: F) -> Result<(), notify::Error>
    where
        F: FnMut(Event) + Send + 'static,
    {
        let (tx, mut rx) = mpsc::channel(100);

        thread::spawn(move || {
            let (raw_tx, raw_rx): (Sender<Result<Event, notify::Error>>, Receiver<_>) = channel();

            let mut watcher: RecommendedWatcher =
                Watcher::new(raw_tx, notify::Config::default()).unwrap();

            for path in self.paths {
                watcher.watch(&path, RecursiveMode::Recursive).unwrap();
            }

            while let Ok(event) = raw_rx.recv() {
                if let Ok(event) = event {
                    let _ = tx.blocking_send(event);
                }
            }
        });

        while let Some(event) = rx.recv().await {
            callback(event);
        }

        Ok(())
    }
}
