use crate::{
    dto::user_dto::UserDto,
    models::tb_user::{self, Model},
};
use bcrypt::DEFAULT_COST;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

pub async fn create_user(
    db: DatabaseConnection,
    user_dto: UserDto,
) -> Result<String, sea_orm::error::DbErr> {
    let existing = tb_user::Entity::find()
        .filter(tb_user::Column::Email.eq(&user_dto.email))
        .one(&db)
        .await?;

    if existing.is_some() {
        tracing::debug!(email = %user_dto.email, "Email is already exists");
        return Err(sea_orm::DbErr::Custom(format!(
            "Email {}, already exists",
            user_dto.email
        )));
    }

    let password = bcrypt::hash(user_dto.password, DEFAULT_COST)
        .expect("Not possible a generated hash of password");

    let user = tb_user::ActiveModel {
        name: Set(user_dto.name),
        email: Set(user_dto.email.clone()),
        password: Set(password),
        ..Default::default()
    };

    let result = user.save(&db).await;

    if result.is_err() {
        tracing::error!("Error saving entity: {}", user_dto.email.clone());

        return Err(sea_orm::DbErr::RecordNotInserted);
    }

    Ok(user_dto.email)
}

pub async fn find_user(
    db: DatabaseConnection,
    user_dto: UserDto,
) -> Result<tb_user::Model, sea_orm::DbErr> {
    let user = tb_user::Entity::find()
        .filter(tb_user::Column::Email.eq(&user_dto.email))
        .one(&db)
        .await
        .unwrap()
        .ok_or_else(|| {
            tracing::error!("User not found");
            sea_orm::DbErr::Custom("User not found".to_string())
        })?;

    Ok(user)
}
