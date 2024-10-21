use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::SimpleResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyUserMessage {

}

impl NotifyUserMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("notify_user = {parsed_message:?}");
        Ok(NotifyUserMessage {
            
        })
    }
}
