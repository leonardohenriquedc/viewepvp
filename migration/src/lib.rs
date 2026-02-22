pub use sea_orm_migration::prelude::*;

mod m20260221_225946_tb_player;
mod m20260221_230000_tb_line;
mod m20260221_230100_tb_line_player;
mod m20260221_230200_tb_confrontation;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260221_225946_tb_player::Migration),
            Box::new(m20260221_230000_tb_line::Migration),
            Box::new(m20260221_230100_tb_line_player::Migration),
            Box::new(m20260221_230200_tb_confrontation::Migration),
        ]
    }
}
