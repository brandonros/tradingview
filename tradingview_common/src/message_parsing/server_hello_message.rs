use std::error::Error;

use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::{box_err, SimpleResult};

use crate::ParsedTradingViewMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHelloMessage {

}

impl ServerHelloMessage {
    pub fn from_object(_parsed_message: &Object) -> SimpleResult<Self> {
        Ok(Self {

        })
    }
}

impl TryFrom<ParsedTradingViewMessage> for ServerHelloMessage {
    type Error = Box<dyn Error>;

    fn try_from(value: ParsedTradingViewMessage) -> Result<Self, Self::Error> {
        match value {
            ParsedTradingViewMessage::ServerHello(msg) => Ok(msg),
            _ => Err(box_err!("failed to cast")),
        }
    }
}
