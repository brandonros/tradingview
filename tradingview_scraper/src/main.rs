mod quote_scraper;
mod candle_scraper;
mod indicator_scraper;
mod utilities;

use std::sync::Arc;
use std::time::Duration;

use async_executor::Executor;
use csv_scraper::CsvScraper;
use simple_error::SimpleResult;
use smol_base::smol_main;
use tradingview_common::TradingViewIndicators;

use crate::candle_scraper::CandleScraper;
use crate::indicator_scraper::IndicatorScraper;
use crate::quote_scraper::QuoteScraper;

async fn async_main(executor: &Arc<Executor<'static>>) -> SimpleResult<()> {
    // logging
    let logging_env = env_logger::Env::default().default_filter_or("debug,websocket_client=info,rustls=info,http_client=info,tradingview_client=info,tradingview_common=info");
    env_logger::Builder::from_env(logging_env).init();

    // init env vars
    dotenvy::from_filename("./.env")?;
    let auth_token = std::env::var("AUTH_TOKEN")?;
    let output_dir = std::env::var("OUTPUT_DIR")?;
    let symbol = std::env::var("SYMBOL")?;
    let session = std::env::var("SESSION")?;
    let timeframe = std::env::var("TIMEFRAME")?;

    // get date
    let date = utilities::get_current_date()?;

    // quote
    let executor_clone = executor.clone();
    let quote_scraper = QuoteScraper {
        auth_token: auth_token.clone(),
        symbol: symbol.clone(),
        session: session.clone(),
    };
    let path = format!("{output_dir}/{date}-{0}-{1}-quote.csv", quote_scraper.symbol, quote_scraper.session);
    let quote_csv_scraper = CsvScraper::new(&path, quote_scraper).await?;
    let quote_handle = executor.spawn(quote_csv_scraper.scrape(executor_clone, Duration::from_secs(5)));

    // candle
    let executor_clone = executor.clone();
    let candle_scraper = CandleScraper {
        auth_token: auth_token.clone(),
        symbol: symbol.clone(),
        session: session.clone(),
        timeframe: timeframe.clone(),
        range: 1,
    };
    let path = format!("{output_dir}/{date}-{0}-{1}-{2}-candle.csv", candle_scraper.symbol, candle_scraper.session, candle_scraper.timeframe);
    let candle_csv_scraper = CsvScraper::new(&path, candle_scraper).await?;
    let candle_handle = executor.spawn(candle_csv_scraper.scrape(executor_clone, Duration::from_secs(5)));

    // indicator
    let executor_clone = executor.clone();
    let indicator_scraper = IndicatorScraper {
        auth_token: auth_token.clone(),
        symbol: symbol.clone(),
        session: session.clone(),
        timeframe: timeframe.clone(),
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
    let path = format!("{output_dir}/{date}-{0}-{1}-{2}-indicator.csv", indicator_scraper.symbol, indicator_scraper.session, indicator_scraper.timeframe);
    let indicator_csv_scraper = CsvScraper::new(&path, indicator_scraper).await?;
    let indicator_handle = executor.spawn(indicator_csv_scraper.scrape(executor_clone, Duration::from_secs(5)));

    quote_handle.await?;
    candle_handle.await?;
    indicator_handle.await?;

    Ok(())
}

smol_main!(async_main);
