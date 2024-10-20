use enum_as_inner::EnumAsInner;
use miniserde::{Deserialize, Serialize};
use miniserde::json::{Number, Object};
use simple_error::{box_err, SimpleResult};

use crate::json_utilities;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteSeriesDataUpdate {
    pub symbol: String,
    pub volume: Option<f64>,
    pub ch: Option<f64>,
    pub chp: Option<f64>,
    pub rch: Option<f64>,
    pub rchp: Option<f64>,
    pub rtc: Option<f64>,
    pub rtc_time: Option<u64>,
    pub lp: Option<f64>,
    pub lp_time: Option<u64>,
    pub ask: Option<f64>,
    pub ask_size: Option<f64>,
    pub bid: Option<f64>,
    pub bid_size: Option<f64>,
    pub trade_loaded: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteSeriesDataMessage {
    pub quote_session_id: String,
    pub quote_update: QuoteSeriesDataUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataUpdateMessage {
    pub chart_session_id: String,
    pub update_key: String,
    pub series_updates: Option<Vec<SeriesUpdate>>,
    pub study_updates: Option<Vec<StudyUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteCompletedMessage {
    pub quote_session_id: String,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimescaleUpdate {
    pub index: u64,
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesUpdate {
    pub index: Number,
    pub timestamp: Number,
    pub open: Number,
    pub high: Number,
    pub low: Number,
    pub close: Number,
    pub volume: Number,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyUpdate {
    pub index: Number,
    pub values: Vec<Number>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimescaleUpdatedMessage {
    pub chart_session_id: String,
    pub update_key: Option<String>,
    pub updates: Option<Vec<TimescaleUpdate>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHelloMessage {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesLoadingMessage {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolResolvedMessage {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesCompletedMessage {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyLoadingMessage {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyErrorMessage {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyCompletedMessage {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickmarkUpdateMessage {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalErrorMessage {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolErrorMessage {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyUserMessage {

}

#[derive(Debug, Clone, EnumAsInner)]
pub enum ParsedTradingViewMessage {
    ServerHello(ServerHelloMessage),
    Ping(usize),
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
            return Ok(ParsedTradingViewMessage::ServerHello(ServerHelloMessage {

            }));
        }
        
        // all other messages have m property
        let message_type = parsed_message.get("m").ok_or(box_err!("failed to get message_type"))?;
        let message_type = json_utilities::value_to_string(message_type)?;
        if message_type == "qsd" {
            //log::info!("qsd = {parsed_message:?}");
            let p = parsed_message.get("p").ok_or(box_err!("failed to get p"))?;
            let p = json_utilities::value_to_array(p)?;
            let quote_session_id = &p[0];
            let quote_session_id = json_utilities::value_to_string(&quote_session_id)?;
            let update = &p[1];
            let update = json_utilities::value_to_object(&update)?;
            let symbol = json_utilities::value_to_string(update.get("n").ok_or(box_err!("failed to get n"))?)?;
            let v = json_utilities::value_to_object(update.get("v").ok_or(box_err!("failed to get v"))?)?;
            //let v_keys = v.keys().collect::<Vec<&String>>();
            // TODO: check more combinations
            let quote_series_data_update = QuoteSeriesDataUpdate {
                symbol,

                // BTC is f64 volume, SPY is u64, perma-cst to f64
                volume: if v.contains_key("volume") { Some(json_utilities::value_to_f64_cast(v.get("volume").ok_or(box_err!("failed to get v"))?)?) } else { None },

                ch: if v.contains_key("ch") { Some(json_utilities::value_to_f64(v.get("ch").ok_or(box_err!("failed to get ch"))?)?) } else { None },
                chp: if v.contains_key("chp") { Some(json_utilities::value_to_f64(v.get("chp").ok_or(box_err!("failed to get chp"))?)?) } else { None },

                rch: if v.contains_key("rch") && !json_utilities::is_null(&v, "rch")? { Some(json_utilities::value_to_f64(v.get("rch").ok_or(box_err!("failed to get rch"))?)?) } else { None },
                rchp: if v.contains_key("rchp") && !json_utilities::is_null(&v, "rchp")? { Some(json_utilities::value_to_f64(v.get("rchp").ok_or(box_err!("failed to get rchp"))?)?) } else { None },

                lp: if v.contains_key("lp") { Some(json_utilities::value_to_f64(v.get("lp").ok_or(box_err!("failed to get lp"))?)?) } else { None },
                lp_time: if v.contains_key("lp_time") { Some(json_utilities::value_to_u64(v.get("lp_time").ok_or(box_err!("failed to get lp_time"))?)?) } else { None },

                rtc: if v.contains_key("rtc") && !json_utilities::is_null(&v, "rtc")? { Some(json_utilities::value_to_f64(v.get("rtc").ok_or(box_err!("failed to get rtc"))?)?) } else { None },
                rtc_time: if v.contains_key("rtc_time") && !json_utilities::is_null(&v, "rtc_time")? { Some(json_utilities::value_to_u64(v.get("rtc_time").ok_or(box_err!("failed to get rtc_time"))?)?) } else { None },

                ask: if v.contains_key("ask") { Some(json_utilities::value_to_f64(v.get("ask").ok_or(box_err!("failed to get ask"))?)?) } else { None },
                ask_size: if v.contains_key("ask_size") { Some(json_utilities::value_to_f64_cast(v.get("ask_size").ok_or(box_err!("failed to get ask_size"))?)?) } else { None },

                bid: if v.contains_key("bid") { Some(json_utilities::value_to_f64(v.get("bid").ok_or(box_err!("failed to get bid"))?)?) } else { None },
                bid_size: if v.contains_key("bid_size") { Some(json_utilities::value_to_f64_cast(v.get("bid_size").ok_or(box_err!("failed to get bid_size"))?)?) } else { None },

                trade_loaded: if v.contains_key("trade_loaded") { Some(json_utilities::value_to_bool(v.get("trade_loaded").ok_or(box_err!("failed to get trade_loaded"))?)?) } else { None },

                // TODO: more fields?
            };
            Ok(ParsedTradingViewMessage::QuoteSeriesData(QuoteSeriesDataMessage {
                quote_session_id,
                quote_update: quote_series_data_update
            }))
        } else if message_type == "du" {
            //log::info!("du = {parsed_message:?}");
            let p = parsed_message.get("p").ok_or(box_err!("failed to get p"))?;
            let p = json_utilities::value_to_array(p)?;
            let chart_session_id = &p[0];
            let chart_session_id = json_utilities::value_to_string(&chart_session_id)?;
            let update = &p[1];
            let update = json_utilities::value_to_object(&update)?;
            let update_keys = update.keys().collect::<Vec<&String>>();
            assert!(update_keys.len() == 1);
            let update_key = update_keys[0];
            if update_key == "sds_1" { // series
                let update_value = json_utilities::value_to_object(update.get(update_key).ok_or(box_err!("failed to get update_key"))?)?;
                if update_value.contains_key("s") {
                    let s = update_value.get("s").ok_or(box_err!("failed to get s"))?;
                    let s = json_utilities::value_to_array(s)?;
                    let series_updates = s.iter().map(|element| {
                        // value -> object
                        let element = json_utilities::value_to_object(&element).expect("failed to cast");
                        
                        // pluck i (index)
                        let i = element.get("i").expect("failed to get i");
                        let i = json_utilities::value_to_number(i).expect("failed to cast");
    
                        // pluck v (values)
                        let v = element.get("v").expect("failed to get v");
                        let v = json_utilities::value_to_array(v).expect("failed to cast");
    
                        // pluck out of values
                        let timestamp = json_utilities::value_to_number(&v[0]).expect("failed to cast");
                        let open = json_utilities::value_to_number(&v[1]).expect("failed to cast");
                        let high = json_utilities::value_to_number(&v[2]).expect("failed to cast");
                        let low = json_utilities::value_to_number(&v[3]).expect("failed to cast");
                        let close = json_utilities::value_to_number(&v[4]).expect("failed to cast");
                        let volume = json_utilities::value_to_number(&v[5]).expect("failed to cast");
    
                        // return
                        SeriesUpdate {
                            index: i,
                            timestamp,
                            open,
                            high,
                            low,
                            close,
                            volume,
                        }
                    }).collect::<Vec<_>>();
                    Ok(ParsedTradingViewMessage::DataUpdate(DataUpdateMessage {
                        chart_session_id,
                        update_key: update_key.to_string(),
                        series_updates: Some(series_updates),
                        study_updates: None
                    }))
                } else {
                    // watch out for weird du message with no updates on it? ns property
                    Ok(ParsedTradingViewMessage::DataUpdate(DataUpdateMessage {
                        chart_session_id,
                        update_key: update_key.to_string(),
                        series_updates: None,
                        study_updates: None
                    }))
                }
            } else if update_key == "st1" || update_key == "st2" { // study
                let update_value = json_utilities::value_to_object(update.get(update_key).ok_or(box_err!("failed to get update_key"))?)?;
                let st = update_value.get("st").ok_or(box_err!("failed to get st"))?;
                let st = json_utilities::value_to_array(st)?;
                let study_updates = st.iter().map(|element| {
                    // value -> object
                    let element = json_utilities::value_to_object(&element).expect("failed to cast");

                    // pluck i (index)
                    let i = element.get("i").expect("failed to get i");
                    let i = json_utilities::value_to_number(i).expect("failed to cast");

                    // pluck v (values)
                    let v = element.get("v").expect("failed to get v");
                    let v = json_utilities::value_to_array(v).expect("failed to cast");
                    let v = v.iter().map(|value| json_utilities::value_to_number(value).expect("failed to cast")).collect::<Vec<_>>();

                    StudyUpdate {
                        index: i,
                        values: v
                    }
                }).collect::<Vec<_>>();
                Ok(ParsedTradingViewMessage::DataUpdate(DataUpdateMessage {
                    chart_session_id,
                    update_key: update_key.to_string(),
                    series_updates: None,
                    study_updates: Some(study_updates)
                }))
            } else {
                todo!("update_key = {update_key}");
            }
        } else if message_type == "quote_completed" {
            //log::info!("quote_completed = {parsed_message:?}");
            let p = parsed_message.get("p").ok_or(box_err!("failed to get p"))?;
            let p = json_utilities::value_to_array(p)?;
            let quote_session_id = &p[0];
            let quote_session_id = json_utilities::value_to_string(&quote_session_id)?;
            let symbol = &p[1];
            let symbol = json_utilities::value_to_string(&symbol)?;
            Ok(ParsedTradingViewMessage::QuoteCompleted(QuoteCompletedMessage {
                quote_session_id,
                symbol
            }))
        } else if message_type == "timescale_update" {
            //log::info!("timescale_update parsed_message = {parsed_message:?}");
            let p = parsed_message.get("p").ok_or(box_err!("failed to get p"))?;
            let p = json_utilities::value_to_array(p)?;
            let chart_session_id = &p[0];
            let chart_session_id = json_utilities::value_to_string(&chart_session_id)?;
            let update = &p[1];
            let update = json_utilities::value_to_object(&update)?;
            let update_keys = update.keys().collect::<Vec<&String>>();
            if update_keys.len() == 0 {
                // weird timescale_update with index/zoffset/changes/marks but nothing of any interest/importance
                Ok(ParsedTradingViewMessage::TimescaleUpdate(TimescaleUpdatedMessage {
                    chart_session_id,
                    update_key: None,
                    updates: None
                }))
            } else if update_keys.len() == 1 {
                let update_key = update_keys[0];
                let update_value = json_utilities::value_to_object(update.get(update_key).ok_or(box_err!("failed to get update_key"))?)?;
                let s = update_value.get("s").ok_or(box_err!("failed to get s"))?;
                let s = json_utilities::value_to_array(s)?;
                let timescale_updates = s.iter().map(|element| {
                    // value -> object
                    let element = json_utilities::value_to_object(&element).expect("failed to cast");
                    
                    // pluck i (index)
                    let i = element.get("i").expect("failed to get i");
                    let i = json_utilities::value_to_u64(i).expect("failed to cast");

                    // pluck v (values)
                    let v = element.get("v").expect("failed to get v");
                    let v = json_utilities::value_to_array(v).expect("failed to cast");

                    // pluck out of values
                    let timestamp = json_utilities::value_to_u64_cast(&v[0]).expect("failed to cast");
                    let open = json_utilities::value_to_f64(&v[1]).expect("failed to cast");
                    let high = json_utilities::value_to_f64(&v[2]).expect("failed to cast");
                    let low = json_utilities::value_to_f64(&v[3]).expect("failed to cast");
                    let close = json_utilities::value_to_f64(&v[4]).expect("failed to cast");
                    let volume = json_utilities::value_to_f64_cast(&v[5]).expect("failed to cast");

                    // return
                    TimescaleUpdate {
                        index: i,
                        timestamp,
                        open,
                        high,
                        low,
                        close,
                        volume,
                    }
                }).collect::<Vec<_>>();
                Ok(ParsedTradingViewMessage::TimescaleUpdate(TimescaleUpdatedMessage {
                    chart_session_id,
                    update_key: Some(update_key.to_string()),
                    updates: Some(timescale_updates)
                }))
            } else {
                unimplemented!()
            }
        } else if message_type == "series_loading" {
            log::info!("series_loading = {parsed_message:?}");
            Ok(ParsedTradingViewMessage::SeriesLoading(SeriesLoadingMessage {
                
            }))
        } else if message_type == "symbol_resolved" {
            log::info!("symbol_resolved = {parsed_message:?}");
            Ok(ParsedTradingViewMessage::SymbolResolved(SymbolResolvedMessage {
                
            }))
        } else if message_type == "series_completed" {
            log::info!("series_completed = {parsed_message:?}");
            Ok(ParsedTradingViewMessage::SeriesCompleted(SeriesCompletedMessage {
                
            }))
        } else if message_type == "study_loading" {
            log::info!("study_loading = {parsed_message:?}");
            Ok(ParsedTradingViewMessage::StudyLoading(StudyLoadingMessage {
                
            }))
        } else if message_type == "study_error" {
            log::info!("study_error = {parsed_message:?}");
            Ok(ParsedTradingViewMessage::StudyError(StudyErrorMessage {
                
            }))
        } else if message_type == "study_completed" {
            log::info!("study_completed = {parsed_message:?}");
            Ok(ParsedTradingViewMessage::StudyCompleted(StudyCompletedMessage {
                
            }))
        } else if message_type == "tickmark_update" {
            log::info!("tickmark_update = {parsed_message:?}");
            Ok(ParsedTradingViewMessage::TickmarkUpdate(TickmarkUpdateMessage {
                
            }))
        } else if message_type == "critical_error" {
            log::info!("critical_error = {parsed_message:?}");
            Ok(ParsedTradingViewMessage::CriticalError(CriticalErrorMessage {
                
            }))
        } else if message_type == "protcol_error" {
            log::info!("protcol_error = {parsed_message:?}");
            Ok(ParsedTradingViewMessage::ProtocolError(ProtocolErrorMessage {
                
            }))
        } else if message_type == "notify_user" {
            log::info!("notify_user = {parsed_message:?}");
            Ok(ParsedTradingViewMessage::NotifyUser(NotifyUserMessage {
                
            }))
        } else {
            unimplemented!("message_type = {message_type}")
        }
    }
}
