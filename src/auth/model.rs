use crate::common::errors::Error;
use crate::common::responses::{Result, WebResult};
use crate::common::stdout;
use crate::data_store;
use chrono::{Duration, Utc};
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
    name: String,
    login: String,
    email: String,
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

pub async fn get_token(store: data_store::Store, auth: Auth) -> WebResult<String> {
    stdout::debug("Auth", &auth);

    let found = match store.users.read().values().find(|u| u.login == auth.login) {
        Some(user) => {
            if user.password != auth.password {
                return Err(reject::custom(Error::WrongCredentials));
            }

            user
        }
        None => return Err(reject::custom(Error::WrongCredentials)),
    }
    .clone();

    let claims = Claims {
        sub: found.id,
        name: found.name,
        login: found.login,
        email: found.email,
        exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
    };

    let secret = get_secret();

    stdout::debug("secret", &secret);

    let header = Header::new(Algorithm::HS512);
    let token = match encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(token) => token,
        Err(_) => return Err(reject::custom(Error::JWTTokenCreation)),
    };

    stdout::debug("token", &token);

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
            .map_err(|err| {
                stdout::error("Authorize with JWT token error", &err);
                reject::custom(Error::JWTToken)
            })?;

            stdout::debug("decoded", &decoded);

            Ok(decoded.claims.sub)
        }
        Err(_) => Err(reject::custom(Error::NoPermission)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(AUTHORIZATION) {
        Some(value) => value,
        None => return Err(Error::NoAuthHeader),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(value) => value,
        Err(_) => return Err(Error::NoAuthHeader),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeader);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
