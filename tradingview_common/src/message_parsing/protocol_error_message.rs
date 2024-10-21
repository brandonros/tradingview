use std::error::Error;

use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::{box_err, SimpleResult};

use crate::ParsedTradingViewMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolErrorMessage {

}

impl ProtocolErrorMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("protocol_error = {parsed_message:?}");
        Ok(ProtocolErrorMessage {
            
        })
    }
}

impl TryFrom<ParsedTradingViewMessage> for ProtocolErrorMessage {
    type Error = Box<dyn Error>;

    fn try_from(value: ParsedTradingViewMessage) -> Result<Self, Self::Error> {
        match value {
            ParsedTradingViewMessage::ProtocolError(msg) => Ok(msg),
            _ => Err(box_err!("failed to cast")),
        }
    }
}
