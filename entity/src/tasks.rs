use crate::generated::task::ActiveModel as TaskActiveModel;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};
use serde_valid::Validate;

pub use crate::{Task, TaskEntity};

#[derive(serde::Deserialize, Default, Validate, Debug, Clone)]
#[serde(default)]
pub struct NewTask {
    #[validate(min_length = 1)]
    pub title: String,
    #[validate(min_length = 1)]
    pub owner: String,
    pub description: Option<String>,
}

pub async fn list_all_tasks(
    db: &DatabaseConnection,
    num_page: Option<u16>,
    page_size: Option<u16>,
) -> Result<Vec<Task>, DbErr> {
    TaskEntity::find()
        .paginate(db, page_size.unwrap_or(50) as u64)
        .fetch_page(num_page.unwrap_or(0) as u64)
        .await
}

pub async fn new_task(task: NewTask, db: &DatabaseConnection) -> Result<Task, DbErr> {
    let mut entity = TaskActiveModel {
        title: ActiveValue::Set(task.title),
        owner: ActiveValue::Set(task.owner),
        ..Default::default()
    };
    if let Some(desc) = task.description {
        entity.description = ActiveValue::Set(desc)
    }
    TaskEntity::insert(entity).exec_with_returning(db).await
}
