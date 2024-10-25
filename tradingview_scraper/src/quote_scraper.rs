use std::{future::Future, sync::Arc};
use std::pin::Pin;

use async_lock::RwLock;
use csv_scraper::ScrapeOperation;
use simple_error::{box_err, SimpleResult};
use async_executor::Executor;
use tradingview_common::{TradingViewClientConfig, TradingViewSymbols};
use tradingview_client::{StatefulMessageProcessor, TradingViewClient};

pub struct QuoteScraper {
    pub symbol: String,
    pub session: String,
    pub message_processor: Arc<RwLock<StatefulMessageProcessor>>,
    pub client: Arc<TradingViewClient>,
}

impl QuoteScraper {
    pub fn new(auth_token: String, symbol: String, session: String) -> Self {
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
        let message_processor = Arc::new(RwLock::new(StatefulMessageProcessor::default()));
        let client = TradingViewClient::new(config, message_processor.clone());
        let client = Arc::new(client);
        Self { symbol, session, message_processor, client }
    }
}

impl ScrapeOperation for QuoteScraper {
    fn execute(&self, _executor: Arc<Executor<'static>>) -> Pin<Box<dyn Future<Output = SimpleResult<String>> + Send + 'static>> {
        let stateful_message_processor = self.message_processor.clone();
        Box::pin(async move {
            let stateful_message_processor = stateful_message_processor.read().await;
            if let Some(quote_update) = &stateful_message_processor.quote_update {
                let volume = quote_update.volume.as_ref().ok_or(box_err!("no volume"))?;
                let lp = quote_update.lp.as_ref().ok_or(box_err!("no lp"))?;
                let lp_time = quote_update.lp_time.as_ref().ok_or(box_err!("no lp_time"))?;    
                let ch = quote_update.ch.as_ref().ok_or(box_err!("no ch"))?;
                let chp = quote_update.chp.as_ref().ok_or(box_err!("no chp"))?;

                let prev_close = lp - ch;
                let now = tradingview_common::utilities::now()?;
                let quote_age = (now as i64) - (*lp_time as i64);

                // log
                let line = format!("{now},{lp_time},{quote_age},{lp:.2},{ch:.2},{chp:.2},{volume:.4},{prev_close:.2}");
                log::info!("[quote] {line}");

                // return
                Ok(format!("{line}\n"))
            } else {
                log::warn!("no quote update");
                
                // log
                let line = format!(",,,,,,,");
                log::warn!("[quote] {line}");

                // return
                Ok(format!("{line}\n"))
            }
        })
    }
}
