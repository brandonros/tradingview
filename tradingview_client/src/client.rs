use std::sync::Arc;
use std::time::Duration;

use async_executor::Executor;
use async_lock::RwLock;
use http::{Request, Uri, Version};
use http_client::HttpClient;

use simple_error::{box_err, SimpleResult};
use tradingview_common::{ParsedTradingViewMessage, TradingViewClientConfig, TradingViewClientMode, TradingViewScrapeResult};
use websocket_client::{WebSocketHelpers, WebSocketReader, WebSocketWriter};
use futures_lite::io::{BufReader, BufWriter};

use crate::utilities;
use crate::reader::TradingViewReader;
use crate::writer::TradingViewWriter;
use crate::message_wrapper::TradingViewMessageWrapper;
use crate::message_processor::TradingViewMessageProcessor;

pub struct TradingViewClient {
    config: TradingViewClientConfig,
    message_processor: Arc<Box<dyn TradingViewMessageProcessor + Send + Sync>>
}

impl TradingViewClient {
    pub fn new(config: TradingViewClientConfig, message_processor: Arc<Box<dyn TradingViewMessageProcessor + Send + Sync>>) -> Self {
        Self {
            config,
            message_processor
        }
    }

    pub async fn wait_for_message_with_timeout<F>(
        duration: Duration,
        buffer: Arc<RwLock<Vec<TradingViewMessageWrapper>>>,
        predicate: F
    ) -> SimpleResult<TradingViewMessageWrapper>
    where
        F: Fn(&TradingViewMessageWrapper) -> bool + Send + 'static,
    {
        utilities::run_with_timeout(duration, Box::pin(utilities::wait_for_message(buffer, predicate)))
            .await
            .ok_or_else(|| {
                let bt = backtrace::Backtrace::new();
                box_err!(format!("timed out\n{bt:?}"))
            })?
            .ok_or_else(|| {
                box_err!("failed to get expected message")
            })
    }

