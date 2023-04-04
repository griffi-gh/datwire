use axum::{extract::{State, Json}, response::IntoResponse};
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
use hyper::StatusCode;
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use anyhow::Result;
use thiserror::Error;
use serde_json::{json, Value};
use crate::AppState;

#[derive(Deserialize)]
pub struct Register {
  pub email: String,
  pub handle: String,
  pub password: String,
}

// #[derive(Error, Debug, Serialize)]
// pub enum ValidationError {
//   #[error("email is already registered")]
//   EmailAlreadyTaken,
//   #[error("invalid email address")]
//   InvalidEmail,
//   #[error("invalid username")]
//   InvalidUsername,
//   #[error("insecure password")]
//   InsecurePassword,
//   #[error("password is too long")]
//   PasswordTooLong,
// }

pub async fn register(
  State(state): State<Arc<AppState>>,
  Json(data): Json<Register>,
) -> (StatusCode, Json<Value>) {
  let password_hash = {
    let salt = SaltString::generate(&mut OsRng);
    let Ok(password_hash) = Argon2::default().hash_password(data.password.as_bytes(), &salt) else {
      return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
        "code": 500,
        "success": false,
      })))
    };
    password_hash.to_string()
  };
  
  (StatusCode::CREATED, Json(json!({
    "code": 201,
    "success": true,
  })))
}
