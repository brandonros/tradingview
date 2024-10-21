use std::sync::Arc;

use simple_error::SimpleResult;
use async_executor::{with_thread_pool, Executor};
use tradingview_common::{TradingViewClientConfig, TradingViewClientMode};
use tradingview_client::{DefaultTradingViewMessageProcessor, TradingViewClient, TradingViewMessageProcessor};

async fn async_main(executor: &Arc<Executor<'static>>) -> SimpleResult<()> {
    // init logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug,websocket_client=info,rustls=info,http_client=info")).init();

    // init env vars
    dotenvy::from_filename("./.env").expect("failed to load env vars");
    let auth_token = std::env::var("AUTH_TOKEN").expect("failed to get AUTH_TOKEN");

    // build message processor
    let message_processor: Arc<Box<dyn TradingViewMessageProcessor + Send + Sync>> = Arc::new(Box::new(DefaultTradingViewMessageProcessor {}));

    // get symbol
    let args = std::env::args().collect::<Vec<_>>();
    let symbol = &args[1];
    let config = TradingViewClientConfig {
        name: symbol.to_string(),
        auth_token: auth_token.clone(),
        chart_symbols: vec![],
        quote_symbols: vec![symbol.to_string()],
        indicators: vec![],
        timeframe: None,
        range: None,
        mode: TradingViewClientMode::Standard
    };
    let client = TradingViewClient::new(config, message_processor);

    // spawn client
    let scrape_result = match client.run(executor.clone()).await {
        Ok(scrape_result) => scrape_result,
        Err(err) => panic!("{err}"),
    };

    log::info!("scrape_result = {scrape_result:?}");

    Ok(())
}

fn main() -> SimpleResult<()> {
    let ex = Arc::new(Executor::new());
    with_thread_pool(&ex, || async_io::block_on(async_main(&ex)))
}

