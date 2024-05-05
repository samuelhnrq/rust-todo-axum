use entity::users::NewUser;
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};

pub async fn fetch_user(
    client: &Client,
    uid: &String,
    jwt: &String,
) -> Result<ClerkUser, reqwest::Error> {
    client
        .get(format!("https://api.clerk.com/v1/users/{uid}"))
        .bearer_auth(jwt)
        .send()
        .await?
        .json::<ClerkUser>()
        .await
}

impl From<ClerkUser> for NewUser {
    fn from(value: ClerkUser) -> Self {
        NewUser {
            email: value
                .email_addresses
                .first()
                .map(|x| x.address.clone())
                .unwrap_or_default(),
            name: format!("{} {}", value.first_name, value.last_name),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ClerkUser {
    pub id: String,
    pub object: String,
    pub external_id: String,
    pub primary_email_address_id: String,
    pub primary_phone_number_id: String,
    pub primary_web3_wallet_id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_image_url: String,
    pub image_url: String,
    pub email_addresses: Vec<EmailAddress>,
    pub phone_numbers: Vec<PhoneNumber>,
    pub last_sign_in_at: i64,
    pub banned: bool,
    pub locked: bool,
    pub lockout_expires_in_seconds: i64,
    pub verification_attempts_remaining: i64,
    pub updated_at: i64,
    pub created_at: i64,
    pub last_active_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct EmailAddress {
    pub id: String,
    pub object: String,
    pub address: String,
    pub reserved: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub id: String,
    pub object: String,
    pub phone: String,
    pub reserved_for_second_factor: bool,
    pub default_second_factor: bool,
    pub reserved: bool,
    pub backup_codes: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
