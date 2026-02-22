use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("tb_confrontation"))
                    .if_not_exists()
                    .col(pk_auto(Alias::new("id")))
                    .col(integer(Alias::new("line_one_id")).not_null())
                    .col(integer(Alias::new("line_two_id")).not_null())
                    .col(date(Alias::new("date_of_confrontation")).not_null())
                    .col(small_integer(Alias::new("point_of_line_one")).not_null())
                    .col(small_integer(Alias::new("point_of_line_two")).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_confrontation_line_one")
                            .from(Alias::new("tb_confrontation"), Alias::new("line_one_id"))
                            .to(Alias::new("tb_line"), Alias::new("id")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_confrontation_line_two")
                            .from(Alias::new("tb_confrontation"), Alias::new("line_two_id"))
                            .to(Alias::new("tb_line"), Alias::new("id")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Alias::new("tb_confrontation"))
                    .to_owned(),
            )
            .await
    }
}
