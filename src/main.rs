mod dto;
mod models;
mod router;
mod services;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, web};
use dotenv::dotenv;
use sea_orm::Database;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use router::{confrontation_router, line_router, player_router};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting server on 127.0.0.1:8080");

    let db = Database::connect("sqlite://database.db")
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connected");

    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![header::CONTENT_TYPE])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(db_data.clone())
            .configure(player_router::config)
            .configure(line_router::config)
            .configure(confrontation_router::config)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
    .inspect(|_| tracing::info!("Server stopped"))
}
