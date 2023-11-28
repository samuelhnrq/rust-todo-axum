use crate::generated::task::ActiveModel as TaskActiveModel;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub use crate::{Task, TaskEntity};

#[derive(serde::Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
}

pub async fn list_all_tasks(db: &DatabaseConnection) -> Result<Vec<Task>, DbErr> {
    return TaskEntity::find().all(db).await;
}

pub async fn new_task(task: NewTask, db: &DatabaseConnection) -> Result<Task, DbErr> {
    let mut entity = TaskActiveModel {
        title: ActiveValue::Set(task.title),
        ..Default::default()
    };
    if let Some(desc) = task.description {
        entity.task_description = ActiveValue::Set(desc)
    }
    return TaskEntity::insert(entity).exec_with_returning(db).await;
}
