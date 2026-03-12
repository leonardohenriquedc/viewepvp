use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("tb_role_permissions")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(integer("role_id"))
                    .col(integer("permission_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_permission_role")
                            .from(Alias::new("tb_role_permissions"), Alias::new("role_id"))
                            .to(Alias::new("tb_role"), Alias::new("id")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_permission_permission")
                            .from(
                                Alias::new("tb_role_permissions"),
                                Alias::new("permission_id"),
                            )
                            .to(Alias::new("tb_permissions"), Alias::new("id")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("tb_role_permissions").to_owned())
            .await
    }
}
