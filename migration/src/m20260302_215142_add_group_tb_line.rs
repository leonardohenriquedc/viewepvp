use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("tb_line"))
                    .add_column(integer("group_id"))
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_group_line")
                            .from_tbl(Alias::new("tb_line"))
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
                    .table(Alias::new("tb_line"))
                    .drop_foreign_key(Alias::new("fk_group_line"))
                    .drop_column(Alias::new("group_id"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
