use std::{fs::{remove_file, OpenOptions}, io::Write};
use play_asia::{all_routes, items::Item, users::LoginResponse};
use poem::{http::{header, StatusCode}, test::TestClient};
use serde_json::{json, to_string_pretty};


#[tokio::test]
async fn test_post_item_no_jwt() {
  let data_path = "test_post_item_no_jwt.json".to_string();
  delete_file_if_exists(&data_path);

  let items: Vec<Item> = vec![];
  create_data(data_path.clone(), &items);

  let routes = all_routes(data_path.clone());
  let client = TestClient::new(routes);
  let res = client
    .post("/items")
    .body_json(&json!({
      "name": "NewItem"
    }))
    .send()
    .await;

  res.assert_status(StatusCode::UNAUTHORIZED);

  delete_file_if_exists(&data_path);
}



#[tokio::test]
async fn test_post_item_with_jwt() {
  // Login user: admin1 ps: admin1
  let data_path = "test_post_item_with_jwt.json".to_string();
  delete_file_if_exists(&data_path);
  let routes = all_routes(data_path.clone());
  let client = TestClient::new(routes);

  let mut login_req = client
    .post("/users/login")
    .body_json(&json!({
      "name": "admin1",
      "pass": "admin1"
    }))
    .send()
    .await;

  let login_body = login_req.0.take_body().into_json::<LoginResponse>().await.unwrap();

  let items: Vec<Item> = vec![];
  create_data(data_path.clone(), &items);

  let res = client
    .post("/items")
    .header(header::AUTHORIZATION, format!("Bearer {}", login_body.token))
    .body_json(&json!({
      "name": "NewItem"
    }))
    .send()
    .await;
  

  res.assert_status(StatusCode::CREATED);
  res.assert_json(json!(
    {
      "id": 1,
      "name": "NewItem"
    }
  )).await;
  delete_file_if_exists(&data_path);
}



// FIXME: Unauthorized issue due to jwt
#[tokio::test]
async fn test_post_item_one_item() {
  let data_path = "test_post_item_one_item.json".to_string();
  delete_file_if_exists(&data_path);

  let items: Vec<Item> = vec![];
  create_data(data_path.clone(), &items);

  println!("1");

  let routes = all_routes(data_path.clone());
  let client = TestClient::new(routes);
  println!("2");
  let res = client
    .post("/items")
    .body_json(&json!({
      "name": "NewItem"
    }))
    .send()
    .await;

  println!("3");
  res.assert_status(StatusCode::OK);

  res.assert_json(json!(vec![
    Item { id: 1, name: "NewItem".to_string() }
  ])).await;

  delete_file_if_exists(&data_path);
}



#[tokio::test]
async fn test_get_items_no_item() {
  let data_path = "test_get_items_no_item.json".to_string();
  delete_file_if_exists(&data_path);

  let items: Vec<Item> = vec![];
  create_data(data_path.clone(), &items);

  let routes = all_routes(data_path.clone());
  let client = TestClient::new(routes);
  let res = client.get("/items").send().await;
  res.assert_status(StatusCode::OK);
  res.assert_json(json!(items)).await;

  delete_file_if_exists(&data_path);
}

#[tokio::test]
async fn test_get_items_with_items() {
  let data_path = "test_get_items_with_items.json".to_string();
  delete_file_if_exists(&data_path);

  let items: Vec<Item> = vec![
    Item { id: 1, name: "Item1".to_string() },
    Item { id: 2, name: "Item2".to_string() },
  ];

  create_data(data_path.clone(), &items);

  let routes = all_routes(data_path.clone());
  let client = TestClient::new(routes);
  let res = client.get("/items").send().await;
  res.assert_status(StatusCode::OK);
  res.assert_json(json!(items)).await;

  delete_file_if_exists(&data_path);
}




fn create_data<T: serde::Serialize>(data_path: String, data: &T) {
  let json_str = to_string_pretty(data).expect("Unable to convert to string");

  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .open(data_path)
    .expect("Unable to create and open file");
  file.write_all(json_str.as_bytes()).expect("Error writing to a file");
}

fn delete_file_if_exists(path: &str) {
  if std::path::Path::new(path).exists() {
    remove_file(path).unwrap();
  }
}