use enum_as_inner::EnumAsInner;
use miniserde::json::Object;
use simple_error::{box_err, SimpleResult};

use crate::json_utilities;
use crate::message_parsing::server_hello_message::ServerHelloMessage;
use crate::message_parsing::quote_series_data_message::QuoteSeriesDataMessage;
use crate::message_parsing::data_update_message::DataUpdateMessage;
use crate::message_parsing::quote_completed_message::QuoteCompletedMessage;
use crate::message_parsing::timescale_updated_message::TimescaleUpdatedMessage;
use crate::message_parsing::series_loading_message::SeriesLoadingMessage;
use crate::message_parsing::symbol_resolved_message::SymbolResolvedMessage;
use crate::message_parsing::series_completed_message::SeriesCompletedMessage;
use crate::message_parsing::study_loading_message::StudyLoadingMessage;
use crate::message_parsing::study_error_message::StudyErrorMessage;
use crate::message_parsing::study_completed_message::StudyCompletedMessage;
use crate::message_parsing::tickmark_update_message::TickmarkUpdateMessage;
use crate::message_parsing::critical_error_message::CriticalErrorMessage;
use crate::message_parsing::protocol_error_message::ProtocolErrorMessage;
use crate::message_parsing::notify_user_message::NotifyUserMessage;

#[derive(Debug, Clone, EnumAsInner)]
pub enum ParsedTradingViewMessage {
    Ping(usize),    
    ServerHello(ServerHelloMessage),
    QuoteSeriesData(QuoteSeriesDataMessage),
    DataUpdate(DataUpdateMessage),
    QuoteCompleted(QuoteCompletedMessage),
    TimescaleUpdate(TimescaleUpdatedMessage),
    SeriesLoading(SeriesLoadingMessage),
    SymbolResolved(SymbolResolvedMessage),
    SeriesCompleted(SeriesCompletedMessage),
    StudyLoading(StudyLoadingMessage),
    StudyError(StudyErrorMessage),
    StudyCompleted(StudyCompletedMessage),
    TickmarkUpdate(TickmarkUpdateMessage),
    CriticalError(CriticalErrorMessage),
    ProtocolError(ProtocolErrorMessage),
    NotifyUser(NotifyUserMessage),
}

impl ParsedTradingViewMessage {
    pub fn from_string(value: &str) -> SimpleResult<Self> {
        log::trace!("value = {value}");

        // ping messages are not json
        if value.starts_with("~h~") {
            let nonce_str = &value[3..];
            let nonce = nonce_str.parse::<usize>().map_err(|_| box_err!("failed to parse nonce"))?;
            return Ok(ParsedTradingViewMessage::Ping(nonce));
        }

        // all other messages are json
        let parsed_message: Object = miniserde::json::from_str(&value)?;

        // check for server hello message
        if parsed_message.contains_key("javastudies") {
            return Ok(ParsedTradingViewMessage::ServerHello(ServerHelloMessage::from_object(&parsed_message)?));
        }
        
        // all other messages have m property
        let message_type = parsed_message.get("m").ok_or(box_err!("failed to get message_type"))?;
        let message_type = json_utilities::value_to_string(message_type)?;
        if message_type == "qsd" {
            Ok(ParsedTradingViewMessage::QuoteSeriesData(QuoteSeriesDataMessage::from_object(&parsed_message)?))
        } else if message_type == "du" {
            Ok(ParsedTradingViewMessage::DataUpdate(DataUpdateMessage::from_object(&parsed_message)?))
        } else if message_type == "quote_completed" {
            Ok(ParsedTradingViewMessage::QuoteCompleted(QuoteCompletedMessage::from_object(&parsed_message)?))
        } else if message_type == "timescale_update" {
            Ok(ParsedTradingViewMessage::TimescaleUpdate(TimescaleUpdatedMessage::from_object(&parsed_message)?))
        } else if message_type == "series_loading" {
            Ok(ParsedTradingViewMessage::SeriesLoading(SeriesLoadingMessage::from_object(&parsed_message)?))
        } else if message_type == "symbol_resolved" {
            Ok(ParsedTradingViewMessage::SymbolResolved(SymbolResolvedMessage::from_object(&parsed_message)?))
        } else if message_type == "series_completed" {
            Ok(ParsedTradingViewMessage::SeriesCompleted(SeriesCompletedMessage::from_object(&parsed_message)?))
        } else if message_type == "study_loading" {
            Ok(ParsedTradingViewMessage::StudyLoading(StudyLoadingMessage::from_object(&parsed_message)?))
        } else if message_type == "study_error" {
            Ok(ParsedTradingViewMessage::StudyError(StudyErrorMessage::from_object(&parsed_message)?))
        } else if message_type == "study_completed" {
            Ok(ParsedTradingViewMessage::StudyCompleted(StudyCompletedMessage::from_object(&parsed_message)?))
        } else if message_type == "tickmark_update" {
            Ok(ParsedTradingViewMessage::TickmarkUpdate(TickmarkUpdateMessage::from_object(&parsed_message)?))
        } else if message_type == "critical_error" {
            Ok(ParsedTradingViewMessage::CriticalError(CriticalErrorMessage::from_object(&parsed_message)?))
        } else if message_type == "protcol_error" {
            Ok(ParsedTradingViewMessage::ProtocolError(ProtocolErrorMessage::from_object(&parsed_message)?))
        } else if message_type == "notify_user" {
            Ok(ParsedTradingViewMessage::NotifyUser(NotifyUserMessage::from_object(&parsed_message)?))
        } else {
            unimplemented!("message_type = {message_type}")
        }
    }
}
