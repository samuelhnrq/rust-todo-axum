use crate::generated::users;
use sea_orm::{
    sea_query::OnConflict, ActiveValue, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
};

pub use crate::{User, UserEntity};

#[derive(serde::Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

pub async fn list_all(
    db: &DatabaseConnection,
    num_page: Option<u16>,
    page_size: Option<u16>,
) -> Result<Vec<User>, DbErr> {
    UserEntity::find()
        .paginate(db, u64::from(page_size.unwrap_or(50)))
        .fetch_page(u64::from(num_page.unwrap_or(0)))
        .await
}

pub async fn new_user(user: NewUser, db: &DatabaseConnection) -> Result<User, DbErr> {
    let entity = users::ActiveModel {
        name: ActiveValue::Set(user.name),
        email: ActiveValue::Set(user.email),
        ..Default::default()
    };
    UserEntity::insert(entity).exec_with_returning(db).await
}

pub async fn upsert(user: NewUser, db: &DatabaseConnection) -> Result<User, DbErr> {
    let entity = users::ActiveModel {
        name: ActiveValue::Set(user.name),
        email: ActiveValue::Set(user.email),
        ..Default::default()
    };
    UserEntity::insert(entity)
        .on_conflict(
            OnConflict::column(users::Column::Id)
                .update_columns([users::Column::Email])
                .to_owned(),
        )
        .exec_with_returning(db)
        .await
}
