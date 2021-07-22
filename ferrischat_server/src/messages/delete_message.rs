use rocket::http::Status;
use rocket::response::{content, status};

pub async fn delete_message() -> status::Custom<&'static str> {
    status::Custom(Status::Ok, "deleted message test")
}