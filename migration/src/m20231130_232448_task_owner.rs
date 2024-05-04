use crate::tables::{Task, Users};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).string().primary_key())
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Task::Table)
                    .add_column(ColumnDef::new(Task::Owner).string().not_null())
                    .to_owned(),
            )
            .await?;
        let owner_ref = TableForeignKey::new()
            .name("owner_fk")
            .from_tbl(Task::Table)
            .from_col(Task::Owner)
            .to_tbl(Users::Table)
            .to_col(Users::Id)
            .to_owned();
        manager
            .alter_table(
                Table::alter()
                    .table(Task::Table)
                    .add_foreign_key(&owner_ref)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Task::Table)
                    .drop_column(Task::Owner)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
