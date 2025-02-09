use std::{collections::HashMap, sync::Mutex};
use jsonwebtoken::{encode, Header, EncodingKey};
use poem::{handler, http::StatusCode, post, web::Json, Error, Response, Result, Route};
use serde::{Serialize, Deserialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use once_cell::sync::Lazy;

use crate::{Claims, ErrorResponse, SECRET_KEY};

pub fn route() -> Route {
  return Route::new()
    .at("/signup", post(sign_up))
    .at("/login", post(login))
}

#[handler]
async fn sign_up(sign_up: Json<User>) -> Result<Json<serde_json::Value>> {
  let mut users = USERS.lock().unwrap();
  if users.contains_key(&sign_up.name) {
    return Result::Err(
      Error::from_response(
        Response::builder()
          .status(StatusCode::CONFLICT)
          .header("Content-Type", "application/json")
          .body(serde_json::to_string(&ErrorResponse {
            error: "User already exists".to_string(),
            msg: "Please use different credentials".to_string()
          })
          .unwrap())
      )
    )
  }

  let hashed_pass = match hash(sign_up.pass.clone(), DEFAULT_COST) {
    Ok(p) => p,
    Err(_e) => return Result::Err(
      Error::from_response(
        Response::builder()
          .status(StatusCode::INTERNAL_SERVER_ERROR)
          .header("Content-Type", "application/json")
          .body(serde_json::to_string(&ErrorResponse {
            error: "Error password issue".to_string(),
            msg: "Please contact support".to_string()
          })
          .unwrap())
      )
    )
  };

  users.insert(sign_up.name.to_string(), hashed_pass);
  
  Ok(Json(serde_json::json!({
    "msg": "Successfully signed up"
  })))
}

#[handler]
async fn login(login: Json<User>) -> Result<Json<serde_json::Value>> {
  let mut users = USERS.lock().unwrap();

  let hashed_pass = match users.get(&login.name) {
    Some(u) => u,
    None => return Result::Err(
      Error::from_response(
        Response::builder()
          .status(StatusCode::UNAUTHORIZED)
          .header("Content-Type", "application/json")
          .body(serde_json::to_string(&ErrorResponse {
            error: "User not found".to_string(),
            msg: "The user doesn't exist".to_string()
          })
          .unwrap())
      )
    )
  };


  match verify(login.pass.clone(), &hashed_pass) {
    Ok(res) => res,
    Err(_e) => return Result::Err(
      Error::from_response(
        Response::builder()
          .status(StatusCode::UNAUTHORIZED)
          .header("Content-Type", "application/json")
          .body(serde_json::to_string(&ErrorResponse {
            error: "Invalid credentials".to_string(),
            msg: "Wrong username or password".to_string()
          })
          .unwrap())
      )
    )
  };

  Ok(Json(serde_json::json!({
    "msg": "Successfully logged in",
    "token": create_jwt(&login.name)
  })))
}

fn create_jwt(name: &str) -> String {
  let claims = Claims {
    sub: name.to_string(),
    exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
  };

  encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY.as_ref())).unwrap()
}


static USERS: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
  let mut users = HashMap::new();
  users.insert("user1".to_string(), "pass1".to_string());
  users.insert("user2".to_string(), "pass2".to_string());

  let mut mod_users = HashMap::new();
  for (name, pass) in users.iter() {
    let p = match hash(pass.clone(), DEFAULT_COST) {
      Ok(p) => p,
      Err(e) =>  {
        println!("Error hashing password {:?}", e);
        "Default".to_string()
      }
    };
    mod_users.insert(name.to_string(), p);
  }

  Mutex::new(mod_users)
});

#[derive(Serialize, Deserialize, Debug)]
struct User {
  name: String,
  pass: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SignUpResponse {
  jwt: String,
}


/*
  TODO:
    Sign in
    Log in
    Log out
    Login
      user: Admin
      pass: Admin
    Logout

    Verify all of these
    Create tests
      Unit
      Component (Might not needed)
      End to end? (Priority, since this is just a small app)

*/