use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, ModelTrait, Set};

use crate::models::player;

pub async fn list_players(db: &DbConn) -> Result<Vec<player::Model>, sea_orm::DbErr> {
    player::Entity::find().all(db).await
}

pub async fn create_player(
    db: &DbConn,
    nickname: String,
    real_name: String,
) -> Result<player::Model, sea_orm::DbErr> {
    let new_player = player::ActiveModel {
        nickname: Set(nickname),
        real_name: Set(real_name),
        ..Default::default()
    };

    new_player.insert(db).await
}

pub async fn delete_player(db: &DbConn, id: i32) -> Result<(), sea_orm::DbErr> {
    let player = player::Entity::find_by_id(id).one(db).await?;

    match player {
        Some(p) => {
            p.delete(db).await?;
            Ok(())
        }
        None => Err(sea_orm::DbErr::Custom(format!(
            "Player with id {} not found",
            id
        ))),
    }
}
