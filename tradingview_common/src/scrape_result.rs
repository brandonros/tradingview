use miniserde::{Deserialize, Serialize};

use crate::message_parsing::server_hello_message::ServerHelloMessage;
use crate::message_parsing::quote_series_data_message::QuoteSeriesDataMessage;
use crate::message_parsing::data_update_message::DataUpdateMessage;
use crate::message_parsing::quote_completed_message::QuoteCompletedMessage;
use crate::message_parsing::timescale_updated_message::TimescaleUpdatedMessage;
use crate::message_parsing::series_loading_message::SeriesLoadingMessage;
use crate::message_parsing::symbol_resolved_message::SymbolResolvedMessage;
use crate::message_parsing::series_completed_message::SeriesCompletedMessage;
use crate::message_parsing::study_loading_message::StudyLoadingMessage;
use crate::message_parsing::study_completed_message::StudyCompletedMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingViewScrapeResult {
    pub server_hello_messages: Vec<ServerHelloMessage>,
    pub symbol_resolved_messages: Vec<SymbolResolvedMessage>,
    pub series_loading_messages: Vec<SeriesLoadingMessage>,
    pub timescale_update_messages: Vec<TimescaleUpdatedMessage>,
    pub series_completed_messages: Vec<SeriesCompletedMessage>,
    pub study_loading_messages: Vec<StudyLoadingMessage>,
    pub study_completed_messages: Vec<StudyCompletedMessage>,
    pub quote_completed_messages: Vec<QuoteCompletedMessage>,
    pub quote_last_price_messages: Vec<QuoteSeriesDataMessage>,
    pub series_data_update_messages: Vec<DataUpdateMessage>, // TODO: split series and study?    
    pub study_data_update_messages: Vec<DataUpdateMessage>, // TODO: split series and study?
}
