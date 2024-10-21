use std::error::Error;

use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::{box_err, SimpleResult};

use crate::ParsedTradingViewMessage;

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

impl TryFrom<ParsedTradingViewMessage> for NotifyUserMessage {
    type Error = Box<dyn Error>;

    fn try_from(value: ParsedTradingViewMessage) -> Result<Self, Self::Error> {
        match value {
            ParsedTradingViewMessage::NotifyUser(msg) => Ok(msg),
            _ => Err(box_err!("failed to cast")),
        }
    }
}
