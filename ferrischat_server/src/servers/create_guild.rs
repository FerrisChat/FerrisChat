use rocket::http::Status;
use rocket::response::{content, status};

#[post("/")]
pub async fn create_guild() -> status::Custom<&'static str> {
    status::Custom(Status::Created, "created guild test")
}

pub const CREATE_GUILD_ROUTES: _ = routes![create_guild];