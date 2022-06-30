use crate::error::Result;

pub trait Notifier {
    fn notify(&self, msg: &str) -> Result<()>;
}
