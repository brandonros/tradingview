use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::SimpleResult;

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
