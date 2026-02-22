use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};

use crate::dto::confrontation_dto::{ConfrontationResponse, CreateConfrontationRequest};
use crate::models::{confrontation, line};

pub async fn create_confrontation(
    db: &DbConn,
    req: CreateConfrontationRequest,
) -> Result<ConfrontationResponse, sea_orm::DbErr> {
    let line_one = line::Entity::find()
        .filter(line::Column::Name.eq(&req.line_one_name))
        .one(db)
        .await?
        .ok_or_else(|| {
            sea_orm::DbErr::Custom(format!("Line with name '{}' not found", req.line_one_name))
        })?;

    let line_two = line::Entity::find()
        .filter(line::Column::Name.eq(&req.line_two_name))
        .one(db)
        .await?
        .ok_or_else(|| {
            sea_orm::DbErr::Custom(format!("Line with name '{}' not found", req.line_two_name))
        })?;

    let date = chrono::NaiveDate::parse_from_str(&req.date_of_confrontation, "%Y-%m-%d")
        .map_err(|e| sea_orm::DbErr::Custom(format!("Invalid date format: {}", e)))?;

    let new_confrontation = confrontation::ActiveModel {
        line_one_id: Set(line_one.id),
        line_two_id: Set(line_two.id),
        date_of_confrontation: Set(date),
        point_of_line_one: Set(req.point_of_line_one),
        point_of_line_two: Set(req.point_of_line_two),
        ..Default::default()
    };

    let created = new_confrontation.insert(db).await?;

    Ok(ConfrontationResponse {
        line_one_name: line_one.name,
        line_two_name: line_two.name,
        date_of_confrontation: created.date_of_confrontation.to_string(),
        point_of_line_one: created.point_of_line_one,
        point_of_line_two: created.point_of_line_two,
    })
}
