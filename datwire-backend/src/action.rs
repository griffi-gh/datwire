//TODO move this somewhere
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
use hyper::StatusCode;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum AccountCreationError {
  #[error("too much users with the same username")]
  TooMuchUsers,
  #[error("email already in use by a different account")]
  EmailAlreadyInUse,
  #[error("password hashing failed")]
  PasswordHashError(#[from] #[serde(skip)] argon2::Error),
  #[error("database error")]
  DatabaseError(#[from] #[serde(skip)] sqlx::Error),
}
impl AccountCreationError {
  pub fn status_code(&self) -> StatusCode {
    match self {
      Self::EmailAlreadyInUse    => StatusCode::CONFLICT,
      Self::TooMuchUsers         => StatusCode::CONFLICT,
      Self::PasswordHashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Self::DatabaseError(_)     => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

pub async fn create_account(
  db: &Pool<Postgres>,
  handle: &str,
  email: &str,
  password: String,
) -> Result<(), AccountCreationError> {
  {
    //Start transaction
    let mut transaction = db.begin().await?;

    //Check if email is in use
    let email_in_use = sqlx::query!(r#"
      SELECT COUNT(*) AS "count!" FROM users WHERE email = $1;
    "#, email).fetch_one(&mut transaction).await?.count > 0;

    //Email adresses can only be used once
    if email_in_use { Err(AccountCreationError::EmailAlreadyInUse)? }

    //Determine user's tag
    let tag = sqlx::query!(r#"
      SELECT COUNT(*) AS "count!" FROM users WHERE handle = $1;
    "#, handle).fetch_one(&mut transaction).await?.count as i32;

    //If the tag is over 9999, we can't create any more accounts with that handle
    if tag > 9999 { Err(AccountCreationError::TooMuchUsers)? }
    
    //Calculate the password hash
    //I probably shouldn't be unwrapping here but the thread can't panic so... 
    let password_hash = tokio::task::spawn_blocking(move || -> Result<String, AccountCreationError> {
      let salt = SaltString::generate(&mut OsRng);
      let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt).unwrap();
      Ok(password_hash.to_string())
    }).await.unwrap()?;

    //Insert the user
    sqlx::query!(r#"
      INSERT INTO users (handle, tag, email, password_hash) VALUES ($1, $2, $3, $4);
    "#, handle, tag, email, password_hash).execute(&mut transaction).await?;

    //Commit the transaction
    transaction.commit().await?;
  }

  Ok(())
}
