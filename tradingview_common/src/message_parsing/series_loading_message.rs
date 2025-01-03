use std::error::Error;

use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::{box_err, SimpleResult};

use crate::ParsedTradingViewMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesLoadingMessage {

}

impl SeriesLoadingMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("series_loading = {parsed_message:?}");
        Ok(SeriesLoadingMessage {
            
        })
    }
}

impl TryFrom<ParsedTradingViewMessage> for SeriesLoadingMessage {
    type Error = Box<dyn Error>;

    fn try_from(value: ParsedTradingViewMessage) -> Result<Self, Self::Error> {
        match value {
            ParsedTradingViewMessage::SeriesLoading(msg) => Ok(msg),
            _ => Err(box_err!("failed to cast")),
        }
    }
}
