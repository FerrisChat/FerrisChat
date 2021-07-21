use rocket::{Rocket, Ignite};

pub async fn entrypoint() {
    rocket::build().mount("/api/v1/guilds", routes![index]).launch().await.expect("failed to launch rocket!")
}
