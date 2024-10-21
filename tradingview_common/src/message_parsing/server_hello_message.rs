use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::SimpleResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHelloMessage {

}

impl ServerHelloMessage {
    pub fn from_object(_parsed_message: &Object) -> SimpleResult<Self> {
        Ok(Self {

        })
    }
}
