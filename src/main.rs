use std::{str::FromStr, sync::Arc};

use axum::{routing::get, Router};
use lighthouse::{
    config::{ContentConfig, ImageConfig, ServerConfig},
    handlers::{self, HomeState, TileState},
};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, Registry};

#[tokio::main]
async fn main() {
    // load config
    let server_config = ServerConfig::read();
    let content_config = ContentConfig::read();
    let image_config = ImageConfig::read();

    // init logger
    let subscriber = Registry::default()
        .with(LevelFilter::from_level(
            Level::from_str(&server_config.logging_level).expect(&format!(
                "enable to parse logging level {}",
                &server_config.logging_level
            )),
        ))
        .with(tracing_subscriber::fmt::Layer::default().with_writer(std::io::stdout));
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    // build our application with a route
    let app = Router::new()
        .route("/", get(handlers::home))
        .with_state(Arc::from(HomeState::from(&content_config, &image_config)))
        .route("/img/:index", get(handlers::tile))
        .with_state(Arc::from(TileState::from(&image_config)))
        .route_service(
            "/assets/styles/output.css",
            ServeFile::new("assets/styles/output.css"),
        )
        .nest_service("/assets/img", ServeDir::new("assets/img"))
        .nest_service("/assets/fonts", ServeDir::new("assets/fonts"))
        .layer(TraceLayer::new_for_http());

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
