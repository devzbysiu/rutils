use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotifeyeError {
    #[error("Generic error")]
    Generic,
}

pub type Result<T> = std::result::Result<T, NotifeyeError>;
