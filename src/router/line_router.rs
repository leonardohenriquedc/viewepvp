use actix_web::{web, HttpResponse, Responder};
use sea_orm::DbConn;

use crate::dto::line_dto::CreateLineRequest;
use crate::services::line_service;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/lines")
            .route("", web::get().to(list_lines))
            .route("", web::post().to(create_line)),
    );
}

async fn list_lines(db: web::Data<DbConn>) -> impl Responder {
    let result = line_service::list_lines(db.get_ref()).await;

    match result {
        Ok(lines) => HttpResponse::Ok().json(lines),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

async fn create_line(
    db: web::Data<DbConn>,
    body: web::Json<CreateLineRequest>,
) -> impl Responder {
    let result =
        line_service::create_line(db.get_ref(), body.name.clone(), body.nicknames.clone()).await;

    match result {
        Ok(line) => HttpResponse::Created().json(line),
        Err(err) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
