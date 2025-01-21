//! Watchr
//!
//! A simple file watcher that watches a list of paths and calls a callback when any of them change

use std::path::PathBuf;

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
    pub async fn watch<F>(self, callback: F)
    where
        F: FnMut(),
    {
        todo!()
    }
}
