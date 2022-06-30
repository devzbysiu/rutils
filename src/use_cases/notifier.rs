pub trait Notifier {
    fn notify(&self, msg: &str);
}
