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
    tracing::info!("GET /lines - listing lines");
    let result = line_service::list_lines(db.get_ref()).await;

    match result {
        Ok(lines) => {
            tracing::info!(count = lines.len(), "Lines listed successfully");
            HttpResponse::Ok().json(lines)
        }
        Err(err) => {
            tracing::error!(error = %err, "Failed to list lines");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": err.to_string()
            }))
        }
    }
}

async fn create_line(
    db: web::Data<DbConn>,
    body: web::Json<CreateLineRequest>,
) -> impl Responder {
    tracing::info!(name = %body.name, "POST /lines - creating line");
    let result =
        line_service::create_line(db.get_ref(), body.name.clone(), body.nicknames.clone()).await;

    match result {
        Ok(line) => {
            tracing::info!(name = %line.name, "Line created successfully");
            HttpResponse::Created().json(line)
        }
        Err(err) => {
            tracing::warn!(error = %err, "Failed to create line");
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": err.to_string()
            }))
        }
    }
}
