use std::error::Error;

use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::{box_err, SimpleResult};

use crate::ParsedTradingViewMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalErrorMessage {

}

impl CriticalErrorMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("critical_error = {parsed_message:?}");
        Ok(CriticalErrorMessage {
            
        })
    }
}

impl TryFrom<ParsedTradingViewMessage> for CriticalErrorMessage {
    type Error = Box<dyn Error>;

    fn try_from(value: ParsedTradingViewMessage) -> Result<Self, Self::Error> {
        match value {
            ParsedTradingViewMessage::CriticalError(msg) => Ok(msg),
            _ => Err(box_err!("failed to cast")),
        }
    }
}