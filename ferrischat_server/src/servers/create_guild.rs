use rocket::http::Status;
use rocket::response::{content, status};

#[post("/")]
async fn create_guild() -> status::Custom<&'static str> {
    status::Custom(Status::Created, "created guild test")
}