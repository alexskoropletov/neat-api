use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
// use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};
use crate::errors::{WebResult, Error, Result};

const BEARER: &str = "Bearer ";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
    exp: usize,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Auth {
    login: String,
    password: String,
}

fn get_secret() -> String {
    match env::var("JWT_SECRET") {
        Ok(val) => val,
        Err(_) => String::from("secret"),
    }
}

#[derive(Debug)]
pub struct InvalidCredentials;

impl reject::Reject for InvalidCredentials {}

pub async fn get_token(
    auth: Auth
    ) -> WebResult<String> {

        println!("Auth {:?}", auth);

        if auth.login != "admin" || auth.password != "admin" {
            return Err(reject::custom(Error::WrongCredentials));
        }

        let claims = Claims {
            sub: String::from("admin"),
            company: String::from("admin"),
            exp: 10000000000,
        };
        let secret = get_secret();

        println!("secret {:?}", secret);

        let header = Header::new(Algorithm::HS512);
        let token = match encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())) {
            Ok(token) => token,
            Err(_) => return Err(reject::custom(Error::JWTTokenCreation)),
        };


        println!("token {:?}", token);
    
        // Ok(warp::reply::json(&token))
        Ok(token)
}

pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| headers)
        .and_then(authorize)
}

async fn authorize(headers: HeaderMap<HeaderValue>) -> WebResult<String> {
    let secret = get_secret();

    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(Error::JWTToken))?;
            println!("decoded {:?}", decoded);

            Ok(decoded.claims.sub)
        }
        Err(_) => Err(reject::custom(Error::NoPermission)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(Error::NoAuthHeader),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(Error::NoAuthHeader),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeader);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}

// pub fn get_claims(token : String) -> Result<(), ()> {
//     let secret = get_secret();
//     let token = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());
//     match token {
//         Ok(token) => {
//             println!("{:?}", token.claims);
//             Some(())
//         },
//         Err(_) => None,
//     };

//     Ok(())
// }