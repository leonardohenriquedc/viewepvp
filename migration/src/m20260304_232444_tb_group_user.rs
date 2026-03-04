use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("tb_group_user")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(integer("user_id"))
                    .col(integer("group_id"))
                    .col(integer("role_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_group_user_user")
                            .from(Alias::new("tb_group_user"), Alias::new("user_id"))
                            .to(Alias::new("tb_user"), Alias::new("id")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_group_user_group")
                            .from(Alias::new("tb_group_user"), Alias::new("group_id"))
                            .to(Alias::new("tb_group"), Alias::new("id")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_group_user_role")
                            .from(Alias::new("tb_group_user"), Alias::new("role_id"))
                            .to(Alias::new("tb_role"), Alias::new("id")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("tb_group_user").to_owned())
            .await
    }
}
