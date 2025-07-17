use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome;
use rocket::http::Status;
use crate::application::use_cases::auth::{verify_token, Claims};

#[derive(Debug)]
pub struct AuthUser(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];
                match verify_token(token) {
                    Ok(claims) => Outcome::Success(AuthUser(claims)),
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            } else {
                Outcome::Error((Status::Unauthorized, ()))
            }
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

#[derive(Debug)]
pub struct AdminUser(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_user = match AuthUser::from_request(request).await {
            Outcome::Success(user) => user,
            _ => return Outcome::Error((Status::Unauthorized, ())),
        };

        if auth_user.0.role == "admin" {
            Outcome::Success(AdminUser(auth_user.0))
        } else {
            Outcome::Error((Status::Forbidden, ()))
        }
    }
} 