use rocket::http::Status;
use rocket::response::{content, status};

pub async fn delete_channel() -> status::Custom<&'static str> {
    status::Custom(Status::Ok, "deleted channel test")
}