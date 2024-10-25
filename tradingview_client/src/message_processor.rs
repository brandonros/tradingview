use async_trait::async_trait;
use simple_error::SimpleResult;
use tradingview_common::ParsedTradingViewMessage;

#[async_trait]
pub trait TradingViewMessageProcessor {
    async fn process_message(&mut self, name: String, message: ParsedTradingViewMessage) -> SimpleResult<()>;
}
