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
    let result = player_service::list_players(db.get_ref()).await;

    match result {
        Ok(players) => HttpResponse::Ok().json(players),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

async fn create_player(
    db: web::Data<DbConn>,
    body: web::Json<CreatePlayerRequest>,
) -> impl Responder {
    let result = player_service::create_player(
        db.get_ref(),
        body.nickname.clone(),
        body.real_name.clone(),
    )
    .await;

    match result {
        Ok(player) => HttpResponse::Created().json(player),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

async fn delete_player(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let result = player_service::delete_player(db.get_ref(), id).await;

    match result {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
