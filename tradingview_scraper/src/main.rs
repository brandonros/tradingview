mod config;
mod quote_scraper;
mod candle_scraper;
mod indicator_scraper;

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
use crate::config::Config;

async fn spawn_quote_csv_scraper(executor: Arc<Executor<'static>>, config: Config) -> SimpleResult<()> {
    // build scraper
    let quote_scraper = QuoteScraper::new(
        config.auth_token.clone(),
        config.symbol.clone(),
        config.session.clone(),
    );

    // spawn
    let client = quote_scraper.client.clone();
    let executor_clone = executor.clone();
    let handle = executor.spawn(async move {
        client.subscribe(executor_clone).await
    });
    handle.detach();

    // build csv scraper
    let date = tradingview_common::utilities::get_current_date()?;
    let path = format!("{0}/{date}-{1}-{2}-quote.csv", config.output_dir, quote_scraper.symbol, quote_scraper.session);
    let quote_csv_scraper = CsvScraper::new(&path, quote_scraper).await?;
    let executor_clone = executor.clone();
    quote_csv_scraper.scrape(executor_clone, Duration::from_secs(5)).await
}

async fn spawn_candle_csv_scraper(executor: Arc<Executor<'static>>, config: Config) -> SimpleResult<()> {
    // build scraper
    let candle_scraper = CandleScraper::new(
        config.auth_token.clone(),
        config.symbol.clone(),
        config.session.clone(),
        config.timeframe.clone(),
        1,
    );

    // spawn
    let client = candle_scraper.client.clone();
    let executor_clone = executor.clone();
    let handle = executor.spawn(async move {
        client.subscribe(executor_clone).await
    });
    handle.detach();

    // build csv scraper
    let date = tradingview_common::utilities::get_current_date()?;
    let path = format!("{0}/{date}-{1}-{2}-{3}-candle.csv", config.output_dir, candle_scraper.symbol, candle_scraper.session, candle_scraper.timeframe);
    let candle_csv_scraper = CsvScraper::new(&path, candle_scraper).await?;
    let executor_clone = executor.clone();
    candle_csv_scraper.scrape(executor_clone, Duration::from_secs(5)).await
}

async fn spawn_indicator_csv_scraper(executor: Arc<Executor<'static>>, config: Config) -> SimpleResult<()> {
    // build scraper
    let indicator = TradingViewIndicators::generate_vwap_mvwap_ema_crossover(
        1,
        "close".to_string(),
        7,
        "close".to_string(),
        25,
        65,
        51,
        21     
    );
    let indicator_scraper = IndicatorScraper::new(
        config.auth_token.clone(),
        config.symbol.clone(),
        config.session.clone(),
        config.timeframe.clone(),
        1,
        indicator
    );

    // spawn
    let client = indicator_scraper.client.clone();
    let executor_clone = executor.clone();
    let handle = executor.spawn(async move {
        client.subscribe(executor_clone).await
    });
    handle.detach();

    // build csv scraper
    let date = tradingview_common::utilities::get_current_date()?;
    let path = format!("{0}/{date}-{1}-{2}-{3}-indicator.csv", config.output_dir, indicator_scraper.symbol, indicator_scraper.session, indicator_scraper.timeframe);
    let indicator_csv_scraper = CsvScraper::new(&path, indicator_scraper).await?;
    let executor_clone = executor.clone();    
    indicator_csv_scraper.scrape(executor_clone, Duration::from_secs(5)).await
}

async fn async_main(executor: &Arc<Executor<'static>>) -> SimpleResult<()> {
    // logging
    let logging_env = env_logger::Env::default().default_filter_or("debug,websocket_client=info,rustls=info,http_client=info,tradingview_client=info,tradingview_common=info");
    env_logger::Builder::from_env(logging_env).init();

    // load config
    let config = Config::from_env()?;

    // spawn handles
    let quote_handle = executor.spawn(spawn_quote_csv_scraper(executor.clone(), config.clone()));
    let candle_handle = executor.spawn(spawn_candle_csv_scraper(executor.clone(), config.clone()));
    let indicator_handle = executor.spawn(spawn_indicator_csv_scraper(executor.clone(), config.clone()));

    // wait for handles
    quote_handle.await?;
    candle_handle.await?;
    indicator_handle.await?;

    Ok(())
}

smol_main!(async_main);
