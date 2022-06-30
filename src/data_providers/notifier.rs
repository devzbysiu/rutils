use crate::error::Result;
use crate::use_cases::notifier::Notifier;

use dotenv_codegen::dotenv;

pub struct IftttNotifier;

impl Notifier for IftttNotifier {
    fn notify(&self, msg: &str) -> Result<()> {
        ureq::post(&ifttt_url())
            .set("Content-Type", "application/json")
            .send_json(ureq::json!({
                "value1": msg,
            }))?;
        Ok(())
    }
}

fn ifttt_url() -> String {
    let key = dotenv!("IFTTT_KEY");
    format!("https://maker.ifttt.com/trigger/notify/with/key/{}", key)
}
