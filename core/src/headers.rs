//! For Production Standard We need `Secrets` to be just built at runtime when needed
//! and 'NOT' be stored in any `CONST` or `static` var as it would have the same lifetime as the app
//! creating some security issues and having the credential leak, or appear in traces...etc...
use std::env;
use serde_json::json;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use crate::{
  errors::AppError,
  envs_manage,
};


/// we create a custom type error for only headers so that we know where it is in more fine grained way
type HeadersResult<T> = std::result::Result<T, AppError>;

/// funciton that would build the headers when needed 
pub fn get_auth_headers() -> HeadersResult<HeaderMap> {
  let api_key = match envs_manage::get_env("CEREBRAS_API_KEY") {
    Ok(value) => {
      println!("Api secret found but keeping it secret...! hahaha");
      value
    },
    Err(e) => {
      println!(
        "{}",
        AppError::EnvSecret(format!(
          "An error occurred while trying to access env var `CEREBRAS_API_KEY`: {}",
           e
        ))
      );
      return Err(AppError::EnvSecret("Missing API key".to_string()));
    },
  };

  let mut headers = HeaderMap::new();
  headers.insert(
    AUTHORIZATION,
    HeaderValue::from_str(&format!("Bearer {}", api_key))
      .map_err(|e| AppError::EnvSecret(format!("Invalid API key format: {}", e)))?,
  );
  headers.insert(
    CONTENT_TYPE,
    HeaderValue::from_static("application/json"),
  );

  Ok(headers)
}
