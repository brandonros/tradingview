use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::SimpleResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyCompletedMessage {

}

impl StudyCompletedMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("study_completed = {parsed_message:?}");
        Ok(StudyCompletedMessage {
            
        })
    }
}
