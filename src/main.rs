mod constants;
mod control;
mod domain;
mod repository;
mod routing;
mod service;
mod validation;

use tracing::{error, info};

use constants::SERVER_URL;


#[tracing::instrument]
pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    info!("Running server!");
    axum::Server::try_bind(&SERVER_URL)?
        .serve(routing::main_router().into_make_service())
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
