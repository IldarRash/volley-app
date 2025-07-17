// backend/src/api/user.rs

use actix_web::{post, web, HttpResponse, Responder};
use crate::dto::request::user::UserRegistrationRequest;

/// Registers a new user.
/// This endpoint receives a `UserRegistrationRequest` and is expected to handle
/// user creation, password encoding, and database persistence.
///
/// # Arguments
/// * `user_request` - A `web::Json` containing the user registration data.
///
/// # Returns
/// * A `Responder` that signifies the result of the operation.
#[post("/register")]
pub async fn register(user_request: web::Json<UserRegistrationRequest>) -> impl Responder {
    // TODO: Implement user registration logic:
    // 1. Validate the incoming request.
    // 2. Check if a user with the same username or email already exists.
    // 3. Encode the password securely.
    // 4. Create a new `User` entity.
    // 5. Save the new user to the database.
    // 6. Generate a JWT or session token.
    // 7. Return the appropriate response (e.g., the created user view with a token).

    println!("Registering user: {:?}", user_request);
    HttpResponse::Created().finish()
} 