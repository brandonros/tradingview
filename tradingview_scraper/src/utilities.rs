use simple_error::SimpleResult;
use time::{macros::format_description, OffsetDateTime};

pub fn now() -> SimpleResult<u64> {
    let start = std::time::SystemTime::now();
    let since = start.duration_since(std::time::UNIX_EPOCH)?;
    Ok(since.as_secs())
}

pub fn get_current_date() -> SimpleResult<String> {
    let now = OffsetDateTime::now_utc();
    let format = format_description!("[year]-[month]-[day]");
    let date_string = now.format(&format)?;
    Ok(date_string)
}
