use miniserde::{json::Object, Deserialize, Serialize};
use simple_error::{box_err, SimpleResult};

use crate::json_utilities;

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
pub struct TimescaleUpdatedMessage {
    pub chart_session_id: String,
    pub update_key: Option<String>,
    pub updates: Option<Vec<TimescaleUpdate>>
}

impl TimescaleUpdatedMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("timescale_update parsed_message = {parsed_message:?}");
        let p = parsed_message.get("p").ok_or(box_err!("failed to get p"))?;
        let p = json_utilities::value_to_array(p)?;
        let chart_session_id = &p[0];
        let chart_session_id = json_utilities::value_to_string(&chart_session_id)?;
        let update = &p[1];
        let update = json_utilities::value_to_object(&update)?;
        let update_keys = update.keys().collect::<Vec<&String>>();
        if update_keys.len() == 0 {
            // weird timescale_update with index/zoffset/changes/marks but nothing of any interest/importance
            Ok(TimescaleUpdatedMessage {
                chart_session_id,
                update_key: None,
                updates: None
            })
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
            Ok(TimescaleUpdatedMessage {
                chart_session_id,
                update_key: Some(update_key.to_string()),
                updates: Some(timescale_updates)
            })
        } else {
            unimplemented!()
        }
    }
}
