use actix_web::{web, HttpResponse, Responder};
use sea_orm::DbConn;

use crate::dto::confrontation_dto::CreateConfrontationRequest;
use crate::services::confrontation_service;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/confrontations").route("", web::post().to(create_confrontation)),
    );
}

async fn create_confrontation(
    db: web::Data<DbConn>,
    body: web::Json<CreateConfrontationRequest>,
) -> impl Responder {
    let result =
        confrontation_service::create_confrontation(db.get_ref(), body.into_inner()).await;

    match result {
        Ok(confrontation) => HttpResponse::Created().json(confrontation),
        Err(err) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
