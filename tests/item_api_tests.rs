use std::{fs::{read_to_string, remove_file, OpenOptions}, io::Write, time::Duration};
use play_asia::{all_routes, items::Item, users::LoginResponse};
use poem::{http::{header, StatusCode}, test::TestClient, Route};
use serde_json::{from_str, json, to_string_pretty, Value};

// POST
#[tokio::test]
async fn test_post_item_no_jwt() {
  let data_path = "test_post_item_no_jwt.json".to_string();
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
  let data_path = "test_post_item_with_jwt.json".to_string();
  let routes = all_routes(data_path.clone());
  let client = TestClient::new(routes);
  let token = get_jwt(&client).await;
  let res = client
    .post("/items")
    .header(header::AUTHORIZATION, format!("Bearer {}", token))
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

#[tokio::test]
async fn test_post_item_three_items_with_data_persistence() {
  let data_path = "test_post_item_three_items_with_data_persistence.json".to_string();
  let routes = all_routes(data_path.clone());
  let client = TestClient::new(routes);
  let token = get_jwt(&client).await;
  let res = client
    .post("/items")
    .header(header::AUTHORIZATION, format!("Bearer {}", token))
    .body_json(&json!({
      "name": "NewItem1"
    }))
    .send()
    .await;
  res.assert_status(StatusCode::CREATED);
  res.assert_json(json!(
    {
      "id": 1,
      "name": "NewItem1"
    }
  )).await;

  let res = client
    .post("/items")
    .header(header::AUTHORIZATION, format!("Bearer {}", token))
    .body_json(&json!({
      "name": "NewItem2"
    }))
    .send()
    .await;
  res.assert_status(StatusCode::CREATED);
  res.assert_json(json!(
    {
      "id": 2,
      "name": "NewItem2"
    }
  )).await;

  let res = client
    .post("/items")
    .header(header::AUTHORIZATION, format!("Bearer {}", token))
    .body_json(&json!({
      "name": "NewItem3"
    }))
    .send()
    .await;
  

  res.assert_status(StatusCode::CREATED);
  res.assert_json(json!(
    {
      "id": 3,
      "name": "NewItem3"
    }
  )).await;

  // Simulate shutting down
  drop(client);
  std::thread::sleep(Duration::from_secs(1));
  
  // Check if data persisted
  let data_str = read_to_string(data_path.as_str()).expect("Error reading data_path");
  let items: Vec<Value> = from_str(&data_str).expect("Error converting to Vec");

  let item_val1: Value = json!({
    "id": 1,
    "name": "NewItem1"
  });
  let item_val2: Value = json!({
    "id": 2,
    "name": "NewItem2"
  });
  let item_val3: Value = json!({
    "id": 3,
    "name": "NewItem3"
  });
  let items_to_check = vec![
    item_val1,
    item_val2,
    item_val3
  ];

  for item_to_check in &items_to_check {
    assert!(items.contains(item_to_check));
  }

  delete_file_if_exists(&data_path);
}


// GET
#[tokio::test]
async fn test_get_items_no_item() {
  let data_path = "test_get_items_no_item.json".to_string();
  let routes = all_routes(data_path.clone());
  let client = TestClient::new(routes);
  let res = client.get("/items").send().await;
  res.assert_status(StatusCode::OK);
  res.assert_json(json!(Vec::<Item>::new())).await;
  delete_file_if_exists(&data_path);
}

#[tokio::test]
async fn test_get_items_by_id() {
  let data_path = "test_get_items_by_id.json".to_string();
  let items: Vec<Item> = vec![
    Item { id: 1, name: "Item1".to_string() },
    Item { id: 2, name: "Item2".to_string() },
  ];

  create_data(data_path.clone(), &items);

  let routes = all_routes(data_path.clone());
  let client = TestClient::new(routes);
  let res = client.get("/items/1").send().await;
  res.assert_status(StatusCode::OK);
  res.assert_json(json!({
    "id": 1, "name": "Item1"
  })).await;

  delete_file_if_exists(&data_path);
}


#[tokio::test]
async fn test_get_items_all() {
  let data_path = "test_get_items_all.json".to_string();
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


// PUT
#[tokio::test]
async fn test_put_item_no_jwt() {
  let data_path = "test_put_item_no_jwt.json".to_string();
  let items: Vec<Item> = vec![
    Item { id: 1, name: "PutItem1".to_string() }
  ];
  create_data(data_path.clone(), &items);
  let routes = all_routes(data_path.clone());
  let client = TestClient::new(routes);
  let res = client
    .put("/items/1")
    .body_json(&json!({
      "name": "NewPutItemName1"
    }))
    .send()
    .await;
  res.assert_status(StatusCode::UNAUTHORIZED);
  delete_file_if_exists(&data_path);
}

#[tokio::test]
async fn test_put_item_with_jwt_by_id() {
  let data_path = "test_put_item_with_jwt_by_id.json".to_string();
  let items: Vec<Item> = vec![
    Item { id: 1, name: "PutItem1".to_string() }
  ];
  create_data(data_path.clone(), &items);
  let routes = all_routes(data_path.clone());
  let client = TestClient::new(routes);
  let token = get_jwt(&client).await;
  let res = client
    .put("/items/1")
    .header(header::AUTHORIZATION, format!("Bearer {}", token))
    .body_json(&json!({
      "name": "NewPutItemName1"
    }))
    .send()
    .await;
  res.assert_status(StatusCode::OK);
  res.assert_json(json!({
    "id": 1,
    "name": "NewPutItemName1"
  })).await;
  delete_file_if_exists(&data_path);
}


/* // DELETE
#[tokio::test]
async fn test_delete_item_no_jwt() {
  let data_path = "test_delete_item_no_jwt.json".to_string();

  let items: Vec<Item> = vec![
    Item { id: 1, name: "PutItem1".to_string() }
  ];
  create_data(data_path.clone(), &items);

  let routes = all_routes(data_path.clone());
  let client = TestClient::new(routes);
  let res = client
    .put("/items/1")
    .body_json(&json!({
      "name": "NewPutItemName1"
    }))
    .send()
    .await;

  res.assert_status(StatusCode::UNAUTHORIZED);

  delete_file_if_exists(&data_path);
}
 */


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

async fn get_jwt(client: &TestClient<Route>) -> String {
  // Login user: admin1 ps: admin1

  let mut login_req = client
    .post("/users/login")
    .body_json(&json!({
      "name": "admin1",
      "pass": "admin1"
    }))
    .send()
    .await;

  let login_body = login_req.0.take_body().into_json::<LoginResponse>().await.unwrap();
  login_body.token
}



