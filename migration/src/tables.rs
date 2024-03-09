use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Task {
    Table,
    Id,
    Title,
    Description,
    Done,
    Owner,
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Name,
    Email,
    CreatedAt,
    UpdatedAt,
}
