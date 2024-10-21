use miniserde::{json::{Number, Object}, Deserialize, Serialize};
use simple_error::{box_err, SimpleResult};

use crate::json_utilities;

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
pub struct DataUpdateMessage {
    pub chart_session_id: String,
    pub update_key: String,
    pub series_updates: Option<Vec<SeriesUpdate>>,
    pub study_updates: Option<Vec<StudyUpdate>>,
}

impl DataUpdateMessage {
    pub fn from_object(parsed_message: &Object) -> SimpleResult<Self> {
        log::debug!("du = {parsed_message:?}");
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
            let message = if update_value.contains_key("s") {
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
                DataUpdateMessage {
                    chart_session_id,
                    update_key: update_key.to_string(),
                    series_updates: Some(series_updates),
                    study_updates: None
                }
            } else {
                // watch out for weird du message with no updates on it? ns property
                DataUpdateMessage {
                    chart_session_id,
                    update_key: update_key.to_string(),
                    series_updates: None,
                    study_updates: None
                }
            };
            Ok(message)
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
            let message = DataUpdateMessage {
                chart_session_id,
                update_key: update_key.to_string(),
                series_updates: None,
                study_updates: Some(study_updates)
            };
            Ok(message)
        } else {
            todo!("update_key = {update_key}");
        }
    }
}
