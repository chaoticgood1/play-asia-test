use poem::{post, get, Response, Result};
use poem::{handler, http::StatusCode, web::Json, Route, Error};
use serde::{Serialize, Deserialize};
use serde_json::Value;

use std::{env, path::PathBuf};
use std::fs::File;
use std::io::Read;

use crate::ErrorResponse;


pub fn route() -> Route {
  return Route::new()
    .at("/items", post(create).get(get_items))
}

#[handler]
async fn create(item: Json<Item>) -> Result<Json<Value>> {
  println!("create()");

  let path = get_file_path();
  if path.is_none() {
    return Result::Err(
      Error::from_response(
        Response::builder()
          .status(StatusCode::CONFLICT)
          .header("Content-Type", "application/json")
          .body(serde_json::to_string(&ErrorResponse {
            error: "Path doesn't exist".to_string(),
            msg: "Please use different credentials".to_string()
          })
          .unwrap())
      )
    )
  }

  Ok(Json(serde_json::json!({
    "msg": "Successfully created item"
  })))
}


#[handler]
async fn get_items() -> Result<Json<Vec<Value>>> {
  println!("get_items()");
  let path = get_file_path();
  if path.is_none() {
    return Result::Err(
      Error::from_response(
        Response::builder()
          .status(StatusCode::CONFLICT)
          .header("Content-Type", "application/json")
          .body(serde_json::to_string(&ErrorResponse {
            error: "Path doesn't exist".to_string(),
            msg: "Please use different credentials".to_string()
          })
          .unwrap())
      )
    )
  }

  let temp_items = vec![
    serde_json::json!({"id": "1", "name": "Rust Programming Book"}),
    serde_json::json!({"id": "2", "name": "Web Development with Rust"}),
  ];


  Ok(Json(temp_items))
}

// #[handler]
// async fn get_items(item: Json<Item>) -> Result<Vec<Json<serde_json::Value>>> {
//   println!("get_items()");


//   let path = get_file_path();
//   if path.is_none() {
//     return Result::Err(
//       Error::from_response(
//         Response::builder()
//           .status(StatusCode::CONFLICT)
//           .header("Content-Type", "application/json")
//           .body(serde_json::to_string(&ErrorResponse {
//             error: "Path doesn't exist".to_string(),
//             msg: "Please use different credentials".to_string()
//           })
//           .unwrap())
//       )
//     )
//   }

//   Ok(Json(serde_json::json!({
//     "msg": "Successfully created item"
//   })))
// }


// fn get_data_json() -> Option<File> {
//   let path = match get_file_path() {
//     Some(p) => p,
//     None => return None
//   };

//   let open = File::open("data.json");
//   if open.is_ok() {
//     return Some(open.unwrap());
//   } else {
//     let create_res = File::create(path);
//     if create_res.is_ok() {
//       return Some(create_res.unwrap());
//     }
//   }

//   return None;
// }


fn get_file_path() -> Option<PathBuf> {
  let current_dir = match env::current_dir() {
    Ok(r) => r,
    Err(e) => {
      println!("{:?}", e);
      return None;
    }
  };
  return Some(current_dir.join("data.json"));
}


#[derive(Serialize, Deserialize, Debug)]
struct Item {
  name: String,
}




/*
  TODO
    CRUD for items
*/