use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use crate::{dto::role_dto::RoleDto, models::tb_role};

pub async fn create_role(
    db: DatabaseConnection,
    role_dto: RoleDto,
) -> Result<String, sea_orm::DbErr> {
    let exists = tb_role::Entity::find()
        .filter(tb_role::Column::Role.eq(&role_dto.name))
        .one(&db)
        .await?;

    if exists.is_some() {
        tracing::debug!(role = %role_dto.name, " already exists");

        return Err(sea_orm::DbErr::Custom(format!(
            "The Role {} already exists",
            role_dto.name
        )));
    }

    let role = tb_role::ActiveModel {
        role: Set(role_dto.name.clone()),
        ..Default::default()
    };

    let result = role.save(&db).await;

    if result.is_err() {
        tracing::error!("error saving a entity role");

        return Err(sea_orm::DbErr::Custom(format!(
            "Error a saving entity tb_role: {}",
            role_dto.name
        )));
    }

    Ok(role_dto.name)
}

pub async fn find_role(
    db: DatabaseConnection,
    role_dto: RoleDto,
) -> Result<tb_role::Model, sea_orm::DbErr> {
    let role = tb_role::Entity::find()
        .filter(tb_role::Column::Role.eq(&role_dto.name))
        .one(&db)
        .await
        .unwrap()
        .ok_or_else(|| {
            tracing::error!("Role not found");
            sea_orm::DbErr::Custom("Role not found".to_string())
        })?;

    Ok(role)
}

pub async fn find_role_by_id(
    db: DatabaseConnection,
    id: i32,
) -> Result<tb_role::Model, sea_orm::DbErr> {
    let role = tb_role::Entity::find_by_id(id)
        .one(&db)
        .await
        .unwrap()
        .ok_or_else(|| {
            tracing::error!("Role not found");
            sea_orm::DbErr::Custom("Role not found".to_string())
        })?;

    Ok(role)
}
