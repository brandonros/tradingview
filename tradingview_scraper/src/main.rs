mod quote_scraper;
mod candle_scraper;
mod indicator_scraper;
mod utilities;

use std::sync::Arc;
use std::time::Duration;

use async_executor::Executor;
use scraper::Scraper;
use simple_error::SimpleResult;
use tradingview_common::TradingViewIndicators;

use crate::candle_scraper::CandleScraper;
use crate::indicator_scraper::IndicatorScraper;
use crate::quote_scraper::QuoteScraper;

#[macro_rules_attribute::apply(smol_macros::main!)]
async fn main(executor: Arc<Executor<'static>>) -> SimpleResult<()> {
    // logging
    let logging_env = env_logger::Env::default().default_filter_or("debug,websocket_client=info,rustls=info,http_client=info");
    env_logger::Builder::from_env(logging_env).init();

    // init env vars
    dotenvy::from_filename("./.env")?;

    // quote
    let executor_clone = executor.clone();
    let quote_scraper = QuoteScraper {
        auth_token: std::env::var("AUTH_TOKEN")?,
        symbol: "BINANCE:BTCUSDT".to_string(),
    };
    let quote_handle = executor.spawn(Scraper::scrape(
        executor_clone,
        Duration::from_secs(5),
        quote_scraper
    ));

    // candle
    let executor_clone = executor.clone();
    let candle_scraper = CandleScraper {
        auth_token: std::env::var("AUTH_TOKEN")?,
        symbol: "BINANCE:BTCUSDT".to_string(),
        timeframe: "5".to_string(),
        range: 1,
    };
    let candle_handle = executor.spawn(Scraper::scrape(
        executor_clone,
        Duration::from_secs(5),
        candle_scraper
    ));

    // indicator
    let executor_clone = executor.clone();
    let indicator_scraper = IndicatorScraper {
        auth_token: std::env::var("AUTH_TOKEN")?,
        symbol: "BINANCE:BTCUSDT".to_string(),
        timeframe: "5".to_string(),
        range: 1,
        indicator: TradingViewIndicators::generate_vwap_mvwap_ema_crossover(
            1,
            "close".to_string(),
            7,
            "close".to_string(),
            25,
            65,
            51,
            21     
        ),
    };
    let indicator_handle = executor.spawn(Scraper::scrape(
        executor_clone,
        Duration::from_secs(5),
        indicator_scraper
    ));

    quote_handle.await?;
    candle_handle.await?;
    indicator_handle.await?;

    Ok(())
}
