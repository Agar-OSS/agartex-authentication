mod config;
mod constants;
mod control;
mod database;
mod domain;
mod repository;
mod routing;
mod service;

use tracing::{error, info};

use config::CONFIG;


#[tracing::instrument]
pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let pool = match database::create_conn_pool().await {
        Ok(pool) => pool,
        Err(err) => {
            error!("Could not connect to database:\n{:?}", err);
            return Err(anyhow::Error::from(err))
        }
    };

    info!("Running server!");
    axum::Server::try_bind(&CONFIG.server_addr)?
        .serve(routing::main_router(&pool).into_make_service())
        .await
        .map_err(anyhow::Error::from)
}

#[tokio::main]
#[tracing::instrument]
async fn main() {
    info!("Starting application");
    if let Err(err) = run().await {
        error!(%err);
    }
}
