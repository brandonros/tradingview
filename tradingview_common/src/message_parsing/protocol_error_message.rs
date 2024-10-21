use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::SimpleResult;

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
