use std::env;

use bcrypt::verify;
use jsonwebtoken::{EncodingKey, Header as JHeader, encode};
use sea_orm::DatabaseConnection;

use crate::{
    dto::{login_dto::LoginDto, user_dto::UserDto},
    models::{custom_erros::CustomError, jwt_structs::Claims},
    services::user_service::{create_user, find_user_by_email},
};

pub async fn validate_login(
    db: DatabaseConnection,
    login_dto: LoginDto,
) -> Result<String, CustomError> {
    tracing::debug!("Called validate_login");

    let user_dto = UserDto {
        email: login_dto.email.clone(),
        name: "".to_string(),
        password: "".to_string(),
    };

    let user = find_user_by_email(db.clone(), user_dto).await;

    if user.is_err() {
        tracing::debug!("Uset not found");
        return Err(CustomError::PermissionDenied(login_dto.email));
    }

    let user = user.ok().unwrap();

    let password_verify = verify(login_dto.password.as_str(), user.password.as_str()).unwrap();

    if !password_verify {
        tracing::debug!("Invalid Token");
        return Err(CustomError::PermissionDenied(login_dto.email));
    }

    let claims = Claims {
        sub: user.email,
        exp: (chrono::Utc::now().timestamp() + 3600) as usize,
        iat: chrono::Utc::now().timestamp() as usize,
    };

    let result_token = generate_token(claims)?;

    Ok(result_token)
}

fn generate_token(claims: Claims) -> Result<String, CustomError> {
    let token = encode(
        &JHeader::default(),
        &claims,
        &EncodingKey::from_secret(env::var("SECRET_KEY").unwrap().as_ref()),
    );

    if token.is_err() {
        return Err(CustomError::ErrorCreating);
    }

    let token = token.ok().unwrap();

    Ok(token)
}

pub async fn new_user(db: DatabaseConnection, user_dto: UserDto) -> Result<String, CustomError> {
    let user = create_user(db.clone(), user_dto.clone()).await;

    if user.is_err() {
        tracing::debug!("User already exists: {}", user_dto.email);

        return Err(CustomError::ThisObjectAlreadyExists);
    }

    Ok(user.ok().unwrap())
}
