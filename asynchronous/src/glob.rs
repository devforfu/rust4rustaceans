use futures::TryStreamExt;
use std::{fs, path::PathBuf};

pub struct Glob {
    stack: Vec<PathBuf>,
    recursive: bool,
    level: usize,
}

impl Glob {
    pub fn new(root: PathBuf, recursive: bool) -> Self {
        Self {
            stack: vec![root],
            recursive,
            level: 0,
        }
    }
}

impl Iterator for Glob {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }

        let path = self.stack.swap_remove(0);

        if path.is_dir() && (self.recursive || self.level <= 0) {
            if let Ok(read_dir) = fs::read_dir(path.clone()) {
                for filename in read_dir {
                    if let Ok(filename) = filename {
                        self.stack.push(filename.path());
                    }
                }
                self.level += 1;
            }
        }

        Some(path)
    }
}

pub struct AsyncGlob {
    stack: Vec<PathBuf>,
    recursive: bool,
    level: usize,
}

impl AsyncGlob {
    pub fn new(root: PathBuf, recursive: bool) -> Self {
        Self {
            stack: vec![root],
            recursive,
            level: 0,
        }
    }

    pub async fn next(&mut self) -> Option<PathBuf> {
        if self.stack.is_empty() {
            return None;
        }

        let path = self.stack.swap_remove(0);

        if path.is_dir() && (self.recursive || self.level <= 0) {
            if let Ok(mut entries) = async_fs::read_dir(path.clone()).await {
                while let Ok(Some(entry)) = entries.try_next().await {
                    self.stack.push(entry.path());
                }
                self.level += 1;
            }
        }

        Some(path)
    }
}
