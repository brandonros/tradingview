use std::error::Error;

use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::{box_err, SimpleResult};

use crate::ParsedTradingViewMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesCompletedMessage {

}

impl SeriesCompletedMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("series_completed = {parsed_message:?}");
        Ok(SeriesCompletedMessage {
            
        })
    }
}

impl TryFrom<ParsedTradingViewMessage> for SeriesCompletedMessage {
    type Error = Box<dyn Error>;

    fn try_from(value: ParsedTradingViewMessage) -> Result<Self, Self::Error> {
        match value {
            ParsedTradingViewMessage::SeriesCompleted(msg) => Ok(msg),
            _ => Err(box_err!("failed to cast")),
        }
    }
}
