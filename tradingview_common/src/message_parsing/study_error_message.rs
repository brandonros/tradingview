use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::SimpleResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyErrorMessage {

}

impl StudyErrorMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("study_error = {parsed_message:?}");
        Ok(StudyErrorMessage {
            
        })
    }
}
