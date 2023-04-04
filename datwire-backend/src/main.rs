use std::{env, net::{SocketAddr, IpAddr}, sync::Arc, hint::black_box};
use axum::{Router, routing::get};
use anyhow::{Result, Context, anyhow};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::signal::ctrl_c;

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
  pub config: Config,
  pub database: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> Result<()> {
  //Read dotenv files
  dotenvy::dotenv()?;

  //Initialize tracing_subscritber
  tracing_subscriber::fmt::try_init().map_err(|err| anyhow!(err))?;

  //Read config from env
  let config = Config {
    addr: env::var("ADDR").unwrap_or("0.0.0.0".into()).parse().with_context(|| "ADDR is invalid")?,
    port: env::var("PORT").unwrap_or("8080".into()).parse().with_context(|| "PORT is invalid")?,
    database_url: env::var("DATABASE_URL").with_context(|| "DATABASE_URL not specified")?,
  };

  //Establish database connection
  tracing::info!("establishing database connection");
  let database = PgPoolOptions::new()
    .max_connections(5)
    .connect(&config.database_url).await?;
  
  //Run database migrations
  tracing::info!("running migrations");
  sqlx::migrate!().run(&database).await?;

  //Create main app state struct
  let state = Arc::new(State { config, database });

  // build application
  let app = Router::new()
    .nest("/api", Router::new()
      .route("/", get(|| async { "api service" }))
    )
    .route("/", get(|| async { "https://github.com/griffi-gh/datwire/" }))
    .with_state(Arc::clone(&state));
  
  // run it with hyper
  tracing::info!("server go brrrrr on http://{}/", state.config.socket_addr());
  let graceful = axum::Server::bind(&state.config.socket_addr())
    .serve(app.into_make_service())
    .with_graceful_shutdown(async { ctrl_c().await.unwrap() })
    .await;
  if let Err(err) = graceful {
    tracing::error!("Server error: {}", err);
  }
  tracing::warn!("server died :(");

  tracing::info!("closing database connection");
  state.database.close().await;

  Ok(())
}