    pub async fn run(&self, executor: Arc<Executor<'static>>) -> SimpleResult<TradingViewScrapeResult> {
        // Build the URI for the request
        let uri: Uri = "wss://data.tradingview.com/socket.io/websocket?type=chart".parse()?;

        // Build the GET request
        let request = Request::builder()
            .method("GET")
            .version(Version::HTTP_11)
            .uri(uri)
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36")
            .header("Host", "data.tradingview.com")
            .header("Origin", "https://www.tradingview.com")            
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")      
            .header("Sec-WebSocket-Version", "13")                        
            .header("Sec-WebSocket-Key", WebSocketHelpers::generate_sec_websocket_key())    
            .body(vec![])?;

        // Get the response
        let mut stream = HttpClient::create_connection(&request).await?;
        let response = HttpClient::request(&mut stream, &request).await?;
        log::info!("response = {response:?}");

        // split
        let (reader, writer) = futures_lite::io::split(stream);
        let reader = BufReader::new(reader);
        let writer = BufWriter::new(writer);

        // create websocket client
        let ws_reader = WebSocketReader::new(reader);
        let ws_writer = WebSocketWriter::new(writer);        

        // Create the TradingViewClient
        let mut tv_reader = TradingViewReader::new(ws_reader);
        let mut tv_writer = TradingViewWriter::new(ws_writer);

        // prepare buffer + references
        let buffer: Vec<TradingViewMessageWrapper> = Vec::new();
        let buffer = RwLock::new(buffer);
        let buffer_arc = Arc::new(buffer);
        let reader_handle_buffer_ref = buffer_arc.clone();

        // scrape result
        let mut scrape_result = TradingViewScrapeResult {
            server_hello_messages: vec![],
            symbol_resolved_messages: vec![],
            series_loading_messages: vec![],
            timescale_update_messages: vec![],
            series_completed_messages: vec![],
            study_loading_messages: vec![],
            study_completed_messages: vec![],
            quote_completed_messages: vec![],
            quote_last_price_messages: vec![],
            study_data_update_messages: vec![],
            series_data_update_messages: vec![],
        };

        // Spawn the reader task
        let _reader_handle = executor.spawn(async move {
            loop {
                match tv_reader.read_message().await {
                    Ok(result) => {
                        match result {
                            Some(message) => {
                                // add message to buffer
                                let mut write_lock = reader_handle_buffer_ref.write().await;
                                write_lock.push(message);
                                drop(write_lock);
                            },
                            None => {
                                log::warn!("received none");
                                break;
                            }
                        }
                    },
                    Err(err) => panic!("{err:?}"),
                }
            }
        });
        
        // Wait for server hello message with timeout
        let server_hello_message = TradingViewClient::wait_for_message_with_timeout(
            Duration::from_secs(1),
            buffer_arc.clone(),
            |message| message.payload.contains("javastudies")
        ).await?;
        let server_hello_message = server_hello_message.parsed_message.as_server_hello().ok_or(box_err!("failed to cast"))?;
        log::info!("server_hello_message = {server_hello_message:?}");
        scrape_result.server_hello_messages.push(server_hello_message.clone());

        // set auth token
        tv_writer.set_auth_token(&self.config.auth_token).await?;
        
        // set locale
        tv_writer.set_locale("en", "US").await?;

        // handle chart sessions
        let mut index = 1;
        for chart_symbol in &self.config.chart_symbols {
            // create chart session
            let chart_session_id = format!("cs_{index:012}");

            // create chart session
            tv_writer.chart_create_session(&chart_session_id).await?;

            // resolve symbol
            let symbol_id = "sds_sym_1";
            tv_writer.resolve_symbol(&chart_session_id, symbol_id, &chart_symbol).await?;

            // wait for symbol resolved message
            let symbol_resolved_message = TradingViewClient::wait_for_message_with_timeout(
                Duration::from_secs(2),
                buffer_arc.clone(),
                |message| message.payload.contains("symbol_resolved")
            ).await?;
            let symbol_resolved_message = symbol_resolved_message.parsed_message.as_symbol_resolved().ok_or(box_err!("failed to cast"))?;
            log::info!("symbol_resolved_message = {symbol_resolved_message:?}");
            scrape_result.symbol_resolved_messages.push(symbol_resolved_message.clone());

            // add symbol to chart session as series
            let series_id = "sds_1";
            tv_writer.create_series(&chart_session_id, series_id, "s1",  symbol_id, &self.config.timeframe.as_ref().unwrap(), self.config.range.unwrap()).await?;

            // switch chart timezone
            tv_writer.switch_timezone(&chart_session_id, "exchange").await?;

            // wait for series loading message
            let series_loading_message = TradingViewClient::wait_for_message_with_timeout(
                Duration::from_secs(2),
                buffer_arc.clone(),
                |message| message.payload.contains("series_loading")
            ).await?;
            log::info!("series_loading_message = {series_loading_message:?}");
            let series_loading_message = series_loading_message.parsed_message.as_series_loading().ok_or(box_err!("failed to cast"))?;
            scrape_result.series_loading_messages.push(series_loading_message.clone());

            // wait for timescale update message
            let timescale_update_message = TradingViewClient::wait_for_message_with_timeout(
                Duration::from_secs(2),
                buffer_arc.clone(),
                |message| message.payload.contains("timescale_update")
            ).await?;
            log::info!("timescale_update_message = {timescale_update_message:?}");
            let timescale_update_message = timescale_update_message.parsed_message.as_timescale_update().ok_or(box_err!("failed to cast"))?;
            scrape_result.timescale_update_messages.push(timescale_update_message.clone());

            // wait for series completed message
            let series_completed_message = TradingViewClient::wait_for_message_with_timeout(
                Duration::from_secs(2),
                buffer_arc.clone(),
                |message| message.payload.contains("series_completed")
            ).await?;
            log::info!("series_completed_message = {series_completed_message:?}");
            let series_completed_message = series_completed_message.parsed_message.as_series_completed().ok_or(box_err!("failed to cast"))?;
            scrape_result.series_completed_messages.push(series_completed_message.clone());

            // optionally create study session
            if self.config.indicators.len() > 0 {
                let study_session_id = "st1";
                tv_writer.create_study(&chart_session_id, study_session_id, "sessions_1", series_id, "Sessions@tv-basicstudies-241", "{}").await?;

                // wait for study loading message
                let study_loading_message = TradingViewClient::wait_for_message_with_timeout(
                    Duration::from_secs(10), // not ideal
                    buffer_arc.clone(),
                    |message| message.payload.contains("study_lodaing")
                ).await?;
                let study_loading_message = study_loading_message.parsed_message.as_study_loading().ok_or(box_err!("failed to cast"))?;
                log::info!("study_loading_message = {study_loading_message:?}");
                scrape_result.study_loading_messages.push(study_loading_message.clone());

                // wait for study completed message
                let study_completed_message = TradingViewClient::wait_for_message_with_timeout(
                    Duration::from_secs(3),
                    buffer_arc.clone(),
                    |message| message.payload.contains("study_completed")
                ).await?;
                let study_completed_message = study_completed_message.parsed_message.as_study_completed().ok_or(box_err!("failed to cast"))?;
                log::info!("study_completed_message = {study_completed_message:?}");
                scrape_result.study_completed_messages.push(study_completed_message.clone());

                let mut index = 2;
                for indciator in &self.config.indicators {
                    let study_value = indciator;
                    let study_id = format!("st{index}");
                    tv_writer.create_study(&chart_session_id, &study_id, study_session_id, series_id, "Script@tv-scripting-101!", study_value).await?;
                    index += 1;

                    // wait for study loading message
                    let study_loading_message = TradingViewClient::wait_for_message_with_timeout(
                        Duration::from_secs(3),
                        buffer_arc.clone(),
                        |message| message.payload.contains("study_loading")
                    ).await?;
                    let study_loading_message = study_loading_message.parsed_message.as_study_loading().ok_or(box_err!("failed to cast"))?;
                    log::info!("study_loading_message = {study_loading_message:?}");
                    scrape_result.study_loading_messages.push(study_loading_message.clone());

                    // wait for study completed message
                    let study_completed_message = TradingViewClient::wait_for_message_with_timeout(
                        Duration::from_secs(3),
                        buffer_arc.clone(),
                        |message| message.payload.contains("study_completed")
                    ).await?;
                    let study_completed_message = study_completed_message.parsed_message.as_study_completed().ok_or(box_err!("failed to cast"))?;
                    log::info!("study_completed_message = {study_completed_message:?}");
                    scrape_result.study_completed_messages.push(study_completed_message.clone());

                    // wait for study data update
                    let study_data_update_message = TradingViewClient::wait_for_message_with_timeout(
                        Duration::from_secs(3),
                        buffer_arc.clone(),
                        |message| {
                            match &message.parsed_message {
                                ParsedTradingViewMessage::DataUpdate(data_update_message) => {
                                    match &data_update_message.study_updates {
                                        Some(study_updates) => {
                                            return study_updates.len() > 0
                                        },
                                        None => return false
                                    }
                                },
                                _ => false
                            }
                        }
                    ).await?;
                    let study_data_update_message = study_data_update_message.parsed_message.as_data_update().ok_or(box_err!("failed to cast"))?;
                    log::info!("study_data_update_message = {study_data_update_message:?}");
                    scrape_result.study_data_update_messages.push(study_data_update_message.clone());
                }
            }

            // increment index
            index += 1;
        }

        // quote_symbol quote session
        let mut index = 1;
        for quote_symbol in &self.config.quote_symbols {
            // create quote session
            let quote_session_id = format!("qs_{index:012}");
            tv_writer.quote_create_session(&quote_session_id).await?;

            // set quote session fields
            tv_writer.quote_set_fields(&quote_session_id).await?;

            // add symbol to quote session
            tv_writer.quote_add_symbols(&quote_session_id, &quote_symbol).await?;

            // turn on quote fast symbols for quote session
            tv_writer.quote_fast_symbols(&quote_session_id, &quote_symbol).await?;

            // wait for quote completed message
            let quote_completed_message = TradingViewClient::wait_for_message_with_timeout(
                Duration::from_secs(2),
                buffer_arc.clone(),
                |message| message.payload.contains("quote_completed")
            ).await?;
            let quote_completed_message = quote_completed_message.parsed_message.as_quote_completed().ok_or(box_err!("failed to cast"))?;
            log::info!("quote_completed_message = {quote_completed_message:?}");
            scrape_result.quote_completed_messages.push(quote_completed_message.clone());

            // wait for quote last price
            let quote_last_price_message = TradingViewClient::wait_for_message_with_timeout(
                Duration::from_secs(2),
                buffer_arc.clone(),
                |message| {
                    match &message.parsed_message {
                        ParsedTradingViewMessage::QuoteSeriesData(quote_series_data_message) => {
                            quote_series_data_message.quote_update.rtc.is_some() || quote_series_data_message.quote_update.lp.is_some()
                        },
                        _ => false
                    }
                }
            ).await?;
            let quote_last_price_message = quote_last_price_message.parsed_message.as_quote_series_data().ok_or(box_err!("failed to cast"))?;
            log::info!("quote_last_price_message = {quote_last_price_message:?}");
            scrape_result.quote_last_price_messages.push(quote_last_price_message.clone());

            // increment index
            index += 1;
        }

        // request more data from series?
        /*for _ in 0..20 {
            tv_writer.request_more_data(chart_session_id1, series_id, 1000).await?;

            // TODO: wait for individual sries_loading / study_loading / study_completed messages

            async_io::Timer::after(Duration::from_secs(1)).await;
        }*/

        // exit if simple
        match self.config.mode {
            TradingViewClientMode::Standard => {
                // TODO: make sure buffer is empty (no missed message extraction)

                // close socket?
                tv_writer.close().await?;

                // return
                return Ok(scrape_result);
            },
            _ => ()
        }

        // read all messages
        loop {
            let result = utilities::wait_for_message(buffer_arc.clone(), |_| true).await;
            match result {
                Some(message) => {
                    let parsed_message = ParsedTradingViewMessage::from_string(&message.payload)?;
                    match &parsed_message {
                        ParsedTradingViewMessage::Ping(nonce) => {
                            log::info!("ping nonce = {nonce}");
                            tv_writer.pong(*nonce).await?;
                        },
                        _ => {
                            // send to message processor
                            self.message_processor.process_message(self.config.name.clone(), parsed_message).await;
                        }
                    }
                },
                None => panic!("closed")
            }
        }
    }
}
