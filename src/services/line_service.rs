use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set, TransactionTrait};
use tracing;

use crate::dto::line_dto::LineWithPlayersResponse;
use crate::models::{line, line_player, player};

pub async fn list_lines(db: &DbConn) -> Result<Vec<LineWithPlayersResponse>, sea_orm::DbErr> {
    let lines = line::Entity::find().all(db).await?;
    let line_players = line_player::Entity::find().all(db).await?;
    let players = player::Entity::find().all(db).await?;

    let result = lines
        .into_iter()
        .map(|l| {
            let nicknames: Vec<String> = line_players
                .iter()
                .filter(|lp| lp.line_id == l.id)
                .filter_map(|lp| {
                    players
                        .iter()
                        .find(|p| p.id == lp.player_id)
                        .map(|p| p.nickname.clone())
                })
                .collect();

            LineWithPlayersResponse {
                name: l.name,
                nicknames,
            }
        })
        .collect();

    Ok(result)
}

pub async fn create_line(
    db: &DbConn,
    name: String,
    nicknames: [String; 5],
) -> Result<LineWithPlayersResponse, sea_orm::DbErr> {
    let found_players = player::Entity::find()
        .filter(player::Column::Nickname.is_in(&nicknames))
        .all(db)
        .await?;

    if found_players.len() != 5 {
        tracing::debug!(requested = ?nicknames, "Some players not found");
        let found_nicks: Vec<&str> = found_players.iter().map(|p| p.nickname.as_str()).collect();
        let missing: Vec<&str> = nicknames
            .iter()
            .filter(|n| !found_nicks.contains(&n.as_str()))
            .map(|n| n.as_str())
            .collect();

        return Err(sea_orm::DbErr::Custom(format!(
            "Players not found: {}",
            missing.join(", ")
        )));
    }

    let txn = db.begin().await?;

    let new_line = line::ActiveModel {
        name: Set(name),
        ..Default::default()
    };
    let created_line = new_line.insert(&txn).await?;

    for p in &found_players {
        let link = line_player::ActiveModel {
            line_id: Set(created_line.id),
            player_id: Set(p.id),
            ..Default::default()
        };
        link.insert(&txn).await?;
    }

    txn.commit().await?;

    Ok(LineWithPlayersResponse {
        name: created_line.name,
        nicknames: nicknames.to_vec(),
    })
}
