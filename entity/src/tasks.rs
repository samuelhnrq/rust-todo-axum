use crate::generated::task::ActiveModel as TaskActiveModel;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};

pub use crate::{Task, TaskEntity};

#[derive(serde::Deserialize, Default)]
pub struct NewTask {
    pub title: String,
    pub owner: i32,
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
        entity.task_description = ActiveValue::Set(desc)
    }
    TaskEntity::insert(entity).exec_with_returning(db).await
}
