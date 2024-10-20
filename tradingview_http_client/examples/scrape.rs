use std::sync::Arc;

use async_executor::Executor;
use simple_error::SimpleResult;
use tradingview_common::{TradingViewIndicators, TradingViewSymbols};
use tradingview_http_client::TradingViewHttpClient;

#[macro_rules_attribute::apply(smol_macros::main!)]
async fn main(_executor: Arc<Executor<'static>>) -> SimpleResult<()> {
    // logging
    let logging_env = env_logger::Env::default().default_filter_or("debug,websocket_client=info,rustls=info,http_client=info");
    env_logger::Builder::from_env(logging_env).init();

    // init env vars
    dotenvy::from_filename("./.env")?;
    let auth_token = std::env::var("AUTH_TOKEN")?;

    // scrape
    let symbol = TradingViewSymbols::build_symbol("splits", None, "regular", "BINANCE:BTCUSDT");
    let timeframe = "5";
    let range = 1;
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
    TradingViewHttpClient::scrape_quote(&auth_token, &symbol).await?;
    TradingViewHttpClient::scrape_candles(&auth_token, &symbol, &timeframe, range).await?;
    TradingViewHttpClient::scrape_indicator(&auth_token, &symbol, &timeframe, range, &indicator).await?;

    Ok(())
}