use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("tb_line_player"))
                    .if_not_exists()
                    .col(pk_auto(Alias::new("id")))
                    .col(integer(Alias::new("line_id")).not_null())
                    .col(integer(Alias::new("player_id")).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_line_player_line")
                            .from(Alias::new("tb_line_player"), Alias::new("line_id"))
                            .to(Alias::new("tb_line"), Alias::new("id")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_line_player_player")
                            .from(Alias::new("tb_line_player"), Alias::new("player_id"))
                            .to(Alias::new("tb_player"), Alias::new("id")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Alias::new("tb_line_player"))
                    .to_owned(),
            )
            .await
    }
}
