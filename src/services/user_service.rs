use crate::{
    dto::{
        user_dto::UserDto,
        user_relations::{GroupRelations, RoleRelations, UserRelations},
    },
    models::{
        tb_group, tb_group_user, tb_role,
        tb_user::{self},
    },
};
use bcrypt::DEFAULT_COST;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    JoinType, QueryFilter, QuerySelect, RelationTrait,
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

pub async fn find_user_by_email(
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

pub async fn get_user_and_relations(
    db: DatabaseConnection,
    user_dto: UserDto,
) -> Result<UserRelations, sea_orm::DbErr> {
    let user = find_user_by_email(db.clone(), user_dto).await?;

    let rows = tb_user::Entity::find()
        .select_only()
        .column(tb_user::Column::Id)
        .column(tb_user::Column::Email)
        .column(tb_user::Column::Password)
        .column_as(tb_group::Column::Id, "group_id")
        .column_as(tb_group::Column::Name, "group_name")
        .column_as(tb_role::Column::Id, "role_id")
        .column_as(tb_role::Column::Role, "role_name")
        .join(JoinType::InnerJoin, tb_group_user::Relation::TbUser.def())
        .join(JoinType::InnerJoin, tb_group_user::Relation::TbGroup.def())
        .join(JoinType::InnerJoin, tb_group_user::Relation::TbRole.def())
        .filter(tb_user::Column::Id.eq(user.id))
        .into_tuple::<(i32, String, String, i32, String, i32, String)>()
        .all(&db)
        .await?;

    if rows.is_empty() {
        return Err(DbErr::RecordNotFound("User not found".into()));
    }

    let mut groups = Vec::new();

    let (user_id, email, password, _, _, _, _) = &rows[0];

    for (_, _, _, group_id, group_name, role_id, role_name) in rows.clone() {
        groups.push(GroupRelations {
            id: group_id,
            name: group_name,
            role: RoleRelations {
                id: role_id,
                role: role_name,
            },
        });
    }

    Ok(UserRelations {
        id: *user_id,
        email: email.clone(),
        password: password.clone(),
        groups,
    })
}
