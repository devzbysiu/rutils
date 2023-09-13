use crate::error::Result;

use flexi_logger::{detailed_format, Age, Cleanup, Criterion, FileSpec, Logger, Naming};
use std::env::{current_exe, var};

pub fn setup_logger() -> Result<()> {
    let exe_path = current_exe()?;
    let program_name = exe_path.file_name().expect("failed to get filename");
    let target_dir = dirs::config_dir()
        .expect("failed to read config")
        .join(program_name);
    let log_str = var("RUST_LOG").unwrap_or_else(|_| "debug".into());
    Logger::try_with_str(log_str)?
        .log_to_file(FileSpec::default().directory(target_dir))
        .format(detailed_format)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(3),
        )
        .start()?;
    Ok(())
}
