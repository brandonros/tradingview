use std::future::Future;
use std::pin::Pin;

use scraper::ScrapeOperation;
use simple_error::SimpleResult;
use tradingview_common::TradingViewSymbols;
use tradingview_http_client::TradingViewHttpClient;

pub struct QuoteScraper {
    pub auth_token: String,
    pub symbol: String,
}

impl ScrapeOperation for QuoteScraper {
    fn execute(&self) -> Pin<Box<dyn Future<Output = SimpleResult<()>> + Send + 'static>> {
        let auth_token = self.auth_token.clone();
        let symbol = self.symbol.clone();
        Box::pin(async move {
            let symbol = TradingViewSymbols::build_symbol("splits", None, "regular", &symbol);
            TradingViewHttpClient::scrape_quote(&auth_token, &symbol).await
        })
    }
}
