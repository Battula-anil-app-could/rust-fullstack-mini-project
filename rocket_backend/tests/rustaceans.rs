use reqwest::{blocking::Client, StatusCode};
use rocket::form::validate::Contains;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
pub mod common;

#[test]
fn test_get_all_rustaceans(){
    let client = Client::new();
    let rustacean1 = common::create_rustacean(&client);
    let rustacean2 = common::create_rustacean(&client);
    let response = client.get(format!("{}/rustaceans", common::APP_HOST)).send().unwrap();
    let rustaceans:Value = response.json().unwrap();
    eprintln!("{:#?}", &rustacean1);
    eprintln!("{:#?}", &rustaceans.as_array());
    assert!(rustaceans.as_array().contains(&rustacean1));
    assert!(rustaceans.as_array().contains(&rustacean2));
    common::delete_rustacean(&client, rustacean1);
    common::delete_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustaceans() {
    let client = Client::new();
    let rustacean: Value  = common::create_rustacean(&client);
    assert_eq!(rustacean, json!(
        {
            "id": rustacean["id"],
            "name": "subbu",
            "email": "subbu@gmail.com",
            "created_at": rustacean["created_at"],
        }
    ));

    common::delete_rustacean(&client, rustacean);
}


#[test]
fn test_update_rustaceans() {
    let client = Client::new();
    let rustacean: Value  = common::create_rustacean(&client);
    let response = client.put(format!("{}/rustaceans/{}",common::APP_HOST, rustacean["id"]))
        .json(&json!({
            "id": rustacean["id"],
            "name": "subbarao",
            "email": "subbu@gmail.com",
            "created_at": rustacean["created_at"],
            }
        ))
        .send()
        .unwrap();
    let rustacean: Value  = response.json().unwrap();
    assert_eq!(rustacean, json!(
        {
            "id": rustacean["id"],
            "name": "subbarao",
            "email": "subbu@gmail.com",
            "created_at": rustacean["created_at"],
        }
    ));

    common::delete_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustacean() {
    let client = Client::new();
    
    let rustacean: Value  = common::create_rustacean(&client);
    let response = client.get(format!("{}/rustaceans/{}",common::APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    let rustacean: Value  = response.json().unwrap();
    eprint!("{:#?}", rustacean);
    assert_eq!(rustacean, json!(
        {
            "id": rustacean["id"],
            "name": "subbu",
            "email": "subbu@gmail.com",
            "created_at": rustacean["created_at"],
        }
    ));

    common::delete_rustacean(&client, rustacean);
}

#[test]
fn test_delte_rustacean() {
    let client = Client::new();
    let rustacean: Value  = common::create_rustacean(&client);
    let response = client.delete(format!("{}/rustaceans/{}",common::APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    
}


#[test]
fn test_view_rustacean_handel() {
    let client = Client::new();
    let response = client.get(format!("{}/rustaceans/{}",common::APP_HOST, 99999))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

}
