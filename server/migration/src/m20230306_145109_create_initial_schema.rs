use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PullRequestEvent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PullRequestEvent::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PullRequestEvent::PrId).text().not_null())
                    .col(
                        ColumnDef::new(PullRequestEvent::EventType)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PullRequestEvent::Author).string().not_null())
                    .col(
                        ColumnDef::new(PullRequestEvent::Timestamp)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PullRequestEvent::Repository)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PullRequestEvent::Title).string().not_null())
                    .col(ColumnDef::new(PullRequestEvent::Text).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PullRequestEvent::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
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
}
