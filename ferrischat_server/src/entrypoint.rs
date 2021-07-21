use rocket::{Rocket, Ignite};
use crate::servers::create_guild;

pub async fn entrypoint() {
    rocket::build().mount("/api/v1/guilds", routes![create_guild]).launch().await.expect("failed to launch rocket!")
}
