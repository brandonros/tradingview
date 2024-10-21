use std::{sync::Arc, time::Duration};

use async_lock::RwLock;
use simple_error::{box_err, SimpleResult};
use tradingview_codec::TradingViewMessageWrapper;
use tradingview_common::ParsedTradingViewMessage;

use crate::utilities;

pub async fn wait_for_message_with_timeout<F>(
    duration: Duration,
    buffer: Arc<RwLock<Vec<TradingViewMessageWrapper>>>,
    predicate: F,
) -> SimpleResult<TradingViewMessageWrapper>
where
    F: Fn(&TradingViewMessageWrapper) -> bool + Send + 'static,
{
    utilities::run_with_timeout(
        duration,
        Box::pin(utilities::wait_for_message(buffer, predicate)),
    )
    .await
    .ok_or_else(|| {
        let bt = backtrace::Backtrace::new();
        box_err!(format!("timed out\n{bt:?}"))
    })?
    .ok_or_else(|| box_err!("failed to get expected message"))
}

pub async fn wait_for_typed_message_with_timeout<T, F>(
    timeout: Duration,
    buffer: Arc<RwLock<Vec<TradingViewMessageWrapper>>>,
    predicate: F,
) -> SimpleResult<T>
where
    F: Fn(&TradingViewMessageWrapper) -> bool + Send + 'static,
    T: TryFrom<ParsedTradingViewMessage>,
{
    let message = wait_for_message_with_timeout(timeout, buffer, predicate).await?;
    message
        .parsed_message
        .try_into()
        .map_err(|_| box_err!("failed to cast message to expected type"))
}
