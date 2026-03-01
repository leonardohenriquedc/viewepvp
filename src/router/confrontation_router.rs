use actix_web::{web, HttpResponse, Responder};
use sea_orm::DbConn;

use crate::dto::confrontation_dto::CreateConfrontationRequest;
use crate::services::confrontation_service;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/confrontations")
            .route("", web::get().to(list_confrontations))
            .route("", web::post().to(create_confrontation)),
    );
}

async fn list_confrontations(db: web::Data<DbConn>) -> impl Responder {
    tracing::info!("GET /confrontations - listing confrontations");
    let result = confrontation_service::list_confrontations(db.get_ref()).await;

    match result {
        Ok(confrontations) => {
            tracing::info!(count = confrontations.len(), "Confrontations listed successfully");
            HttpResponse::Ok().json(confrontations)
        }
        Err(err) => {
            tracing::error!(error = %err, "Failed to list confrontations");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": err.to_string()
            }))
        }
    }
}

async fn create_confrontation(
    db: web::Data<DbConn>,
    body: web::Json<CreateConfrontationRequest>,
) -> impl Responder {
    tracing::info!(
        line_one = %body.line_one_name,
        line_two = %body.line_two_name,
        "POST /confrontations - creating confrontation"
    );
    let result =
        confrontation_service::create_confrontation(db.get_ref(), body.into_inner()).await;

    match result {
        Ok(confrontation) => {
            tracing::info!(
                line_one = %confrontation.line_one_name,
                line_two = %confrontation.line_two_name,
                "Confrontation created successfully"
            );
            HttpResponse::Created().json(confrontation)
        }
        Err(err) => {
            tracing::warn!(error = %err, "Failed to create confrontation");
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": err.to_string()
            }))
        }
    }
}
