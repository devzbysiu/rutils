use crate::error::Result;
use crate::use_cases::notifier::Notifier;

use notify_rust::{Notification, Timeout, Urgency};

#[allow(clippy::module_name_repetitions)]
pub struct DesktopNotifier;

impl Notifier for DesktopNotifier {
    fn notify(&self, msg: &str) -> Result<()> {
        Notification::new()
            .summary("Urgent")
            .body(msg)
            .timeout(Timeout::Never)
            .urgency(Urgency::Critical)
            .show()?;
        Ok(())
    }
}
