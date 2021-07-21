use rocket::{Rocket, Ignite};
use crate::servers::create_guild::create_guild;
use rocket::response::status;

#[post("/")]
/// POST `/api/v1/guilds/`
async fn private_create_guild() -> status::Custom<&'static str> {
    create_guild().await
}

pub async fn entrypoint() {
    rocket::build().mount("/api/v1/guilds", routes![private_create_guild]).launch().await.expect("failed to launch rocket!")
}
