use simple_error::SimpleResult;

pub fn now() -> SimpleResult<u64> {
    let start = std::time::SystemTime::now();
    let since = start.duration_since(std::time::UNIX_EPOCH)?;
    Ok(since.as_secs())
}
