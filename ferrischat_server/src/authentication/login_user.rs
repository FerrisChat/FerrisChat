use rocket::http::Status;
use rocket::response::{content, status};

pub async fn login_user() -> status::Custom<&'static str> {
    status::Custom(Status::Ok, "logged in user test")
}