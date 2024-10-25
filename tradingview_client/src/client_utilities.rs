use std::{sync::Arc, time::Duration};

use async_lock::RwLock;
use simple_error::{box_err, SimpleResult};
use tradingview_codec::TradingViewMessageWrapper;
use tradingview_common::ParsedTradingViewMessage;

use crate::message_utilities;

pub async fn wait_for_typed_message_with_timeout<T, F>(
    timeout: Duration,
    buffer: Arc<RwLock<Vec<TradingViewMessageWrapper>>>,
    predicate: F,
) -> SimpleResult<T>
where
    F: Fn(&TradingViewMessageWrapper) -> bool + Send + 'static,
    T: TryFrom<ParsedTradingViewMessage>,
{
    let message = message_utilities::wait_for_message_with_timeout(timeout, buffer, predicate).await?;
    message
        .parsed_message
        .try_into()
        .map_err(|_| box_err!("failed to cast message to expected type"))
}
