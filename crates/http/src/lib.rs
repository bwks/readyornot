/// Check if an HTTP service is running at the given URL.
///
/// Returns `Ok(true)` if the server responds with 200 OK,
/// `Ok(false)` for other status codes, and propagates errors
/// for connection failures, timeouts, etc.
pub async fn check_http(url: &str) -> Result<bool, reqwest::Error> {
    let response = reqwest::get(url).await?;
    Ok(response.status() == reqwest::StatusCode::OK)
}
