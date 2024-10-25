use std::{future::Future, sync::Arc};
use std::pin::Pin;

use async_lock::RwLock;
use csv_scraper::ScrapeOperation;
use simple_error::SimpleResult;
use async_executor::Executor;
use tradingview_common::{TradingViewClientConfig, TradingViewSymbols};
use tradingview_client::{StatefulMessageProcessor, TradingViewClient};

pub struct IndicatorScraper {
    pub symbol: String,
    pub session: String,
    pub timeframe: String,
    pub message_processor: Arc<RwLock<StatefulMessageProcessor>>,
    pub client: Arc<TradingViewClient>,
}

impl IndicatorScraper {
    pub fn new(auth_token: String, symbol: String, session: String, timeframe: String, range: usize, indicator: String) -> Self {
        let built_symbol = TradingViewSymbols::build_symbol("splits", None, &session, &symbol);
        let config = TradingViewClientConfig {
            name: "client".to_string(),
            auth_token: auth_token.to_string(),
            chart_symbols: vec![built_symbol.to_string()],
            quote_symbols: vec![],
            indicators: vec![
                indicator.to_string()
            ],
            timeframe: Some(timeframe.to_string()),
            range: Some(range),
        };
        let message_processor = Arc::new(RwLock::new(StatefulMessageProcessor::default()));
        let client = TradingViewClient::new(config, message_processor.clone());
        let client = Arc::new(client);
        Self { symbol, session, timeframe, client, message_processor }
    }
}

impl ScrapeOperation for IndicatorScraper {
    fn execute(&self, _executor: Arc<Executor<'static>>) -> Pin<Box<dyn Future<Output = SimpleResult<String>> + Send + 'static>> {
        let stateful_message_processor = self.message_processor.clone();
        Box::pin(async move {
            let stateful_message_processor = stateful_message_processor.read().await;
            if let Some(study_update) = &stateful_message_processor.study_update {
                let values = &study_update.values;
                let now = tradingview_common::utilities::now()?;
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
              
                // build line
                let line = format!("{now},{candle_timestamp},{mvwap:.2},{vwap:.2},{long_entry},{short_entry},{ema1:.2},{ema2:.2}");
                log::info!("[indicator] {line}");
              
                // return
                Ok(format!("{line}\n"))
            } else {
                log::warn!("no study update");
                
                // log
                let line = format!(",,,,,,,");
                log::warn!("[indictator] {line}");

                // return
                Ok(format!("{line}\n"))
            }
        })
    }
}