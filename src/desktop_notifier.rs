use crate::data_providers::desktop_notifier::DesktopNotifier;
use crate::error::Result;
use crate::use_cases::notifier;

pub fn notify<S: Into<String>>(msg: S) -> Result<()> {
    notifier::notify(&msg.into(), Box::new(DesktopNotifier))?;
    Ok(())
}
