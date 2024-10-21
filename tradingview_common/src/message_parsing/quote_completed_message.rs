use std::error::Error;

use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::{box_err, SimpleResult};

use crate::{json_utilities, ParsedTradingViewMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteCompletedMessage {
    pub quote_session_id: String,
    pub symbol: String,
}

impl QuoteCompletedMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("quote_completed = {parsed_message:?}");
        let p = parsed_message.get("p").ok_or(box_err!("failed to get p"))?;
        let p = json_utilities::value_to_array(p)?;
        let quote_session_id = &p[0];
        let quote_session_id = json_utilities::value_to_string(&quote_session_id)?;
        let symbol = &p[1];
        let symbol = json_utilities::value_to_string(&symbol)?;
        Ok(QuoteCompletedMessage {
            quote_session_id,
            symbol
        })
    }
}

impl TryFrom<ParsedTradingViewMessage> for QuoteCompletedMessage {
    type Error = Box<dyn Error>;

    fn try_from(value: ParsedTradingViewMessage) -> Result<Self, Self::Error> {
        match value {
            ParsedTradingViewMessage::QuoteCompleted(msg) => Ok(msg),
            _ => Err(box_err!("failed to cast")),
        }
    }
}
