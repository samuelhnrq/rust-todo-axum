use std::{
  error::Error,
  num::NonZeroUsize,
  sync::{LazyLock, RwLock},
};

use crate::generated::{prelude::Users, users};
use lru::LruCache;
use sea_orm::{
  sea_query::OnConflict, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
  PaginatorTrait, QueryFilter,
};

type User = users::Model;

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
) -> Result<Vec<User>, DbErr> {
  Users::find()
    .paginate(db, u64::from(page_size.unwrap_or(50)))
    .fetch_page(u64::from(num_page.unwrap_or(0)))
    .await
}

static USERS_CACHE: LazyLock<RwLock<LruCache<String, User>>> =
  LazyLock::new(|| RwLock::new(LruCache::new(NonZeroUsize::new(10).unwrap())));

fn build_cache_key(oauth_sub: &String) -> String {
  format!("user_{oauth_sub}")
}

// Future notes: most places that call this have a replacement Model, replace instead of just deleting
fn put_key(cache_key: String, other: Option<User>) -> Result<(), impl Error> {
  USERS_CACHE
    .write()
    .inspect_err(|err| {
      log::error!("failed to lock cache mutex for key {cache_key} {:?}", err);
    })
    .map(|mut cache_ref| {
      if let Some(replacement) = other {
        cache_ref.put(cache_key, replacement);
      } else {
        cache_ref.pop(&cache_key);
      }
    })
}

async fn query_by_sub(db: &DatabaseConnection, sub: &String) -> Option<User> {
  Users::find()
    .filter(users::Column::OauthSub.eq(sub))
    .one(db)
    .await
    .inspect_err(|err| log::info!("Failed to query user from DB {:?}", err))
    .ok()
    .flatten()
}

pub async fn find_by_sub(db: &DatabaseConnection, sub: &String) -> Option<User> {
  let key = build_cache_key(sub);
  {
    let mut cache = USERS_CACHE.write().ok()?;
    let cached = cache.get(&key);
    if let Some(cached_user) = cached {
      log::debug!("User cache hit {sub}");
      return Some(cached_user.clone());
    }
    log::debug!("User cache miss {sub}");
  }
  if let Some(db_user) = query_by_sub(db, sub).await {
    log::debug!("Fetched user successfully, caching {sub}");
    USERS_CACHE.write().ok()?.put(key, db_user.clone());
    Some(db_user)
  } else {
    None
  }
}

pub async fn new_user(user: NewUser, db: &DatabaseConnection) -> Result<User, DbErr> {
  let cache_key = build_cache_key(&user.oauth_sub);
  let entity = users::ActiveModel {
    name: ActiveValue::Set(user.name),
    email: ActiveValue::Set(user.email),
    oauth_sub: ActiveValue::Set(user.oauth_sub),
    ..Default::default()
  };
  let db_user = Users::insert(entity).exec_with_returning(db).await;
  db_user
    .as_ref()
    .inspect(|&inserted| {
      put_key(cache_key, Some(inserted.clone())).ok();
    })
    .ok();
  db_user
}

pub async fn upsert(user: NewUser, db: &DatabaseConnection) -> Result<User, DbErr> {
  let cache_key = build_cache_key(&user.oauth_sub);
  let entity = users::ActiveModel {
    id: ActiveValue::NotSet,
    name: ActiveValue::Set(user.name),
    email: ActiveValue::Set(user.email),
    oauth_sub: ActiveValue::Set(user.oauth_sub),
    ..Default::default()
  };
  let user = Users::insert(entity)
    .on_conflict(
      OnConflict::column(users::Column::Email)
        .update_columns([users::Column::Name, users::Column::OauthSub])
        .to_owned(),
    )
    .exec_with_returning(db)
    .await;
  user
    .as_ref()
    .inspect(|&inserted| {
      put_key(cache_key, Some(inserted.clone())).ok();
    })
    .ok();
  user
}
