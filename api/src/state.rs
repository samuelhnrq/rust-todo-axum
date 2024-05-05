use entity::HyperTarot;
use utils::authentication::fetch_remote_jwk;

use crate::adapters::database;

pub async fn create() -> HyperTarot {
    let connection = database::connect().await;
    let jwk = fetch_remote_jwk().await;
    HyperTarot { connection, jwk }
}
