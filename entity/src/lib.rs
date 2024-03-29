mod generated;
mod state;
pub mod tasks;
pub mod users;

pub use generated::task::{Entity as TaskEntity, Model as Task};
pub use generated::users::{Entity as UserEntity, Model as User};
pub use state::AppState;
