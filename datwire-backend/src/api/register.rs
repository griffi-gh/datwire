use axum::extract::{State, Json};
use hyper::StatusCode;
use serde::Deserialize;
use std::sync::Arc;
use serde_json::{json, Value};
use crate::AppState;
use crate::action::create_account;

#[derive(Deserialize)]
pub struct Register {
  pub email: String,
  pub handle: String,
  pub password: String,
}

pub async fn register(
  State(state): State<Arc<AppState>>,
  Json(data): Json<Register>,
) -> (StatusCode, Json<Value>) {
  match create_account(&state.database, &data.handle, &data.email, data.password).await {
    Ok(_) => (StatusCode::CREATED, Json(json!({
      "code": 201,
      "success": true,
    }))),
    Err(e) => (e.status_code(), Json(json!({
      "code": e.status_code().as_u16(),
      "success": false,
      "error": e.as_ref(),
    })))
  }
}
