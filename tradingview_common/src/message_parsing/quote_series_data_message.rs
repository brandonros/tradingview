use miniserde::{json::Object, Deserialize, Serialize};
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

impl QuoteSeriesDataMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("qsd = {parsed_message:?}");
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
        Ok(QuoteSeriesDataMessage {
            quote_session_id,
            quote_update: quote_series_data_update
        })
    }
}
