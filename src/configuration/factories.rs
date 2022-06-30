use crate::data_providers::notifier::IftttNotifier;
use crate::use_cases::notifier::Notifier;

pub fn get_notifiers() -> Vec<Box<dyn Notifier>> {
    vec![Box::new(IftttNotifier)]
}
