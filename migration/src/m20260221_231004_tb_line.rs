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
                    .col(integer("player_one_id").not_null())
                    .col(integer("player_two_id").not_null())
                    .col(integer("player_three_id").not_null())
                    .col(integer("player_four_id").not_null())
                    .col(integer("player_five_id").not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_one_id")
                            .from("tb_line", "player_one_id")
                            .to("tb_player", "id"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_two_id")
                            .from("tb_line", "player_two_id")
                            .to("tb_player", "id"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_three_id")
                            .from("tb_line", "player_three_id")
                            .to("tb_player", "id"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_four_id")
                            .from("tb_line", "player_four_id")
                            .to("tb_player", "id"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_five_id")
                            .from("tb_line", "player_five_id")
                            .to("tb_player", "id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("tb_line").to_owned())
            .await
    }
}
