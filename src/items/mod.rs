use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use poem::http::Method;
use poem::web::{Data, Path};
use poem::{get, post, Endpoint, EndpointExt, IntoResponse, Middleware, Request, Response, Result};
use poem::{handler, http::StatusCode, web::Json, Route};
use serde::{Serialize, Deserialize};
use serde_json::{from_str, json, Value};
use std::fs::{read_to_string, write, OpenOptions};
use std::io::Write;

use crate::{error_response_json, response_json, Claims, ErrorResponse, SECRET_KEY};

pub fn route(data_path: String) -> Route {
  let route = Route::new()
    .at("/items", post(post_item)
      .get(get_items)
      .with(AuthMiddleware::new(data_path.clone()))
    )
    .at("/items/:id", get(get_item)
      .put(put_item)
      .delete(delete_item)
      .with(AuthMiddleware::new(data_path.clone()))
    );

  return route;
}

#[handler]
async fn post_item(item_req: Json<ItemReq>, data_path: Data<&String>) -> Result<Response> {
  let data_str = match read_to_string(data_path.as_str()) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error post_item 2".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  let mut items: Vec<Value> = match from_str(&data_str) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error post_item 3".to_string(),
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
          error: "Server error post_item 4".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    };

    match write(data_path.as_str(), new_items_str.as_bytes()) {
      Ok(res) => res,
      Err(_e) => {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error post_item 5".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    }

    return Ok(response_json(StatusCode::CREATED, &item));
  }
  
  Err(error_response_json(StatusCode::BAD_REQUEST, ErrorResponse {
    error: "Server error post_item 6".to_string(),
    msg: "Item already exists".to_string()
  }))
}

#[handler]
async fn get_items(data_path: Data<&String>) -> Result<Response> {
  let data_str = match read_to_string(data_path.as_str()) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error get_items 2".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  let mut data: Vec<Value> = match from_str(&data_str) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error get_items 3".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };
  // println!("data {:?}", data);

  Ok(response_json(StatusCode::OK, &data))
}

