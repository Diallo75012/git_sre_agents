// use serde_json::Value;
// use tokio::sync::mpsc;
// use crate::agents::*;
// use crate::errors::*;
// 
// 
// /// Dispatcher receives messages and routes them to correct async functions
// async fn dispatcher(mut rx: mpsc::Receiver<RoutedMessage>) -> Result<String, AppError> {
//   while let Some(rm) = rx.recv().await {
//     match rm.next_node.as_str() {
//       "sre1_agent" => {
//         // we send the message to the agent and there we will receive it and start working. Will receive something like `'{instruction:....}'`
//         //sre1_agent(rm.message).await;
//         println!("Going to sre1_agent node step");
//         sre1_agent_node_work_orchestration(rm.message).await;
//       }
//       "sre2_agent" => {
//         println!("Going to sre2_agent node step");
//         //sre2_agent_node_work_orchestration(rm.message).await;
//       }
//       "pr_agent" => {
//         //some_other_agent(rm.message).await;
//         println!("Going to sone other agent node step")
//       }
//       _ => {
//         return Err(AppError::UnknownNode(rm.next_node));
//       }
//     }
//   }
//   Ok("success".to_string())
// }
// 
// /// Entrypoint that starts dispatcher and sends the first RoutedMessage
// pub async fn transmitter(next_step: &str, msg: &Value) -> Result<String, AppError> {
//   let (tx, rx) = mpsc::channel::<RoutedMessage>(32);
//   let dispatcher_handle = tokio::spawn(dispatcher(rx));
// 
//   let initial_message = RoutedMessage {
//     next_node: next_step.to_string(),
//     message: msg.clone(),
//   };
// 
//   // Send safely
//   match tx.send(initial_message).await {
//     Ok(_) => { /* Message sent */ }
//     Err(e) => {
//       return Err(AppError::ChannelSendError(format!(
//         "Failed to send message: {}",
//          e
//       )))
//     }
//   }
// 
//   // Wait safely
//   match dispatcher_handle.await {
//     Ok(result) => result, // returns Ok or Err from `dispatcher`
//     Err(e) => Err(AppError::JoinError(format!("Dispatcher failed: {}", e))),
//   }
// }


use serde_json::Value;
use tokio::sync::mpsc;
use std::collections::HashMap;
use crate::agents::*;
use crate::errors::*;
use async_trait::async_trait;


/// Trait to be implemented by all node agents.
#[async_trait]
pub trait NodeHandler: Send + Sync {
  async fn handle(&self, message: Value, tx: mpsc::Sender<RoutedMessage>) -> Result<(), AppError>;
}

/// Dispatcher continuously receives messages and routes them via registered handlers.
pub async fn start_dispatcher(
  mut rx: mpsc::Receiver<RoutedMessage>,
  routes: HashMap<String, Box<dyn NodeHandler>>,
  tx: mpsc::Sender<RoutedMessage>,
) -> Result<String, AppError> {
  // this is a loop which will evaluate whatever comes to it to route to the right node
  //so here we `recv()` messages. keep in mind "multiple producer single consumer" `mpsc`
  // our `rx` is always the same receiver but each node would sent its `tx` transmitter evaluated in this loop
  while let Some(RoutedMessage { next_node, message }) = rx.recv().await {
    match routes.get(&next_node) {
      Some(handler) => {
        // Clone tx so handler can send next message if needed
        let tx_clone = tx.clone();
        // each route has a `handle()` trait attached
        // trait is a signature function that can be shared but whatever is inside is specific
        // to each nodes, this is how we can play with it
        handler.handle(message, tx_clone).await?;
      }
      None => {
        return Err(AppError::UnknownNode(next_node));
      }
    }
  }
  // when the queue will be emptied out it would just stop the `thread` channel and the app would be able
  // to gracefully finish
  Ok("Dispatcher finished".to_string())
}

/// Initializes the dispatcher and returns a sender to be used by the initial node
/// Entrypoint that starts dispatcher and sends the first RoutedMessage
pub fn transmitter(
  routes: HashMap<String, Box<dyn NodeHandler>>,
) -> (mpsc::Sender<RoutedMessage>, tokio::task::JoinHandle<Result<String, AppError>>) {
  let (tx, rx) = mpsc::channel::<RoutedMessage>(32);
  let handle = tokio::spawn(start_dispatcher(rx, routes, tx.clone()));
  (tx, handle)
}
