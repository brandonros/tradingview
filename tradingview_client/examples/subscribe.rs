use std::{sync::Arc, time::Duration};

use async_lock::RwLock;
use simple_error::SimpleResult;
use async_executor::{with_thread_pool, Executor};
use tradingview_common::{TradingViewClientConfig, TradingViewIndicators};
use tradingview_client::{StatefulMessageProcessor, TradingViewClient, TradingViewMessageProcessor};

async fn async_main(executor: &Arc<Executor<'static>>) -> SimpleResult<()> {
    // init logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug,websocket_client=info,rustls=info,http_client=info,tradingview_client=info,tradingview_common=info")).init();

    // init env vars
    dotenvy::from_filename("./.env").expect("failed to load env vars");
    let auth_token = std::env::var("AUTH_TOKEN").expect("failed to get AUTH_TOKEN");

    // build message processor
    let stateful_message_processor = Arc::new(RwLock::new(StatefulMessageProcessor::default()));

    // get symbol from args
    let args = std::env::args().collect::<Vec<_>>();
    let symbol = &args[1];

    // build indicator
    let vwap_mvwap_ema_crossover = TradingViewIndicators::generate_vwap_mvwap_ema_crossover(
        1,
        "close".to_string(),
        7,
        "close".to_string(),
        25,
        65,
        51,
        21
    );

    // build config
    let config = TradingViewClientConfig {
        name: symbol.to_string(),
        auth_token: auth_token.clone(),
        chart_symbols: vec![
            symbol.to_string()
        ],
        quote_symbols: vec![
            symbol.to_string()
        ],
        indicators: vec![
            vwap_mvwap_ema_crossover.to_string()
        ],
        timeframe: Some("5".to_string()),
        range: Some(1),
    };

    // build client
    let message_processor_trait_object = stateful_message_processor.clone() as Arc<RwLock<dyn TradingViewMessageProcessor + Send + Sync>>;
    let client = TradingViewClient::new(config, message_processor_trait_object);

    // spawn handler that checks state
    let local_stateful_message_processor = stateful_message_processor.clone();
    let state_handle = executor.spawn(async move {
        loop {
            // sleep
            async_io::Timer::after(Duration::from_secs(5)).await;

            // lock messge processor
            let stateful_message_processor = local_stateful_message_processor.read().await;
            log::info!("stateful_message_process = {stateful_message_processor:?}");
        }
    });
    state_handle.detach();

    // subscribe
    client.subscribe(executor.clone()).await
}

fn main() -> SimpleResult<()> {
    let ex = Arc::new(Executor::new());
    with_thread_pool(&ex, || async_io::block_on(async_main(&ex)))
}
