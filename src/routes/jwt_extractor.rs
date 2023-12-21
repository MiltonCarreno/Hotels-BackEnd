use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::future::{Ready, ready};
use actix_web::{
    FromRequest, web, error::ErrorUnauthorized, HttpRequest, 
    dev::Payload, http::header::AUTHORIZATION
};

use crate::routes::utils::Claims;
use crate::database::AppState;

#[derive(Serialize, Deserialize)]
pub struct AuthorizationToken {
    pub id: usize,
}

impl FromRequest for AuthorizationToken {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers()
            .get(AUTHORIZATION);

        let auth_token = match auth_header {
            Some(header_val) => header_val.to_str()
                .unwrap_or("").to_string(),
            None => "".to_string(),
        };

        if auth_token.is_empty() {
            return ready(Err(ErrorUnauthorized("Unauthorized: No token found!")));
        }

        let secret = req.app_data::<web::Data<AppState>>().unwrap()
            .secret.to_string();

        let decoded_token = decode::<Claims>(
            &auth_token, &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default()
        );

        match decoded_token {
            Ok(token) => {
                println!("Authorized!: {:#?}", token.claims);
                ready(Ok(AuthorizationToken {id: token.claims.user_id}))
            },
            Err(e) => {
                println!("Unauthorized: {e}");
                ready(Err(ErrorUnauthorized("Unauthorized: Invalid token!")))
            }
        }
    }
}