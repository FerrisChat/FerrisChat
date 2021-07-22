use rocket::http::Status;
use rocket::response::{content, status};

pub async fn logout_user() -> status::Custom<&'static str> {
    status::Custom(Status::Ok, "logged out user test")
}