use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::SimpleResult;

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

