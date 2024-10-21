use std::error::Error;

use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::{box_err, SimpleResult};

use crate::ParsedTradingViewMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickmarkUpdateMessage {

}

impl TickmarkUpdateMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("tickmark_update = {parsed_message:?}");
        Ok(TickmarkUpdateMessage {
            
        })
    }
}

impl TryFrom<ParsedTradingViewMessage> for TickmarkUpdateMessage {
    type Error = Box<dyn Error>;

    fn try_from(value: ParsedTradingViewMessage) -> Result<Self, Self::Error> {
        match value {
            ParsedTradingViewMessage::TickmarkUpdate(msg) => Ok(msg),
            _ => Err(box_err!("failed to cast")),
        }
    }
}
