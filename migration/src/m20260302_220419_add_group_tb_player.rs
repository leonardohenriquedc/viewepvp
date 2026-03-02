use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table("tb_player")
                    .add_column(integer("group_id"))
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_group_player")
                            .from_tbl(Alias::new("tb_player"))
                            .from_col(Alias::new("group_id"))
                            .to_tbl(Alias::new("tb_group"))
                            .to_col(Alias::new("id")),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table("tb_player")
                    .drop_foreign_key(Alias::new("fk_group_player"))
                    .drop_column(Alias::new("group_id"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
