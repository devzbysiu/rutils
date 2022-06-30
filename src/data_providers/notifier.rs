use crate::use_cases::notifier::Notifier;

pub struct IftttNotifier;

impl Notifier for IftttNotifier {
    fn notify(&self, _msg: &str) {
        unimplemented!()
    }
}
