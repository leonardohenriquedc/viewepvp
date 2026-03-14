use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table("tb_user")
                    .drop_foreign_key("fk_group_user")
                    .drop_column("group_id")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table("tb_user")
                    .add_column(integer("group_id"))
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_group_user")
                            .from_tbl("tb_user")
                            .from_col("group_id")
                            .to_tbl("tb_group")
                            .to_col("id"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
