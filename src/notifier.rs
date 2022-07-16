use crate::configuration::factories::notifier::get_notifiers;
use crate::error::Result;

pub fn notify<S: Into<String>>(msg: S) -> Result<()> {
    let notifiers = get_notifiers();
    let msg = msg.into();
    for notifier in notifiers {
        notifier.notify(&msg)?;
    }
    Ok(())
}
