use crate::guilds::*;
use crate::users::*;
use crate::channels::*;
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

#[post("/")]
/// POST `/api/v1/users/`
async fn private_create_user() -> status::Custom<&'static str> { create_user().await }

#[get("/<id>")]
/// GET `/api/v1/users/<id>`
async fn private_get_user(id: u64) -> status::Custom<&'static str> { get_user(id).await }

#[post("/")]
/// POST `/api/v1/channels/`
async fn private_create_channel() -> status::Custom<&'static str> { create_channel().await }

#[get("/<id>")]
/// GET `/api/v1/channels/<id>`
async fn private_get_channel(id: u64) -> status::Custom<&'static str> { get_channel(id).await }


pub async fn entrypoint() {
    load_db().await;

    rocket::build()
        .mount("/api/v1/guilds", routes![private_create_guild, private_get_guild])
        .mount("/api/v1/user", routes![private_create_user, private_get_user])
        .mount("/api/v1/channels", routes![private_create_channel, private_get_channel])
        .launch()
        .await
        .expect("failed to launch rocket!")
}
