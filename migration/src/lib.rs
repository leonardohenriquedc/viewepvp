pub use sea_orm_migration::prelude::*;

mod m20260221_225946_tb_player;
mod m20260221_230000_tb_line;
mod m20260221_230100_tb_line_player;
mod m20260221_230200_tb_confrontation;
mod m20260302_212912_tb_user;
mod m20260302_213032_tb_group;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260221_225946_tb_player::Migration),
            Box::new(m20260221_230000_tb_line::Migration),
            Box::new(m20260221_230100_tb_line_player::Migration),
            Box::new(m20260221_230200_tb_confrontation::Migration),
            Box::new(m20260302_212912_tb_user::Migration),
            Box::new(m20260302_213032_tb_group::Migration),
        ]
    }
}
