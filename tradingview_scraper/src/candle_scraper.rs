use std::future::Future;
use std::pin::Pin;

use scraper::ScrapeOperation;
use simple_error::SimpleResult;
use tradingview_common::TradingViewSymbols;
use tradingview_http_client::TradingViewHttpClient;

pub struct CandleScraper {
    pub auth_token: String,
    pub symbol: String,
    pub timeframe: String,
    pub range: usize,
}

impl ScrapeOperation for CandleScraper {
    fn execute(&self) -> Pin<Box<dyn Future<Output = SimpleResult<()>> + Send + 'static>> {
        let auth_token = self.auth_token.clone();
        let symbol = self.symbol.clone();
        let timeframe = self.timeframe.clone();
        let range = self.range;
        Box::pin(async move {
            let symbol = TradingViewSymbols::build_symbol("splits", None, "regular", &symbol);
            TradingViewHttpClient::scrape_candles(&auth_token, &symbol, &timeframe, range).await
        })
    }
}