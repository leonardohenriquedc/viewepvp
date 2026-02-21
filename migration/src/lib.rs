pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260221_225946_tb_player;
mod m20260221_230104_tb_confrontation;
mod m20260221_231004_tb_line;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260221_225946_tb_player::Migration),
            Box::new(m20260221_230104_tb_confrontation::Migration),
            Box::new(m20260221_231004_tb_line::Migration),
        ]
    }
}
