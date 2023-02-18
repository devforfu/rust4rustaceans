use async_lib::glob::AsyncGlob;
use futures::executor::block_on;
use std::path::PathBuf;

fn main() {
    if let Ok(home_dir) = std::env::var("HOME") {
        block_on(async {
            let mut glob = AsyncGlob::new(PathBuf::from(home_dir), false);
            while let Some(path) = glob.next().await {
                println!("{:?}", path);
            }
        });
    }
}
