//! This crate is for the env vars management
//! we use `dotenvy` and load `.env` vars with `dotenv().ok()`
//! after all envrionment variables are saved in memory for the lifetime of the app
//! We use `std::env` to set environment variables which are saved in memory for the lifetime of the app
//! It is different from `Python` as here `.env` file is not altered,
//! in `Rust` the `.env` is not overriden, it is all happenning in memory 
#![allow(unused_doc_comments)]
use dotenvy::{
  // from_path,
  // from_path_iter,
  dotenv,
};
use std::{
  env,
  // path::Path,
};
use anyhow::Result;
use crate::errors::AppError;


// function that get env vars
pub fn get_env(key: &str) -> Result<String, AppError> {
  /// relative path to root .env ../ src , ../ core, ../ root project so it is too long
  /// from_path(Path::new("../../.env")).ok();
  /// use this shorter and will load all .env env vars: here uses `dotenvy`
  dotenv().ok();
  /// here uses Rust `std` library `env` to get envs as those are loaded previously by `dotenvy`
  /// ```no_run
  /// match env::var(key) {
  ///   Ok(value) => Ok(value.to_string()),
  ///   Err(e) => Err(AppError::Env(format!("{}", e))),
  /// }
  /// ```
  /// also can use instead of `std` library `dotenvy` `var` to get the value:
  match dotenvy::var(key) {
    Ok(value) => Ok(value.to_string()),
    Err(e) => Err(AppError::Env(format!("{}", e))),
  }
}

/// function that sets env vars
pub fn create_or_update_env(key: &str, value: &str) -> Result<String, AppError> {
  /// load .env if present
  if let Err(e) = dotenv() {
    eprintln!("Warning: Failed to load .env: {}", e);
    // Not critical, so we continue
  }

  /// This never fails, so just use it directly
  /// and no need to load dotenv, it is different from python as in Rust those are saved in memory,
  /// no .env file changes, just loaded to the environment
  /// rust as changed those function and those need to be called in unsafe blocks ` std::env::remove_var` and ` std::env::set_var`
  unsafe {
    env::set_var(key, value);
  }
  /// we use the other helper function `get_env()` to check if it has been saved properly
  match get_env(key) {
  	Ok(present) => Ok(format!("Environement variable successfully set. key: {}, value:{}", key, present)),
  	Err(e) => Err(AppError::Env(format!("{:?}", e))),
  }
}
