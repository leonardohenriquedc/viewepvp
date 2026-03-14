use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table("tb_confrontation")
                    .add_column(integer("group_id"))
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_group_confrontation")
                            .from_tbl(Alias::new("tb_confrontation"))
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
                    .table("tb_confrontation")
                    .drop_foreign_key("fk_group_confrontation")
                    .drop_column("group_id")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
