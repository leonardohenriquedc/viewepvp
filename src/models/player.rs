use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tb_player")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub nickname: String,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub real_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::line_player::Entity")]
    LinePlayer,
}

impl Related<super::line_player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LinePlayer.def()
    }
}

impl Related<super::line::Entity> for Entity {
    fn to() -> RelationDef {
        super::line_player::Relation::Line.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::line_player::Relation::Player.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
