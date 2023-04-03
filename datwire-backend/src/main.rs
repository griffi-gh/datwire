use std::{env, net::{SocketAddr, IpAddr}};
use axum::{Router, routing::get};
use anyhow::{Result, Context, anyhow};

pub(crate) struct Config {
  pub addr: IpAddr,
  pub port: u16,
  pub database_url: String,
}
impl Config {
  pub fn socket_addr(&self) -> SocketAddr {
    SocketAddr::new(self.addr, self.port)
  }
}
pub(crate) struct State {
  config: Config,
}

#[tokio::main]
async fn main() -> Result<()> {
  dotenvy::dotenv()?;
  tracing_subscriber::fmt::try_init().map_err(|err| anyhow!(err))?;

  let config = Config {
    addr: env::var("ADDR").unwrap_or("0.0.0.0".into()).parse().with_context(|| "ADDR is invalid")?,
    port: env::var("PORT").unwrap_or("8080".into()).parse().with_context(|| "PORT is invalid")?,
    database_url: env::var("DATABASE_URL").with_context(|| "DATABASE_URL not specified")?,
  };

  // build application
  let app = Router::new().route("/", get(|| async { "Hello, World!" }));

  // run it with hyper
  tracing::info!("server go brrrrr on {}", config.socket_addr());
  axum::Server::bind(&config.socket_addr())
    .serve(app.into_make_service())
    .await?;

  Ok(())
}
