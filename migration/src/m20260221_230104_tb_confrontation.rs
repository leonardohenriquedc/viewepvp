use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("tb_confrontation")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(integer("line_one_id"))
                    .col(integer("line_two_id"))
                    .col(date("date_of_confrontation"))
                    .col(smallinteger("point_of_line_one"))
                    .col(smallinteger("point_of_line_two"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_line_one")
                            .from("tb_confrontation", "line_one_id")
                            .to("tb_line", "id"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_line_two")
                            .from("tb_confrontation", "line_two_id")
                            .to("tb_line", "id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("tb_confrontation").to_owned())
            .await
    }
}
