use crate::configuration::factories::get_notifiers;
use crate::error::Result;

mod configuration;
mod data_providers;
mod error;
mod use_cases;

pub fn notify<S: Into<String>>(msg: S) -> Result<()> {
    let notifiers = get_notifiers();
    let msg = msg.into();
    for notifier in notifiers {
        notifier.notify(&msg)?;
    }
    Ok(())
}
