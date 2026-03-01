use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, ModelTrait, QueryFilter, Set};
use tracing;

use crate::dto::player_dto::PlayerResponse;
use crate::models::player;

pub async fn list_players(db: &DbConn) -> Result<Vec<PlayerResponse>, sea_orm::DbErr> {
    let players = player::Entity::find().all(db).await?;

    Ok(players
        .into_iter()
        .map(|p| PlayerResponse {
            nickname: p.nickname,
            real_name: p.real_name,
        })
        .collect())
}

pub async fn create_player(
    db: &DbConn,
    nickname: String,
    real_name: String,
) -> Result<PlayerResponse, sea_orm::DbErr> {
    let existing = player::Entity::find()
        .filter(player::Column::Nickname.eq(&nickname))
        .one(db)
        .await?;

    if existing.is_some() {
        tracing::debug!(nickname = %nickname, "Nickname already exists");
        return Err(sea_orm::DbErr::Custom(format!(
            "Nickname '{}' já existe",
            nickname
        )));
    }

    let new_player = player::ActiveModel {
        nickname: Set(nickname),
        real_name: Set(real_name),
        ..Default::default()
    };

    let created = new_player.insert(db).await?;

    Ok(PlayerResponse {
        nickname: created.nickname,
        real_name: created.real_name,
    })
}

pub async fn delete_player(db: &DbConn, nickname: &str) -> Result<(), sea_orm::DbErr> {
    let player = player::Entity::find()
        .filter(player::Column::Nickname.eq(nickname))
        .one(db)
        .await?;

    match player {
        Some(p) => {
            p.delete(db).await?;
            Ok(())
        }
        None => Err(sea_orm::DbErr::Custom(format!(
            "Player with nickname '{}' not found",
            nickname
        ))),
    }
}
