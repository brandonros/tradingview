use std::error::Error;

use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::{box_err, SimpleResult};

use crate::ParsedTradingViewMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyLoadingMessage {

}

impl StudyLoadingMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("study_loading = {parsed_message:?}");
        Ok(StudyLoadingMessage {
            
        })
    }
}

impl TryFrom<ParsedTradingViewMessage> for StudyLoadingMessage {
    type Error = Box<dyn Error>;

    fn try_from(value: ParsedTradingViewMessage) -> Result<Self, Self::Error> {
        match value {
            ParsedTradingViewMessage::StudyLoading(msg) => Ok(msg),
            _ => Err(box_err!("failed to cast")),
        }
    }
}
