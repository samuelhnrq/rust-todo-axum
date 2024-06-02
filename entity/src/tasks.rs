use crate::generated::{prelude::Task, task};
use sea_orm::{prelude::Uuid, ActiveValue, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};
use serde_valid::Validate;

#[derive(serde::Deserialize, Default, Validate, Debug, Clone)]
#[serde(default)]
pub struct NewTask {
    #[validate(min_length = 1)]
    pub title: String,
    pub owner: Uuid,
    pub description: Option<String>,
}

pub async fn list_all(
    db: &DatabaseConnection,
    num_page: Option<u16>,
    page_size: Option<u16>,
) -> Result<Vec<task::Model>, DbErr> {
    Task::find()
        .paginate(db, u64::from(page_size.unwrap_or(50)))
        .fetch_page(u64::from(num_page.unwrap_or(0)))
        .await
}

pub async fn new_task(task: NewTask, db: &DatabaseConnection) -> Result<task::Model, DbErr> {
    let mut entity = task::ActiveModel {
        title: ActiveValue::Set(task.title),
        owner_id: ActiveValue::Set(task.owner),
        ..Default::default()
    };
    if let Some(desc) = task.description {
        entity.description = ActiveValue::Set(desc);
    }
    Task::insert(entity).exec_with_returning(db).await
}
