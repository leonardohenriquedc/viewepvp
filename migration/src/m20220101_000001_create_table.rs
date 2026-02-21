use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("tb_line")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(integer("player_one_id"))
                    .col(integer("player_two_id"))
                    .col(integer("player_three_id"))
                    .col(integer("player_four_id"))
                    .col(integer("player_five_id"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("post").to_owned())
            .await
    }
}
