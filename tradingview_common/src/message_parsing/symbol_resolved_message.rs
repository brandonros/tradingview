use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::SimpleResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolResolvedMessage {

}

impl SymbolResolvedMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("symbol_resolved = {parsed_message:?}");
        Ok(SymbolResolvedMessage {
            
        })
    }
}
