use jsonwebtoken::{decode, DecodingKey, Validation};

use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

use crate::AppConfig;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: u32,
    pub role: String,
    pub exp: u64,
}

pub struct AuthenticatedUser {
    pub id: u32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(token) = req.headers().get_one("token") {
            let config = req.rocket().state::<AppConfig>().unwrap();

            let data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            );

            let claims = match data {
                Ok(p) => p.claims,
                Err(_) => {
                    return Outcome::Error((Status::Unauthorized, "Invalid token".to_string()))
                }
            };
            Outcome::Success(AuthenticatedUser { id: claims.sub })
        } else {
            Outcome::Error((Status::Unauthorized, "token absent".to_string()))
        }
    }
}
