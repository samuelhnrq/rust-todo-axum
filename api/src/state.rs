use entity::HyperTarot;
use reqwest::Client;
use utils::authentication::{core_client_factory, fetch_remote_jwk};

use crate::adapters::database;

pub async fn create() -> HyperTarot {
    let connection = database::connect().await;
    let jwk = fetch_remote_jwk().await;
    HyperTarot {
        connection,
        jwk,
        auth_client: core_client_factory().await,
        requests: Client::new(),
    }
}
