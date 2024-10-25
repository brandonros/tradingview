use async_trait::async_trait;
use simple_error::SimpleResult;
use tradingview_common::{ParsedTradingViewMessage, QuoteSeriesDataUpdate, SeriesUpdate, StudyUpdate, TimescaleUpdate};

use crate::message_processor::TradingViewMessageProcessor;

#[derive(Debug)]
pub struct StatefulMessageProcessor {
    // quote
    pub quote_update: Option<QuoteSeriesDataUpdate>,

    // timescale update
    pub timescale_update: Option<TimescaleUpdate>,

    // study updates
    pub study_update: Option<StudyUpdate>,

    // series update
    pub series_update: Option<SeriesUpdate>,

    // time
    pub last_quote_update: Option<u64>,
    pub last_timescale_update: Option<u64>,
    pub last_series_update: Option<u64>,
    pub last_study_update: Option<u64>,
  }

impl Default for StatefulMessageProcessor {
    fn default() -> Self {
        Self {
        // quote
        quote_update: None,

        // timescale update
        timescale_update: None,

        // study updates
        study_update: None,

        // series updates
        series_update: None,

        // time
        last_quote_update: None,
        last_timescale_update: None,          
        last_series_update: None,
        last_study_update: None,
      }
    }
}

#[async_trait]
impl TradingViewMessageProcessor for StatefulMessageProcessor {
    async fn process_message(
        &mut self,
        _name: String,
        parsed_message: ParsedTradingViewMessage,
    ) -> SimpleResult<()> {
        match &parsed_message {
            ParsedTradingViewMessage::Ping(_nonce) => (),
            ParsedTradingViewMessage::QuoteSeriesData(quote_series_data_message) => {
              match &mut self.quote_update {
                None => {
                  // first quote update
                  self.quote_update = Some(quote_series_data_message.quote_update.clone());
                }
                Some(quote_update) => {
                  // subsequent quote updates
                  if let Some(volume) = quote_series_data_message.quote_update.volume {
                    quote_update.volume = Some(volume);
                  }
                  if let Some(ch) = quote_series_data_message.quote_update.ch {
                    quote_update.ch = Some(ch);
                  }
                  if let Some(chp) = quote_series_data_message.quote_update.chp {
                    quote_update.chp = Some(chp);
                  }
                  if let Some(rch) = quote_series_data_message.quote_update.rch {
                    quote_update.rch = Some(rch);
                  }
                  if let Some(rchp) = quote_series_data_message.quote_update.rchp {
                    quote_update.rchp = Some(rchp);
                  }
                  if let Some(rtc) = quote_series_data_message.quote_update.rtc {
                    quote_update.rtc = Some(rtc);
                  }
                  if let Some(rtc_time) = quote_series_data_message.quote_update.rtc_time {
                    quote_update.rtc_time = Some(rtc_time);
                  }
                  if let Some(lp) = quote_series_data_message.quote_update.lp {
                    quote_update.lp = Some(lp);
                  }
                  if let Some(lp_time) = quote_series_data_message.quote_update.lp_time {
                    quote_update.lp_time = Some(lp_time);
                  }
                  if let Some(ask) = quote_series_data_message.quote_update.ask {
                    quote_update.ask = Some(ask);
                  }
                  if let Some(ask_size) = quote_series_data_message.quote_update.ask_size {
                    quote_update.ask_size = Some(ask_size);
                  }
                  if let Some(bid) = quote_series_data_message.quote_update.bid {
                    quote_update.bid = Some(bid);
                  }
                  if let Some(bid_size) = quote_series_data_message.quote_update.bid_size {
                    quote_update.bid_size = Some(bid_size);
                  }
                },
                
              }

              self.last_quote_update = Some(tradingview_common::utilities::now()?);
          },
          ParsedTradingViewMessage::TimescaleUpdate(timescale_update_message) => {
            // timescale updates
            match &timescale_update_message.updates {
              Some(updates) => {
                if updates.len() == 0 {
                  log::warn!("empty timescale updates?");
                } else if updates.len() == 1 {
                  self.timescale_update = Some(updates[0].clone());
                  self.last_timescale_update = Some(tradingview_common::utilities::now()?);
                } else {
                  log::warn!("multiple timescale updates?");
                }
              },
              None => {
                log::warn!("no timescale updates?");
              }
            }
          },
          ParsedTradingViewMessage::DataUpdate(data_update_message) => {
            // study updates (indicators)
            match &data_update_message.study_updates {
              Some(study_updates) => {
                if study_updates.len() == 0 {
                  log::warn!("empty study updates?");
                } else if study_updates.len() == 1 {
                  self.study_update = Some(study_updates[0].clone());
                  self.last_study_update = Some(tradingview_common::utilities::now()?);
                } else {
                  log::warn!("multiple study updates?");
                }
              },
              None => ()
            }

            // series updates (candles)
            match &data_update_message.series_updates {
              Some(series_updates) => {
                if series_updates.len() == 0 {
                  log::warn!("empty study updates?");
                } else if series_updates.len() == 1 {
                  self.series_update = Some(series_updates[0].clone());
                  self.last_series_update = Some(tradingview_common::utilities::now()?);
                } else {
                  log::warn!("multiple series updates?");
                }
              },
              None => ()
            }
          }
          _ => {
              log::info!("unknown_message = {:?}", parsed_message);
          }
        }

        Ok(())
    }
}
