use miniserde::{Deserialize, Serialize};

use crate::message_parsing::{
    ServerHelloMessage,
    QuoteSeriesDataMessage,
    DataUpdateMessage,
    QuoteCompletedMessage,
    TimescaleUpdatedMessage,
    SeriesLoadingMessage,
    SymbolResolvedMessage,
    SeriesCompletedMessage,
    StudyLoadingMessage,
    StudyCompletedMessage,
};

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
