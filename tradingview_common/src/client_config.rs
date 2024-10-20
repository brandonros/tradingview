use miniserde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum TradingViewClientMode {
    Standard,
    Streaming
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TradingViewClientConfig {
    pub name: String,
    pub auth_token: String,
    pub chart_symbols: Vec<String>,
    pub quote_symbols: Vec<String>,
    pub indicators: Vec<String>,
    pub timeframe: Option<String>, // not needed for quotes
    pub range: Option<usize>, // not needed for quotes
    pub mode: TradingViewClientMode
}
