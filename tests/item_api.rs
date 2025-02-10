use std::{fs::{remove_file, OpenOptions}, io::Write};

use play_asia::{all_routes, items::Item};
use poem::{http::StatusCode, test::TestClient};
use serde_json::{json, to_string_pretty};

#[tokio::test]
async fn test_get_items_no_item() {
  let data_path = "data_test1.json".to_string();
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
  let data_path = "data_test2.json".to_string();
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