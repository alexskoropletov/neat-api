use crate::common::errors::Error;
use crate::common::responses::{Result, WebResult};
use crate::common::stdout;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

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
        Err(_) => {
            stdout::warn("JWT_SECRET not set, using default", ());
            String::from("secret")
        }
    }
}

#[derive(Debug)]
pub struct InvalidCredentials;

impl reject::Reject for InvalidCredentials {}

pub async fn get_token(auth: Auth) -> WebResult<String> {
    stdout::debug("Auth", &auth);

    if auth.login != "admin" || auth.password != "admin" {
        return Err(reject::custom(Error::WrongCredentials));
    }

    let claims = Claims {
        sub: String::from("admin"),
        company: String::from("admin"),
        exp: 10000000000,
    };
    let secret = get_secret();

    stdout::debug("secret", &secret);

    let header = Header::new(Algorithm::HS512);
    let token = match encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(token) => token,
        Err(_) => return Err(reject::custom(Error::JWTTokenCreation)),
    };

    stdout::debug("token", &token);

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
            
            stdout::debug("decoded", &decoded);

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
