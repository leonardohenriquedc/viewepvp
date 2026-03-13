use std::env;

use jsonwebtoken::{DecodingKey, Validation, decode};
use sea_orm::DatabaseConnection;

use crate::{
    dto::{user_dto::UserDto, user_relations::UserRelations},
    models::{custom_erros::CustomError, jwt_structs::Claims},
    services::{authentication_server::generate_token, user_service::get_user_and_relations},
};

pub async fn authentication_token(
    db: DatabaseConnection,
    token: String,
) -> Result<(String, UserRelations), CustomError> {
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(env::var("SECRET_KEY").unwrap().as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|err| CustomError::PermissionDenied("decoding error of the token".to_string()))?;

    let claims = decoded.claims;

    let user_relations = get_user_and_relations(
        db.clone(),
        UserDto {
            name: "".to_string(),
            email: claims.sub.clone(),
            password: "".to_string(),
        },
    )
    .await
    .map_err(|err| CustomError::NotFound("This sub not found".to_string()))?;

    let new_token = generate_token(claims)?;

    Ok((new_token, user_relations))
}

pub async fn authentication_roles_permissions_by_group(
    user: UserRelations,
    group_name: String,
) -> bool {
    let mut exists = false;

    user.groups.iter().for_each(|group| {
        if group.name == group_name {
            exists = true;
        }
    });

    exists
}
