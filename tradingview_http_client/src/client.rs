
use http_client::HttpClient;
use simple_error::{box_err, SimpleResult};
use tradingview_common::{TradingViewClientConfig, TradingViewClientMode, TradingViewScrapeResult};

use crate::utilities;

pub struct TradingViewHttpClient {
}

impl TradingViewHttpClient {
    pub async fn scrape_quote(auth_token: &str, symbol: &str) -> SimpleResult<()> {
        // scrape
        let request_body = TradingViewClientConfig {
            name: "client".to_string(),
            auth_token: auth_token.to_string(),
            chart_symbols: vec![],
            quote_symbols: vec![symbol.to_string()],
            indicators: vec![],
            timeframe: None,
            range: None,
            mode: TradingViewClientMode::Standard
        };
        let response_body: TradingViewScrapeResult = HttpClient::json_request("http://127.0.0.1:3000/scrape", &request_body).await?;

        // parse response
        let quote_last_price_message = &response_body.quote_last_price_messages[0];
        let quote_update = &quote_last_price_message.quote_update;
        let volume = quote_update.volume.as_ref().ok_or(box_err!("no volume"))?;
        let lp = quote_update.lp.as_ref().ok_or(box_err!("no lp"))?;
        let lp_time = quote_update.lp_time.as_ref().ok_or(box_err!("no lp_time"))?;    
        let ch = quote_update.ch.as_ref().ok_or(box_err!("no ch"))?;
        let chp = quote_update.chp.as_ref().ok_or(box_err!("no chp"))?;
        let prev_close = lp - ch;
        let now = utilities::now()?;
        let quote_age = now - lp_time;

        // log
        log::info!("[quote] now = {now} lp_time = {lp_time} quote_age = {quote_age}s lp = {lp} ch = {ch} chp = {chp} volume = {volume} prev_close = {prev_close}");

        Ok(())
    }

    pub async fn scrape_candles(auth_token: &str, symbol: &str, timeframe: &str, range: usize) -> SimpleResult<()> {
        // scrape
        let request_body = TradingViewClientConfig {
            name: "client".to_string(),
            auth_token: auth_token.to_string(),
            chart_symbols: vec![symbol.to_string()],
            quote_symbols: vec![],
            indicators: vec![],
            timeframe: Some(timeframe.to_string()),
            range: Some(range),
            mode: TradingViewClientMode::Standard
        };
        let response_body: TradingViewScrapeResult = HttpClient::json_request("http://127.0.0.1:3000/scrape", &request_body).await?;

        // parse request
        let timescale_update_message = &response_body.timescale_update_messages[0];
        let updates = timescale_update_message.updates.as_ref().unwrap();
        let current_candle = &updates[updates.len() - 1];
        let candle_start = current_candle.timestamp;
        let timeframe_secs = if timeframe == "5" {
            300 // 5 minutes
        } else {
            unimplemented!()
        };
        let candle_end = candle_start + timeframe_secs - 1;
        let now = utilities::now()?;
        let candle_age = now - candle_start;
        let candle_remaining = candle_end - now;
        let open = current_candle.open;
        let high = current_candle.high;
        let low = current_candle.low;
        let close = current_candle.close;
        let volume = current_candle.volume;

        // log              
        log::info!("[candles] now = {now} candle_start = {candle_start} candle_end = {candle_end} candle_age = {candle_age}s candle_remaining = {candle_remaining}s open = {open} high = {high} low = {low} close = {close} volume = {volume}");

        Ok(())
    }

    pub async fn scrape_indicator(auth_token: &str, symbol: &str, timeframe: &str, range: usize, indicator: &str) -> SimpleResult<()> {
        // scrape
        let request_body = TradingViewClientConfig {
            name: "client".to_string(),
            auth_token: auth_token.to_string(),
            chart_symbols: vec![symbol.to_string()],
            quote_symbols: vec![],
            indicators: vec![
                indicator.to_string()
            ],
            timeframe: Some(timeframe.to_string()),
            range: Some(range),
            mode: TradingViewClientMode::Standard
        };
        let response_body: TradingViewScrapeResult = HttpClient::json_request("http://127.0.0.1:3000/scrape", &request_body).await?;

        // parse response
        let study_update_message = &response_body.study_data_update_messages[0];
        let study_updates = study_update_message.study_updates.as_ref().unwrap();
        let study_update = &study_updates[study_updates.len() - 1];
        let values = &study_update.values;
        let now = utilities::now()?;
        let candle_timestamp = &values[0];
        let mvwap = &values[1];
        let vwap = &values[2];
        let long_entry = &values[3];
        let short_entry = &values[4];
        let _buy_alert = &values[5];
        let _risky = &values[6];
        let _wait = &values[7];
        let _enter_here = &values[8];
        let ema1 = &values[9];
        let ema2 = &values[10];
        let _senkou_a = &values[11];
        let _senkou_b = &values[12];

        // log
        log::info!("[indicator] now = {now} candle_timestamp = {candle_timestamp} mvwap = {mvwap} vwap = {vwap} long_entry = {long_entry} short_entry = {short_entry} ema1 = {ema1} ema2 = {ema2}");

        Ok(())
    }
}
