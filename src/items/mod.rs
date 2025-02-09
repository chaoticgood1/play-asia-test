use poem::web::Path;
use poem::{get, post, put, Response, Result};
use poem::{handler, http::StatusCode, web::Json, Route, Error};
use serde::{Serialize, Deserialize};
use serde_json::{from_str, json, Value};
use std::fs::{read_to_string, write};

use crate::ErrorResponse;


pub fn route() -> Route {
  return Route::new()
    .at("/items", post(create_item).get(get_items))
    .at("/items/:id", get(get_item))
}

#[handler]
async fn create_item(item_req: Json<ItemReq>) -> Result<Response> {
  let data_str = match read_to_string(DATA_JSON) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error create 1".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  let mut items: Vec<Value> = match from_str(&data_str) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error create 2".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  if !contains_item(&items, &item_req.name) {
    let item = Item {
      id: get_new_last_id(&items),
      name: item_req.name.to_string()
    };

    items.push(json!(item));
    let new_items_str = match serde_json::to_string_pretty(&items) {
      Ok(res) => res,
      Err(_e) => {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error create 3".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    };

    match write(DATA_JSON, new_items_str.as_bytes()) {
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


#[handler]
async fn get_item(id: Path<u64>) -> Result<Response> {
  let data_str = match read_to_string(DATA_JSON) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error read 3".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  let mut items: Vec<Value> = match from_str(&data_str) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error read 4".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  for item in items.iter() {
    let tmp_id = match item.get("id").and_then(|id| id.as_u64()) {
      Some(res) => res,
      None =>  {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error read 5".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    };

    if tmp_id == *id {
      return Ok(response_json(StatusCode::OK, item))
    }
    println!("item {:?}", tmp_id);
  }

  Err(error_response_json(StatusCode::NOT_FOUND, ErrorResponse {
    error: "Not found".to_string(),
    msg: "Item does not exist".to_string()
  }))
}





fn contains_item(items: &Vec<Value>, name: &str) -> bool {
  for item in items.iter() {
    let n = match item.get("name").and_then(|n| n.as_str()) {
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