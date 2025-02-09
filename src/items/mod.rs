use poem::{post, get, Response, Result};
use poem::{handler, http::StatusCode, web::Json, Route, Error};
use serde::{Serialize, Deserialize};
use serde_json::{from_str, json, Value};
use std::fs::{read_to_string, write};

use crate::ErrorResponse;


pub fn route() -> Route {
  return Route::new()
    .at("/items", post(create).get(get_items))
}

#[handler]
async fn create(item_req: Json<ItemReq>) -> Result<Response> {
  let data_str = match read_to_string(DATA_JSON) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error create 1".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  let mut data: Vec<Value> = match from_str(&data_str) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error create 2".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  if !contains_item(&data, &item_req.name) {
    let item = Item {
      id: get_new_last_id(&data),
      name: item_req.name.to_string()
    };

    data.push(json!(item));
    let new_data_str = match serde_json::to_string_pretty(&data) {
      Ok(res) => res,
      Err(_e) => {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error create 3".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    };

    match write(DATA_JSON, new_data_str.as_bytes()) {
      Ok(res) => res,
      Err(_e) => {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error create 4".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    }

    return Ok(response_json(StatusCode::CREATED, &item));
  }
  
  Err(error_response_json(StatusCode::BAD_REQUEST, ErrorResponse {
    error: "Server error create 5".to_string(),
    msg: "Item already exists".to_string()
  }))
}

#[handler]
async fn get_items() -> Result<Response> {
  println!("get_items()");

  let data_str = match read_to_string(DATA_JSON) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error read 1".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  let mut data: Vec<Value> = match from_str(&data_str) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error read 2".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  Ok(response_json(StatusCode::OK, &data))
}

fn contains_item(data: &Vec<Value>, name: &str) -> bool {
  for data in data.iter() {
    let n = match data.get("name").and_then(|n| n.as_str()) {
      Some(res) => res,
      None => return false,
    };

    if n == name {
      return true
    }
    // println!("item {:?}", name);
  }
  return false;
}

fn get_new_last_id(data: &Vec<Value>) -> u64 {
  let mut highest_id = 1;
  for data in data.iter() {
    let n = match data.get("id").and_then(|n| n.as_u64()) {
      Some(res) => res,
      None => 1,
    };

    if n >= highest_id {
      highest_id = n + 1;
    }
  }
  highest_id
}


fn response_json<T>(status_code: StatusCode, data: T) -> Response
where
  T: Serialize
{
  Response::builder()
    .status(status_code)
    .header("Content-Type", "application/json")
    .body(serde_json::to_string(&data)
    .unwrap())
}

fn error_response_json<T>(status_code: StatusCode, data: T) -> Error
where
  T: Serialize
{
  Error::from_response(
    Response::builder()
      .status(status_code)
      .header("Content-Type", "application/json")
        .body(serde_json::to_string(&data).unwrap()
      )
  )
}


static DATA_JSON: &str = "data.json";


#[derive(Serialize, Deserialize, Debug)]
struct ItemReq {
  name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
  id: u64,
  name: String,
}





/*
  TODO
    CRUD for items
*/