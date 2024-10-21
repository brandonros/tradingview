use std::sync::Arc;
use std::time::Duration;

use simple_error::SimpleResult;
use async_executor::{with_thread_pool, Executor};
use tradingview_common::{TradingViewClientConfig, TradingViewClientMode, TradingViewIndicators, TradingViewSymbols};
use tradingview_client::{DefaultTradingViewMessageProcessor, TradingViewClient, TradingViewMessageProcessor};

async fn async_main(executor: &Arc<Executor<'static>>) -> SimpleResult<()> {
    // init logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug,websocket_client=info,rustls=info,http_client=info")).init();

    // init env vars
    dotenvy::from_filename("./.env").expect("failed to load env vars");
    let auth_token = std::env::var("AUTH_TOKEN").expect("failed to get AUTH_TOKEN");

    // build message processor
    let message_processor1: Arc<Box<dyn TradingViewMessageProcessor + Send + Sync>> = Arc::new(Box::new(DefaultTradingViewMessageProcessor {}));
    let message_processor2: Arc<Box<dyn TradingViewMessageProcessor + Send + Sync>> = Arc::new(Box::new(DefaultTradingViewMessageProcessor {}));
        
    // build clients
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
    let clients = vec![
        TradingViewClient::new(TradingViewClientConfig {
            name: "SPY5REG".to_string(),
            auth_token: auth_token.clone(),
            chart_symbols: vec![TradingViewSymbols::build_symbol("splits", Some("USD"), "regular", "AMEX:SPY")],
            quote_symbols: vec![TradingViewSymbols::build_symbol("splits", Some("USD"), "regular", "AMEX:SPY")],
            indicators: vec![
              vwap_mvwap_ema_crossover.clone()
            ],
            timeframe: Some("5".to_string()),
            range: Some(300),
            mode: TradingViewClientMode::Streaming
        }, message_processor1),

        TradingViewClient::new(TradingViewClientConfig {
            name: "SPY5EXT".to_string(),
            auth_token: auth_token.clone(),
            chart_symbols: vec![TradingViewSymbols::build_symbol("splits", Some("USD"), "extended", "AMEX:SPY")],
            quote_symbols: vec![TradingViewSymbols::build_symbol("splits", Some("USD"), "extended", "AMEX:SPY")],
            indicators: vec![
              vwap_mvwap_ema_crossover.clone()
            ],
            timeframe: Some("5".to_string()),
            range: Some(300),
            mode: TradingViewClientMode::Streaming
        }, message_processor2),
    ];

    // spawn clients on threads
    let mut handles = vec![];
    for client in clients {
        let executor_clone = executor.clone();
        let handle = executor.spawn(async move {
            match client.run(executor_clone).await {
                Ok(_) => (),
                Err(err) => panic!("{err}"),
            }
        });
        handles.push(handle);
    }

    // watch handles
    loop {
        for handle in &handles {
            if handle.is_finished() {
                panic!("a handle finished");
            }
            std::thread::sleep(Duration::from_millis(1000));
        }
    }
}

fn main() -> SimpleResult<()> {
    let ex = Arc::new(Executor::new());
    with_thread_pool(&ex, || async_io::block_on(async_main(&ex)))
}
