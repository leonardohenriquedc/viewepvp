use crate::{
    dto::{
        group_dto::GroupDto, role_dto::RoleDto, user_dto::UserDto, user_relations::GroupRelations,
    },
    models::tb_group_user,
    services::{group_service::find_group, roles_service::find_role},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

pub async fn create_group_user(
    db: DatabaseConnection,
    user_dto: UserDto,
    group_dto: GroupDto,
    role_dto: RoleDto,
) -> Result<String, sea_orm::DbErr> {
    let existe_role = find_role(db.clone(), role_dto.clone()).await?;
    let exists_user = find_group(db.clone(), group_dto.clone()).await?;
    let exists_group = find_group(db.clone(), group_dto.clone()).await?;

    let exists_group_user = tb_group_user::Entity::find()
        .filter(tb_group_user::Column::UserId.eq(exists_user.id))
        .filter(tb_group_user::Column::GroupId.eq(exists_group.id))
        .filter(tb_group_user::Column::RoleId.eq(existe_role.id))
        .one(&db)
        .await?;

    if exists_group_user.is_some() {
        tracing::error!("This conection table already exists");

        return Err(sea_orm::DbErr::Custom(format!(
            "This conection already exists, user: {}, group: {}, role: {}",
            user_dto.email, group_dto.name, role_dto.name
        )));
    }

    let group_user = tb_group_user::ActiveModel {
        user_id: Set(exists_user.id.into()),
        group_id: Set(exists_group.id.into()),
        role_id: Set(existe_role.id.into()),
        ..Default::default()
    };

    let result = group_user.save(&db).await;

    if result.is_err() {
        tracing::error!("Not possible saving entity");

        return Err(sea_orm::DbErr::Custom(format!(
            "Not possible saving entity"
        )));
    }

    Ok(user_dto.email)
}
