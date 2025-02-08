use std::{sync::Mutex, collections::HashMap};
use poem::{handler, http::StatusCode, post, web::Json, Error, Response, Result, Route};
use serde::{Serialize, Deserialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use once_cell::sync::Lazy;

pub fn route() -> Route {
  return Route::new()
    .at("/signup", post(sign_up))
}

#[handler]
async fn sign_up(user: Json<User>) -> Result<Json<serde_json::Value>> {
  println!("{:?}", user);

  let mut users = USERS.lock().unwrap();
  if users.contains_key(&user.name) {
    return Result::Err(
      Error::from_response(
        Response::builder()
          .status(StatusCode::CONFLICT)
          .header("Content-Type", "application/json")
          .body(serde_json::to_string(&ErrorResponse {
            error: "User already exists".to_string(),
            msg: "Please use different credentials".to_string()
          }).unwrap())
      )
    )
  }

  let pass = match hash(user.pass.clone(), DEFAULT_COST) {
    Ok(p) => p,
    Err(e) => return Result::Err(
      Error::from_response(
        Response::builder()
          .status(StatusCode::INTERNAL_SERVER_ERROR)
          .header("Content-Type", "application/json")
          .body(serde_json::to_string(&ErrorResponse {
            error: "Error password issue".to_string(),
            msg: "Please contact support".to_string()
          }).unwrap())
      )
    )
  };

  users.insert(user.name.to_string(), pass);
  
  Ok(Json(serde_json::json!({
    "code": 200,
    "msg": "Test"
  })))
}

async fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
  hash(password, DEFAULT_COST)
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

const SECRET_KEY: &str = "SuperSecretKey";

#[derive(Serialize, Deserialize, Debug)]
struct User {
  name: String,
  pass: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
  error: String,
  msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SignUpResponse {
  jwt: String,
}



// async fn register_user(user: &User) -> Result<String, String> {
  // unsafe {
  //   if USERS.contains_key(&user.username) {
  //     return Err("Username already taken".to_string());
  //   }

  //   let hashed_password = hash_password(&user.password).unwrap();
  //   USERS.insert(user.username.clone(), hashed_password);

  //   Ok("User registered successfully".to_string())
  // }

  // return Result::Err(Error::from_string("In progress", StatusCode::BAD_REQUEST));
// }