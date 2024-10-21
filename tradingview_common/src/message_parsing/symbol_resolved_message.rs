use std::error::Error;

use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::{box_err, SimpleResult};

use crate::ParsedTradingViewMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolResolvedMessage {

}

impl SymbolResolvedMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("symbol_resolved = {parsed_message:?}");
        Ok(SymbolResolvedMessage {
            
        })
    }
}

impl TryFrom<ParsedTradingViewMessage> for SymbolResolvedMessage {
    type Error = Box<dyn Error>;

    fn try_from(value: ParsedTradingViewMessage) -> Result<Self, Self::Error> {
        match value {
            ParsedTradingViewMessage::SymbolResolved(msg) => Ok(msg),
            _ => Err(box_err!("failed to cast")),
        }
    }
}
