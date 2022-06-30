use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotifeyeError {
    #[error("Ureq error")]
    UreqError(#[from] ureq::Error),
}

pub type Result<T> = std::result::Result<T, NotifeyeError>;
