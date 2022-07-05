use crate::error::Result;

use daemonize::Daemonize;
use std::fs::File;
use std::path::{Path, PathBuf};

// TODO: get rif of `sweetch-bot` logs
pub fn daemonize<F: FnOnce()>(fun: F) -> Result<()> {
    let daemonize = Daemonize::new()
        .stdout(File::create(log_path("sweetch-bot.out"))?)
        .stderr(File::create(log_path("sweetch-bot.log"))?);

    match daemonize.start() {
        Ok(_) => fun(),
        Err(e) => eprintln!("Error, {}", e),
    }
    Ok(())
}

fn log_path(filename: &str) -> PathBuf {
    Path::new(&std::env::temp_dir()).join(filename)
}
