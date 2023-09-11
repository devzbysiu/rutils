#[cfg(feature = "ntfy_notifier")]
pub mod ntfy_notifier;

#[cfg(feature = "ifttt_notifier")]
pub mod ifttt_notifier;

#[cfg(feature = "desktop_notifier")]
pub mod desktop_notifier;
