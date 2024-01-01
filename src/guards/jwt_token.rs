use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::etc::json_web_token::{Claims, JwtService};

pub struct JwtToken(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the Authorization header from the request
        if let Some(auth) = request.headers().get_one("Authorization") {
            let token = auth.replace("Bearer ", ""); // Remove "Bearer " prefix

            // Decode and verify the token
            match JwtService::verify_token(&token) {
                Ok(claims) => {
                    // Token is valid, return JwtToken with decoded claims
                    Outcome::Success(JwtToken(claims))
                }
                Err(_) => {
                    // Token is invalid, return a 401 Unauthorized response
                    Outcome::Error((Status::Unauthorized, ()))
                }
            }
        } else {
            // No Authorization header provided, return a 401 Unauthorized response
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}
