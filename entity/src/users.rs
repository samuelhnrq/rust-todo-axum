use crate::generated::users::ActiveModel as UserActiveModel;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};

pub use crate::{User, UserEntity};

#[derive(serde::Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

pub async fn list_all_users(
    db: &DatabaseConnection,
    num_page: Option<u16>,
    page_size: Option<u16>,
) -> Result<Vec<User>, DbErr> {
    return UserEntity::find()
        .paginate(db, page_size.unwrap_or(50) as u64)
        .fetch_page(num_page.unwrap_or(0) as u64)
        .await;
}

pub async fn new_user(user: NewUser, db: &DatabaseConnection) -> Result<User, DbErr> {
    let entity = UserActiveModel {
        name: ActiveValue::Set(user.name),
        email: ActiveValue::Set(user.email),
        ..Default::default()
    };
    return UserEntity::insert(entity).exec_with_returning(db).await;
}
