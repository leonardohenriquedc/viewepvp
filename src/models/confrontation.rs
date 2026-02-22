use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tb_confrontation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub line_one_id: i32,
    pub line_two_id: i32,
    pub date_of_confrontation: Date,
    pub point_of_line_one: i16,
    pub point_of_line_two: i16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::line::Entity",
        from = "Column::LineOneId",
        to = "super::line::Column::Id"
    )]
    LineOne,
    #[sea_orm(
        belongs_to = "super::line::Entity",
        from = "Column::LineTwoId",
        to = "super::line::Column::Id"
    )]
    LineTwo,
}

impl Related<super::line::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LineOne.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
