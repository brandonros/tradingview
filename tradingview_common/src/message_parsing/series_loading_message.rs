use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::SimpleResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesLoadingMessage {

}

impl SeriesLoadingMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("series_loading = {parsed_message:?}");
        Ok(SeriesLoadingMessage {
            
        })
    }
}
