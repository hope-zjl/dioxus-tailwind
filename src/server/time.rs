use dioxus::prelude::*;

#[server]
pub async fn get_time() -> Result<String, ServerFnError> {
    let now = std::time::SystemTime::now();
    let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    let seconds = duration.as_secs();
    Ok(seconds.to_string())
}