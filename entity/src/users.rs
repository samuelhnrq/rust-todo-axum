use crate::generated::{prelude::Users, users};
use sea_orm::{
    sea_query::OnConflict, ActiveValue, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
};

#[derive(serde::Deserialize)]
pub struct NewUser {
    pub name: String,
    pub oauth_sub: String,
    pub email: String,
}

pub async fn list_all(
    db: &DatabaseConnection,
    num_page: Option<u16>,
    page_size: Option<u16>,
) -> Result<Vec<users::Model>, DbErr> {
    Users::find()
        .paginate(db, u64::from(page_size.unwrap_or(50)))
        .fetch_page(u64::from(num_page.unwrap_or(0)))
        .await
}

pub async fn new_user(user: NewUser, db: &DatabaseConnection) -> Result<users::Model, DbErr> {
    let entity = users::ActiveModel {
        name: ActiveValue::Set(user.name),
        email: ActiveValue::Set(user.email),
        oauth_sub: ActiveValue::Set(user.oauth_sub),
        ..Default::default()
    };
    Users::insert(entity).exec_with_returning(db).await
}

pub async fn upsert(user: NewUser, db: &DatabaseConnection) -> Result<users::Model, DbErr> {
    let entity = users::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(user.name),
        email: ActiveValue::Set(user.email),
        oauth_sub: ActiveValue::Set(user.oauth_sub),
        ..Default::default()
    };
    Users::insert(entity)
        .on_conflict(
            OnConflict::column(users::Column::Email)
                .update_columns([users::Column::Name, users::Column::OauthSub])
                .to_owned(),
        )
        .exec_with_returning(db)
        .await
}
