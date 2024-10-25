use std::{future::Future, sync::Arc};
use std::pin::Pin;

use async_lock::RwLock;
use csv_scraper::ScrapeOperation;
use simple_error::SimpleResult;
use async_executor::Executor;
use tradingview_common::{TradingViewClientConfig, TradingViewSymbols};
use tradingview_client::{StatefulMessageProcessor, TradingViewClient};

pub struct CandleScraper {
    pub symbol: String,
    pub session: String,
    pub timeframe: String,
    pub message_processor: Arc<RwLock<StatefulMessageProcessor>>,
    pub client: Arc<TradingViewClient>,
}

impl CandleScraper {
    pub fn new(auth_token: String, symbol: String, session: String, timeframe: String, range: usize) -> Self {
        let built_symbol = TradingViewSymbols::build_symbol("splits", None, &session, &symbol);
        let config = TradingViewClientConfig {
            name: "client".to_string(),
            auth_token: auth_token.to_string(),
            chart_symbols: vec![built_symbol.to_string()],
            quote_symbols: vec![],
            indicators: vec![],
            timeframe: Some(timeframe.to_string()),
            range: Some(range),
        };
        let message_processor = Arc::new(RwLock::new(StatefulMessageProcessor::default()));
        let client = TradingViewClient::new(config, message_processor.clone());
        let client = Arc::new(client);
        Self { symbol, session, timeframe, client, message_processor }
    }
}

impl ScrapeOperation for CandleScraper {
    fn execute(&self, _executor: Arc<Executor<'static>>) -> Pin<Box<dyn Future<Output = SimpleResult<String>> + Send + 'static>> {
        let timeframe = self.timeframe.clone();
        let stateful_message_processor = self.message_processor.clone();
        Box::pin(async move {
            let stateful_message_processor = stateful_message_processor.read().await;
            if let Some(series_update) = &stateful_message_processor.series_update {
                let candle_start = series_update.timestamp;
                let timeframe_secs = if timeframe == "1" {
                    60 // 1 minute
                } else if timeframe == "5" {
                    300 // 5 minutes
                }else {
                    unimplemented!()
                };
                let candle_end = candle_start + timeframe_secs - 1;
                let now = tradingview_common::utilities::now()?;
                let candle_age = now - candle_start;
                let candle_remaining = (candle_end as i64) - (now as i64); // watch out for underflow?
                let open = series_update.open;
                let high = series_update.high;
                let low = series_update.low;
                let close = series_update.close;
                let volume = series_update.volume;

                // log
                let line = format!("{now},{candle_start},{candle_end},{candle_age},{candle_remaining},{open:.2},{high:.2},{low:.2},{close:.2},{volume:.4}");              
                log::info!("[candle] {line}");

                // return
                Ok(format!("{line}\n"))
            } else {
                log::warn!("no candle update");
                
                // log
                let line = format!(",,,,,,,,");              
                log::warn!("[candle] {line}");

                // return
                Ok(format!("{line}\n"))
            }
        })
    }
}