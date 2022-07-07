#![allow(clippy::missing_errors_doc)]

mod configuration;
mod data_providers;
pub mod error;
mod use_cases;

#[cfg(feature = "notifier")]
pub mod notifier;

#[cfg(feature = "daemon")]
pub mod daemon;

#[cfg(feature = "scheduler")]
pub mod scheduler;

#[cfg(feature = "file_logger")]
pub mod file_logger;
