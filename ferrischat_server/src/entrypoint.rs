use rocket::{Rocket, Ignite};
use crate::servers::create_guild::CREATE_GUILD_ROUTES;

pub async fn entrypoint() {
    rocket::build().mount("/api/v1/guilds", CREATE_GUILD_ROUTES).launch().await.expect("failed to launch rocket!")
}
