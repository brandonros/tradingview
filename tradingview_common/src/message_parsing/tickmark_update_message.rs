use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::SimpleResult;

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
