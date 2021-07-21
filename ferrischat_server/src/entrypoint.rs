use crate::servers::*;
use rocket::response::status;
use rocket::{Ignite, Rocket};
use ferrischat_db::load_db;

#[post("/")]
/// POST `/api/v1/guilds/`
async fn private_create_guild() -> status::Custom<&'static str> {
    create_guild().await
}

#[get("/<id>")]
/// GET `/api/v1/guilds/<id>`
async fn private_get_guild(id: u64) -> status::Custom<&'static str> {
    get_guild(id).await
}


pub async fn entrypoint() {
    load_db().await;

    rocket::build()
        .mount("/api/v1/guilds", routes![private_create_guild, private_get_guild])
        .launch()
        .await
        .expect("failed to launch rocket!")
}
