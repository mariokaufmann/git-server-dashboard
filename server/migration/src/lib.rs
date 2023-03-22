pub use sea_orm_migration::prelude::*;

mod m20230306_145109_create_initial_schema;
mod m20230322_211054_add_pr_link;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230306_145109_create_initial_schema::Migration),
            Box::new(m20230322_211054_add_pr_link::Migration),
        ]
    }
}
