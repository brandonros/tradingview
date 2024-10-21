use std::{future::Future, sync::Arc};
use std::pin::Pin;

use csv_scraper::ScrapeOperation;
use simple_error::{box_err, SimpleResult};
use async_executor::Executor;
use tradingview_common::{TradingViewClientConfig, TradingViewSymbols};
use tradingview_client::{DefaultTradingViewMessageProcessor, TradingViewClient, TradingViewMessageProcessor};

use crate::utilities;

pub struct QuoteScraper {
    pub auth_token: String,
    pub symbol: String,
    pub session: String,
}

impl ScrapeOperation for QuoteScraper {
    fn execute(&self, executor: Arc<Executor<'static>>) -> Pin<Box<dyn Future<Output = SimpleResult<String>> + Send + 'static>> {
        let auth_token = self.auth_token.clone();
        let symbol = self.symbol.clone();
        let session = self.session.clone();
        Box::pin(async move {
            // scrape
            let built_symbol = TradingViewSymbols::build_symbol("splits", None, &session, &symbol);
            let config = TradingViewClientConfig {
                name: "client".to_string(),
                auth_token: auth_token.to_string(),
                chart_symbols: vec![],
                quote_symbols: vec![built_symbol.to_string()],
                indicators: vec![],
                timeframe: None,
                range: None,
            };
            let message_processor: Arc<Box<dyn TradingViewMessageProcessor + Send + Sync>> = Arc::new(Box::new(DefaultTradingViewMessageProcessor {}));
            let client = TradingViewClient::new(config, message_processor);
            let scrape_result = client.scrape(executor).await?;
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
            let quote_age = (now as i64) - (*lp_time as i64);

            // log
            let line = format!("{now},{lp_time},{quote_age},{lp:.2},{ch:.2},{chp:.2},{volume:.4},{prev_close:.2}");
            log::info!("[quote] {line}");

            // return
            Ok(format!("{line}\n"))
        })
    }
}
