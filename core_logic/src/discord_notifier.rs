#![allow(unused_doc_comments)]
use reqwest::{
  //header::CONTENT_TYPE,
  Client,
  StatusCode,
};
use serde_json::json;
use crate::errors::AppError;
use crate::write_debug_log::*;

pub async fn notify_human(message: &str, url: &str) -> Result<String, AppError> {
  /// Create JSON payload: this assumes Cerebras expects a field like `content`
  let payload = json!({
    "content": message
  });

  let client = Client::new();
  /// this is a Future : `Future<Output = Result<Response, Error>>`
  let notification = client
    .post(url)
    // use `json(&struct)` only on something implementing `serialize`, otherwise just use `.body()` and `header()`
    .json(&payload)
    //.header(CONTENT_TYPE, "application/json")
    //.body(&payload)
    .send()
    .await
    .map_err(|e| AppError::Notify(e.to_string()))?;

    if notification.status() != StatusCode::OK {
      return Err(AppError::Notify(notification.status().to_string()));
    }

    let body = notification
      .text()
      .await
      .map_err(|e| AppError::Notify(e.to_string()))?;
    /// if the message reaches the discord category, discord doesn't return any message but just a 204 status_code

    // we log what is sent to Discord
    write_step_cmd_debug("\nDISCORD NOTIFICATION SENT ->\n");
    write_step_cmd_debug(&body);

    Ok(body)

}
