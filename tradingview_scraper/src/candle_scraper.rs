use std::{future::Future, sync::Arc};
use std::pin::Pin;

use scraper::ScrapeOperation;
use simple_error::SimpleResult;
use async_executor::Executor;
use tradingview_common::{TradingViewClientConfig, TradingViewClientMode, TradingViewSymbols};
use tradingview_client::{DefaultTradingViewMessageProcessor, TradingViewClient, TradingViewMessageProcessor};

use crate::utilities;

pub struct CandleScraper {
    pub auth_token: String,
    pub symbol: String,
    pub timeframe: String,
    pub range: usize,
}

impl ScrapeOperation for CandleScraper {
    fn execute(&self, executor: Arc<Executor<'static>>) -> Pin<Box<dyn Future<Output = SimpleResult<()>> + Send + 'static>> {
        let auth_token = self.auth_token.clone();
        let symbol = self.symbol.clone();
        let timeframe = self.timeframe.clone();
        let range = self.range;
        Box::pin(async move {
            // scrape
            let symbol = TradingViewSymbols::build_symbol("splits", None, "regular", &symbol);
            let config = TradingViewClientConfig {
                name: "client".to_string(),
                auth_token: auth_token.to_string(),
                chart_symbols: vec![symbol.to_string()],
                quote_symbols: vec![],
                indicators: vec![],
                timeframe: Some(timeframe.to_string()),
                range: Some(range),
                mode: TradingViewClientMode::Standard
            };
            let message_processor: Arc<Box<dyn TradingViewMessageProcessor + Send + Sync>> = Arc::new(Box::new(DefaultTradingViewMessageProcessor {}));
            let client = TradingViewClient::new(config, message_processor);
            let scrape_result = client.run(executor).await?;

            // parse response
            let timescale_update_message = &scrape_result.timescale_update_messages[0];
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
        })
    }
}