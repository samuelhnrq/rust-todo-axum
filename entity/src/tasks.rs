use crate::generated::{prelude::Tasks, tasks, users};
use sea_orm::{
    prelude::Uuid, sea_query::OnConflict, ActiveValue, ColumnTrait, DatabaseConnection, DbErr,
    EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder,
};

#[derive(Debug, Clone)]
pub struct UpsertTask {
    pub edit_target: Option<Uuid>,
    pub title: String,
    pub owner: Uuid,
    pub description: Option<String>,
}

pub async fn get_by_id(db: &DatabaseConnection, id: &Uuid) -> Option<tasks::Model> {
    Tasks::find_by_id(*id)
        .one(db)
        .await
        .inspect_err(|err| log::error!("Failed to find user by id {:?}", err))
        .ok()
        .flatten()
}

pub async fn list_for_user(
    db: &DatabaseConnection,
    user: &users::Model,
    num_page: Option<u16>,
    page_size: Option<u16>,
) -> Result<Vec<tasks::Model>, DbErr> {
    Tasks::find()
        .filter(tasks::Column::OwnerId.eq(user.id))
        .order_by(tasks::Column::CreatedAt, Order::Desc)
        .paginate(db, u64::from(page_size.unwrap_or(50)))
        .fetch_page(u64::from(num_page.unwrap_or(0)))
        .await
}

pub async fn delete_task(task_id: Uuid, db: &DatabaseConnection) -> Option<bool> {
    Tasks::delete_by_id(task_id)
        .exec(db)
        .await
        .inspect_err(|err| log::error!("failed to delete task {:?}", err))
        .map(|res| res.rows_affected == 1)
        .ok()
}

pub async fn upsert_task(
    tasks: UpsertTask,
    db: &DatabaseConnection,
) -> Result<tasks::Model, DbErr> {
    let mut entity = tasks::ActiveModel {
        title: ActiveValue::Set(tasks.title),
        owner_id: ActiveValue::Set(tasks.owner),
        ..Default::default()
    };
    if let Some(id) = tasks.edit_target {
        entity.id = ActiveValue::set(id);
    }
    if let Some(desc) = tasks.description {
        entity.description = ActiveValue::Set(desc);
    }
    Tasks::insert(entity)
        .on_conflict(
            OnConflict::column(tasks::Column::Id)
                .update_columns([tasks::Column::Description, tasks::Column::Title])
                .to_owned(),
        )
        .exec_with_returning(db)
        .await
}
