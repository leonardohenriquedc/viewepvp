use actix_web::{
    HttpResponseBuilder, Responder,
    http::{StatusCode, header::ContentType},
    web,
};
use sea_orm::DatabaseConnection;

use crate::{
    dto::{login_dto::LoginDto, user_dto::UserDto},
    services::{authentication_server::validate_login, user_service::create_user},
};

#[actix_web::post("/login")]
pub async fn login(
    db: web::Data<DatabaseConnection>,
    login_dto: web::Json<LoginDto>,
) -> impl Responder {
    let token = validate_login(db.get_ref().to_owned(), login_dto.into_inner()).await;

    if token.is_err() {
        return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED).finish();
    }

    let token = token.unwrap();

    HttpResponseBuilder::new(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .finish()
}

#[actix_web::post("/new-user")]
pub async fn new_user(
    db: web::Data<DatabaseConnection>,
    data: web::Json<UserDto>,
) -> impl Responder {
    let email_user = create_user(db.get_ref().to_owned(), data.into_inner()).await;

    if email_user.is_err() {
        tracing::debug!("It is not possible to create a user");
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
    }

    let email_user = email_user.ok().unwrap();

    HttpResponseBuilder::new(StatusCode::OK).body(email_user)
}
