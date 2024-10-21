use std::{future::Future, sync::Arc};
use std::pin::Pin;

use csv_scraper::ScrapeOperation;
use simple_error::SimpleResult;
use async_executor::Executor;
use tradingview_common::{TradingViewClientConfig, TradingViewClientMode, TradingViewSymbols};
use tradingview_client::{DefaultTradingViewMessageProcessor, TradingViewClient, TradingViewMessageProcessor};

use crate::utilities;

pub struct IndicatorScraper {
    pub auth_token: String,
    pub symbol: String,
    pub session: String,
    pub timeframe: String,
    pub range: usize,
    pub indicator: String,
}

impl ScrapeOperation for IndicatorScraper {
    fn execute(&self, executor: Arc<Executor<'static>>) -> Pin<Box<dyn Future<Output = SimpleResult<String>> + Send + 'static>> {
        let auth_token = self.auth_token.clone();
        let symbol = self.symbol.clone();
        let session = self.session.clone();
        let timeframe = self.timeframe.clone();
        let range = self.range;
        let indicator = self.indicator.clone();
        Box::pin(async move {
            // scrape
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
                mode: TradingViewClientMode::Standard
            };
            let message_processor: Arc<Box<dyn TradingViewMessageProcessor + Send + Sync>> = Arc::new(Box::new(DefaultTradingViewMessageProcessor {}));
            let client = TradingViewClient::new(config, message_processor);
            let scrape_result = client.run(executor).await?;
            drop(client);

            // parse response
            let study_update_message = &scrape_result.study_data_update_messages[0];
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

            // build line
            let line = format!("{now},{candle_timestamp},{mvwap:.2},{vwap:.2},{long_entry},{short_entry},{ema1:.2},{ema2:.2}");
            log::info!("[indicator] {line}");

            // return
            Ok(format!("{line}\n"))
        })
    }
}