#[handler]
async fn get_item(id: Path<u64>, data_path: Data<&String>) -> Result<Response> {
  let data_str = match read_to_string(data_path.as_str()) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error get_item 1".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  let mut items: Vec<Value> = match from_str(&data_str) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error get_item 2".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  for item in items.iter() {
    let tmp_id = match item.get("id").and_then(|id| id.as_u64()) {
      Some(res) => res,
      None =>  {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error get_item 3".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    };

    if tmp_id == *id {
      return Ok(response_json(StatusCode::OK, item))
    }
  }

  Err(error_response_json(StatusCode::NOT_FOUND, ErrorResponse {
    error: "Not found".to_string(),
    msg: "Item does not exist".to_string()
  }))
}

#[handler]
async fn put_item(id: Path<u64>, item_req: Json<ItemReq>, data_path: Data<&String>) -> Result<Response> {
  let data_str = match read_to_string(data_path.as_str()) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error put_item 1".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  let mut items: Vec<Value> = match from_str(&data_str) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error put_item 2".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };
  
  let mut updated_item_op = None;
  for index in 0..items.iter().len() {
    let mut item = match items.get_mut(index) {
      Some(res) => res,
      None => {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error put_item 3".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    };

    let tmp_id = match item.get("id").and_then(|id| id.as_u64()) {
      Some(res) => res,
      None =>  {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error put_item 4".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    };

    if tmp_id == *id {
      let mut update_item = match item.get_mut("name") {
        Some(res) => res,
        None => {
          return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
            error: "Server error put_item 6".to_string(),
            msg: "Please contact support".to_string()
          }))
        }
      };

      let name: &str = match update_item.as_str() {
        Some(res) => res,
        None => {
          return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
            error: "Server error post_item 4".to_string(),
            msg: "Please contact support".to_string()
          }))
        }
      };

      if name == item_req.name {
        // NOTE: ChatGPT said this should be StatusCode::NO_CONTENT or OK
        //        I chose OK so that there is a response body
        return Ok(response_json(StatusCode::OK, item))
      } else {
        *update_item = json!(item_req.name);
        updated_item_op = Some(update_item.clone());
      }

      
      break;
    }
  }

  if updated_item_op.is_some() {
    let new_items_str = match serde_json::to_string_pretty(&items) {
      Ok(res) => res,
      Err(_e) => {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error post_item 5".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    };

    match write(data_path.as_str(), new_items_str.as_bytes()) {
      Ok(res) => res,
      Err(_e) => {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error post_item 6".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    }

    let update_item = updated_item_op.unwrap();
    return Ok(response_json(StatusCode::OK, update_item))
  }
  
  Err(error_response_json(StatusCode::BAD_REQUEST, ErrorResponse {
    error: "Server error post_item 7".to_string(),
    msg: "Item already exists".to_string()
  }))
}

#[handler]
async fn delete_item(id: Path<u64>, data_path: Data<&String>) -> Result<Response> {
  let data_str = match read_to_string(data_path.as_str()) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error delete_item 1".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };

  let mut items: Vec<Value> = match from_str(&data_str) {
    Ok(res) => res,
    Err(_) => {
      return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
        error: "Server error delete_item 2".to_string(),
        msg: "Please contact support".to_string()
      }))
    }
  };
  
  let mut has_deleted = false;
  for index in 0..items.iter().len() {
    let item = match items.get(index) {
      Some(res) => res,
      None => {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error delete_item 3".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    };

    let tmp_id = match item.get("id").and_then(|id| id.as_u64()) {
      Some(res) => res,
      None =>  {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error delete_item 4".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    };

    if tmp_id == *id {
      items.remove(index);
      has_deleted = true;
      break;
    }
  }

  if has_deleted {
    let new_items_str = match serde_json::to_string_pretty(&items) {
      Ok(res) => res,
      Err(_e) => {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error delete_item 5".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    };

    match write(data_path.as_str(), new_items_str.as_bytes()) {
      Ok(res) => res,
      Err(_e) => {
        return Err(error_response_json(StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse {
          error: "Server error delete_item 6".to_string(),
          msg: "Please contact support".to_string()
        }))
      }
    }

    return Ok(response_json(StatusCode::OK, ItemDeleted {
      message: "Item deleted successfully".to_string()
    }))
  }
  
  Err(error_response_json(StatusCode::BAD_REQUEST, ErrorResponse {
    error: "Server error delete_item 7".to_string(),
    msg: "Item doesn't exist".to_string()
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


#[derive(Serialize, Deserialize, Debug)]
struct ItemReq {
  name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
  pub id: u64,
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemDeleted {
  message: String,
}



struct AuthMiddleware {
  data_path: String
}

impl AuthMiddleware {
  pub fn new(data_path: String) -> Self {
    Self { data_path }
  }
}

impl<E: Endpoint> Middleware<E> for AuthMiddleware {
  type Output = AuthMiddlewareImpl<E>;

  fn transform(&self, ep: E) -> Self::Output {
    AuthMiddlewareImpl {
      inner: ep,
      data_path: self.data_path.clone()
    }
  }
}

struct AuthMiddlewareImpl<E> {
  inner: E,
  data_path: String,
}

impl<E: Endpoint> Endpoint for AuthMiddlewareImpl<E> {
  type Output = Response;

  async fn call(&self, req: Request) -> Result<Self::Output> {
    if req.method() == Method::GET {
      let res = self.inner.call(req).await;

      match res {
        Ok(resp) => {
          let resp = resp.into_response();
          return Ok(resp)
        }
        Err(err) => {
          println!("error: {err}");
          return Err(err)
        }
      }
    }

    if let Some(auth_header) = req.headers().get("Authorization") {
      if let Ok(auth_str) = auth_header.to_str() {
        if auth_str.starts_with("Bearer ") {
          let token = &auth_str[7..];

          let key = DecodingKey::from_secret(SECRET_KEY.as_ref());
          let mut validation = Validation::new(Algorithm::HS256);
          validation.validate_exp = true; // Check expiration

          if let Ok(_claims) = decode::<Claims>(token, &key, &validation) {
            // println!("claims {:?}", claims);
            let res = self.inner.call(req).await;

            match res {
              Ok(resp) => {
                let resp = resp.into_response();
                return Ok(resp)
              }
              Err(err) => {
                // println!("err {:?}", err);
                return Err(err.status().into())
              }
            }
          }
        }
      }
    }

    // println!("Middleware - Data Path: {:?}", self.data_path);

    if let Err(_) = ensure_file_exists(&self.data_path) {
      // NOTE: Should be addressed internally, can't expose error outside
      return Err(error_response_json(
        StatusCode::INTERNAL_SERVER_ERROR,
        ErrorResponse {
            error: "Server error post_item 1".to_string(),
            msg: "Please contact support".to_string(),
        },
      ));
    }

    // println!("Unauthorized");
    Err(StatusCode::UNAUTHORIZED.into())
  }
}


fn ensure_file_exists(path: &str) -> std::io::Result<()> {
  if !std::path::Path::new(path).exists() {
    let mut file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(path)?;
    file.write(b"[]")?;
  }
  Ok(())
}
