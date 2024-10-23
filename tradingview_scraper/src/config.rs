use simple_error::SimpleResult;

#[derive(Debug, Clone)]
pub struct Config {
    pub auth_token: String,
    pub output_dir: String,
    pub symbol: String,
    pub session: String,
    pub timeframe: String,
}

impl Config {
    pub fn from_env() -> SimpleResult<Self> {
        dotenvy::from_filename("./.env")?;
        Ok(Self {
            auth_token: std::env::var("AUTH_TOKEN")?,
            output_dir: std::env::var("OUTPUT_DIR")?,
            symbol: std::env::var("SYMBOL")?,
            session: std::env::var("SESSION")?,
            timeframe: std::env::var("TIMEFRAME")?,
        })
    }
}
