use std::future::Future;
use std::pin::Pin;

use scraper::ScrapeOperation;
use simple_error::SimpleResult;
use tradingview_common::TradingViewSymbols;
use tradingview_http_client::TradingViewHttpClient;

pub struct IndicatorScraper {
    pub auth_token: String,
    pub symbol: String,
    pub timeframe: String,
    pub range: usize,
    pub indicator: String,
}

impl ScrapeOperation for IndicatorScraper {
    fn execute(&self) -> Pin<Box<dyn Future<Output = SimpleResult<()>> + Send + 'static>> {
        let auth_token = self.auth_token.clone();
        let symbol = self.symbol.clone();
        let timeframe = self.timeframe.clone();
        let range = self.range;
        let indicator = self.indicator.clone();
        Box::pin(async move {
            let symbol = TradingViewSymbols::build_symbol("splits", None, "regular", &symbol);
            TradingViewHttpClient::scrape_indicator(&auth_token, &symbol, &timeframe, range, &indicator).await
        })
    }
}