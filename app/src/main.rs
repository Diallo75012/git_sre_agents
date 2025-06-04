use anyhow::Result;
// use tokio::time::{
//   sleep,
//   Duration,
// };
use core::{
  envs_manage,
  errors::AppError,
};


// function wrapper of the `Engine`
async fn run() {
  match envs_manage::get_env("city") {
  	Ok(value) => {
  	  println!("{}", value)
  	},
  	Err(e) => {
  	  println!("{}", AppError::Env(format!("An error occurred while trying to access env var: {}", e)));
  	}, 
  }

  match envs_manage::create_or_update_env("location", "Shibuya") {
  	Ok(value) => {
  	  println!("{:?}", value)
  	},
  	Err(e) => {
  	  AppError::Env(format!("An error occurred while trying to access env var: {}", e));
  	}
  }
}


#[tokio::main]
async fn main() {
  run().await
}
