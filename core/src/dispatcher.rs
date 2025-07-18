use serde_json::Value;
use tokio::sync::mpsc;
use crate::agents::*;
use crate::errors::*;


/// Dispatcher receives messages and routes them to correct async functions
async fn dispatcher(mut rx: mpsc::Receiver<RoutedMessage>) -> Result<String, AppError> {
  while let Some(rm) = rx.recv().await {
    match rm.next_node.as_str() {
      "RequestAnalyzer" => {
        //request_analyzer(rm.message).await;
        println!("Going to RequestAnalyzer node step")
      }
      "some_other_agent" => {
        //some_other_agent(rm.message).await;
        println!("Going to sone other agent node step")
      }
      _ => {
        return Err(AppError::UnknownNode(rm.next_node));
      }
    }
  }
  Ok("success".to_string())
}

/// Entrypoint that starts dispatcher and sends the first RoutedMessage
pub async fn transmitter(next_step: &str, msg: &Value) -> Result<String, AppError> {
  let (tx, rx) = mpsc::channel::<RoutedMessage>(32);
  let dispatcher_handle = tokio::spawn(dispatcher(rx));

  let initial_message = RoutedMessage {
    next_node: next_step.to_string(),
    message: msg.clone(),
  };

  // Send safely
  match tx.send(initial_message).await {
    Ok(_) => { /* Message sent */ }
    Err(e) => {
      return Err(AppError::ChannelSendError(format!(
        "Failed to send message: {}",
         e
      )))
    }
  }

  // Wait safely
  match dispatcher_handle.await {
    Ok(result) => result, // returns Ok or Err from `dispatcher`
    Err(e) => Err(AppError::JoinError(format!("Dispatcher failed: {}", e))),
  }
}
