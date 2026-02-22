mod dto;
mod models;
mod router;
mod services;

use actix_web::{web, App, HttpServer};
use sea_orm::Database;

use router::{confrontation_router, line_router, player_router};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::connect("sqlite://database.db")
        .await
        .expect("Failed to connect to database");

    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(player_router::config)
            .configure(line_router::config)
            .configure(confrontation_router::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
