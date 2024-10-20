use std::{future::Future, sync::Arc};
use std::pin::Pin;

use scraper::ScrapeOperation;
use simple_error::{box_err, SimpleResult};
use async_executor::Executor;
use tradingview_common::{TradingViewClientConfig, TradingViewClientMode, TradingViewSymbols};
use tradingview_client::{DefaultTradingViewMessageProcessor, TradingViewClient, TradingViewMessageProcessor};

use crate::utilities;

pub struct QuoteScraper {
    pub auth_token: String,
    pub symbol: String,
}

impl ScrapeOperation for QuoteScraper {
    fn execute(&self, executor: Arc<Executor<'static>>) -> Pin<Box<dyn Future<Output = SimpleResult<()>> + Send + 'static>> {
        let auth_token = self.auth_token.clone();
        let symbol = self.symbol.clone();
        Box::pin(async move {
            // scrape
            let symbol = TradingViewSymbols::build_symbol("splits", None, "regular", &symbol);
            let config = TradingViewClientConfig {
                name: "client".to_string(),
                auth_token: auth_token.to_string(),
                chart_symbols: vec![],
                quote_symbols: vec![symbol.to_string()],
                indicators: vec![],
                timeframe: None,
                range: None,
                mode: TradingViewClientMode::Standard
            };
            let message_processor: Arc<Box<dyn TradingViewMessageProcessor + Send + Sync>> = Arc::new(Box::new(DefaultTradingViewMessageProcessor {}));
            let client = TradingViewClient::new(config, message_processor);
            let scrape_result = client.run(executor).await?;
            drop(client);

            // parse response
            let quote_last_price_message = &scrape_result.quote_last_price_messages[0];
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
        })
    }
}
