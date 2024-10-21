use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::SimpleResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesCompletedMessage {

}

impl SeriesCompletedMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("series_completed = {parsed_message:?}");
        Ok(SeriesCompletedMessage {
            
        })
    }
}
