use crate::error::Result;
use crate::use_cases::notifier::Notifier;

#[allow(clippy::module_name_repetitions)]
pub struct NtfyNotifier;

impl Notifier for NtfyNotifier {
    fn notify(&self, msg: &str) -> Result<()> {
        ureq::post(&ntfy_url()).send_string(msg)?;
        Ok(())
    }
}

fn ntfy_url() -> String {
    "https://ntfy.sh/1450_devzbysiu_notifs".to_string()
}
