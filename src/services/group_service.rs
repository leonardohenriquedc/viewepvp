use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use crate::{
    dto::group_dto::{self, GroupDto},
    models::tb_group,
};

pub async fn create_group(
    db: DatabaseConnection,
    group_dto: GroupDto,
) -> Result<String, sea_orm::DbErr> {
    let exists = tb_group::Entity::find()
        .filter(tb_group::Column::Name.eq(&group_dto.name))
        .one(&db)
        .await?;

    if exists.is_some() {
        tracing::debug!(group = %group_dto.name, "group already exists");

        return Err(sea_orm::DbErr::Custom(format!(
            "The group {} already exists",
            group_dto.name
        )));
    }

    let group = tb_group::ActiveModel {
        name: Set(Some(group_dto.name.clone())),
        ..Default::default()
    };

    let result = group.save(&db).await;

    if result.is_err() {
        tracing::error!("Erro saving entity: {}", group_dto.name);

        return Err(sea_orm::DbErr::Custom(format!(
            "Error saving entity: {}",
            group_dto.name
        )));
    }

    Ok(group_dto.name)
}

pub async fn find_group(
    db: DatabaseConnection,
    group: GroupDto,
) -> Result<tb_group::Model, sea_orm::DbErr> {
    let group = tb_group::Entity::find()
        .filter(tb_group::Column::Name.eq(&group.name))
        .one(&db)
        .await
        .unwrap()
        .ok_or_else(|| {
            tracing::error!("Role not found");
            sea_orm::DbErr::Custom("Role not found".to_string())
        })
        .unwrap();

    Ok(group)
}
