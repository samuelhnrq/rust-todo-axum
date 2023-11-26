use entity::task::Entity as TaskEntity;
pub use entity::task::Model as Task;

use sea_orm::{error::DbErr, DatabaseConnection, EntityTrait};

pub async fn list_all_tasks(db: &DatabaseConnection) -> Result<Vec<Task>, DbErr> {
    return TaskEntity::find().all(db).await;
}
