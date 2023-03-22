use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PullRequestEvent::Table)
                    .add_column(
                        ColumnDef::new(PullRequestEvent::PrLink)
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PullRequestEvent::Table)
                    .drop_column(PullRequestEvent::PrLink)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
#[allow(dead_code)]
enum PullRequestEvent {
    Table,
    Id,
    PrId,
    EventType,
    Author,
    Timestamp,
    Repository,
    Title,
    Text,
    PrLink,
}
