use actix_web::{
    HttpRequest, HttpResponseBuilder, Responder,
    http::{
        StatusCode,
        header::{self, ContentType},
    },
    web,
};
use sea_orm::DatabaseConnection;

use crate::{dto::group_dto::GroupDto, services::autorization_server::authentication_token};

#[actix_web::get("/groups")]
pub async fn get_groups(req: HttpRequest, db: web::Data<DatabaseConnection>) -> impl Responder {
    let token = match req.headers().get(header::AUTHORIZATION) {
        Some(h) => match h.to_str() {
            Ok(v) => v,
            Err(_) => {
                tracing::debug!("Invalid authorization header format");
                return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
            }
        },
        None => {
            tracing::debug!("Token not found");
            return HttpResponseBuilder::new(StatusCode::FORBIDDEN).finish();
        }
    };

    let token = token.trim().replace("Bearer ", "");

    let auth_result = authentication_token(db.as_ref().to_owned(), token.to_string()).await;

    if auth_result.is_err() {
        tracing::debug!("Not valid token");
        return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED).finish();
    }

    let (token, user_relations) = auth_result.unwrap();

    let groups: Vec<GroupDto> = user_relations
        .groups
        .iter()
        .map(|group| GroupDto {
            name: group.name.to_owned(),
        })
        .collect();

    HttpResponseBuilder::new(StatusCode::OK)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .json(groups)
}
