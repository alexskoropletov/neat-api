use serde::{Serialize, Deserialize};
// use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;

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

impl warp::reject::Reject for InvalidCredentials {}

pub async fn get_token(
    auth: Auth
    ) -> Result<impl warp::Reply, warp::Rejection> {

        println!("Auth {:?}", auth);

        if auth.login != "admin" || auth.password != "admin" {
            return Ok(warp::reply::json(&String::from("error")));
            // return warp::reject::custom(InvalidCredentials);
        }

        let my_claims = Claims {
            sub: String::from("admin"),
            company: String::from("admin"),
            exp: 10000000000,
        };
        let secret = get_secret() + auth.login.as_str();

        println!("secret {:?}", secret);

        let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref())) {
            Ok(token) => token,
            Err(_) => String::from("error"),
        };


        println!("token {:?}", token);
    
        Ok(warp::reply::json(&token))
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