#![allow(clippy::missing_errors_doc)]

mod data_providers;
pub mod error;
mod use_cases;

#[cfg(feature = "ifttt_notifier")]
pub mod ifttt_notifier;

#[cfg(feature = "desktop_notifier")]
pub mod desktop_notifier;

#[cfg(feature = "daemon")]
pub mod daemon;

#[cfg(feature = "scheduler")]
pub mod scheduler;

#[cfg(feature = "file_logger")]
pub mod file_logger;
