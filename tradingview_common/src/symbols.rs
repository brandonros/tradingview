pub struct TradingViewSymbols;

impl TradingViewSymbols {
    pub fn build_symbol(adjustment: &str, currency_id: Option<&str>, session: &str, symbol: &str) -> String {
        // crypto does not use currency_id?
        match currency_id {
            Some(currency_id) => format!(r#"={{\"adjustment\":\"{adjustment}\",\"currency-id\":\"{currency_id}\",\"session\":\"{session}\",\"symbol\":\"{symbol}\"}}"#),
            None => format!(r#"={{\"adjustment\":\"{adjustment}\",\"session\":\"{session}\",\"symbol\":\"{symbol}\"}}"#),
        }
    }
}
