use crate::guilds::*;
use crate::users::*;
use crate::channels::*;
use crate::members::*;
use crate::messages::*;
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

#[post("/")]
/// POST `/api/v1/message/`
async fn private_send_message() -> status::Custom<&'static str> { send_message().await }

#[get("/<id>")]
/// GET `/api/v1/message/<id>`
async fn private_get_message(id: u64) -> status::Custom<&'static str> { get_message(id).await }

#[post("/")]
/// POST `/api/v1/member/`
async fn private_create_member() -> status::Custom<&'static str> { create_member().await }

#[get("/<id>")]
/// GET `/api/v1/member/<id>`
async fn private_get_member(id: u64) -> status::Custom<&'static str> { get_member(id).await }


pub async fn entrypoint() {
    load_db().await;

    rocket::build()
        .mount("/api/v1/guilds", routes![private_create_guild, private_get_guild])
        .mount("/api/v1/user", routes![private_create_user, private_get_user])
        .mount("/api/v1/channels", routes![private_create_channel, private_get_channel])
        .mount("/api/v1/message", routes![private_send_message, private_get_message])
        .mount("/api/v1/member", routes![private_create_member, private_get_member])
        .launch()
        .await
        .expect("failed to launch rocket!")
}
