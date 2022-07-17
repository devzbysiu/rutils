use thiserror::Error;

#[allow(clippy::module_name_repetitions, clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum RutilsError {
    #[error("Ureq error")]
    #[cfg(feature = "ifttt_notifier")]
    UreqError(#[from] ureq::Error),

    #[cfg(feature = "desktop_notifier")]
    #[error("Desktop notification error")]
    NotifierError(#[from] notify_rust::error::Error),

    #[error("IO Error")]
    IoError(#[from] std::io::Error),

    #[cfg(feature = "file_logger")]
    #[error("FlexiLogger error")]
    FileLoggerError(#[from] flexi_logger::FlexiLoggerError),
}

pub type Result<T> = std::result::Result<T, RutilsError>;
