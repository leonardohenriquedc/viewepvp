use actix_web::{web, HttpResponse, Responder};
use sea_orm::DbConn;

use crate::dto::player_dto::CreatePlayerRequest;
use crate::services::player_service;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/players")
            .route("", web::get().to(list_players))
            .route("", web::post().to(create_player))
            .route("/{id}", web::delete().to(delete_player)),
    );
}

async fn list_players(db: web::Data<DbConn>) -> impl Responder {
    tracing::info!("GET /players - listing players");
    let result = player_service::list_players(db.get_ref()).await;

    match result {
        Ok(players) => {
            tracing::info!(count = players.len(), "Players listed successfully");
            HttpResponse::Ok().json(players)
        }
        Err(err) => {
            tracing::error!(error = %err, "Failed to list players");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": err.to_string()
            }))
        }
    }
}

async fn create_player(
    db: web::Data<DbConn>,
    body: web::Json<CreatePlayerRequest>,
) -> impl Responder {
    tracing::info!(nickname = %body.nickname, "POST /players - creating player");
    let result = player_service::create_player(
        db.get_ref(),
        body.nickname.clone(),
        body.real_name.clone(),
    )
    .await;

    match result {
        Ok(player) => {
            tracing::info!(nickname = %player.nickname, "Player created successfully");
            HttpResponse::Created().json(player)
        }
        Err(err) => {
            tracing::warn!(error = %err, "Failed to create player");
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": err.to_string()
            }))
        }
    }
}

async fn delete_player(
    db: web::Data<DbConn>,
    path: web::Path<String>,
) -> impl Responder {
    let nickname = path.into_inner();
    tracing::info!(nickname = %nickname, "DELETE /players - deleting player");
    let result = player_service::delete_player(db.get_ref(), &nickname).await;

    match result {
        Ok(()) => {
            tracing::info!(nickname = %nickname, "Player deleted successfully");
            HttpResponse::NoContent().finish()
        }
        Err(err) => {
            tracing::warn!(error = %err, "Failed to delete player");
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": err.to_string()
            }))
        }
    }
}
