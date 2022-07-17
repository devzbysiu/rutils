use crate::error::Result;

pub trait Notifier {
    fn notify(&self, msg: &str) -> Result<()>;
}

pub fn notify(msg: &str, notifier: Box<dyn Notifier>) -> Result<()> {
    notifier.notify(msg)
}
