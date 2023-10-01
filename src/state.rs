use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

#[derive(Clone)]
pub struct AppState {
    pub conn: Pool<ConnectionManager<PgConnection>>,
}
