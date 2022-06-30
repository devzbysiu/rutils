use crate::configuration::factories::get_notifiers;

mod configuration;
mod data_providers;
mod use_cases;

pub fn notify<S: Into<String>>(msg: S) -> Result<()> {
    let notifiers = get_notifiers();
}
