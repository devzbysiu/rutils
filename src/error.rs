use thiserror::Error;

#[allow(clippy::module_name_repetitions, clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum RutilsError {
    #[error("Ureq error")]
    UreqError(#[from] ureq::Error),

    #[error("IO Error")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, RutilsError>;
