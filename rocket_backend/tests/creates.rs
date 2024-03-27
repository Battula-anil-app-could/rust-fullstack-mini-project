use reqwest::{blocking::*, StatusCode};
use rocket::form::validate::Contains;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::*;
pub mod common;
#[test]
fn test_get_all_crates() {
    let client = Client::new();
    let rustacean = common::create_rustacean(&client);
    let crate1 = common::create_crate(&client, &rustacean);
    let crate2 = common::create_crate(&client, &rustacean);
    let response = client.get(format!("{}/crates", common::APP_HOST)).send().unwrap();
    
    let a_crate: Value = response.json().unwrap();

    assert!(a_crate.as_array().contains(&crate1));
    assert!(a_crate.as_array().contains(&crate2));

    
    common::delete_crate(&client, crate1);
    common::delete_crate(&client, crate2);
    common::delete_rustacean(&client, rustacean);
}

#[test]
fn test_create_crate(){
    let client = Client::new();
    let rustacean = common::create_rustacean(&client);
    let a_crate = common::create_crate(&client, &rustacean);

    common::delete_crate(&client, a_crate);
    common::delete_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    let client = Client::new();
    let rustacean = common::create_rustacean(&client);
    let a_crate: Value  = common::create_crate(&client, &rustacean);
    let response = client.put(format!("{}/crates/{}",common::APP_HOST, a_crate["id"]))
        .json(&json!({
            "id": a_crate["id"],
            "rustacean_id": a_crate["rustacean_id"],
            "code": "println()",
            "name": "ragu",
            "version": "v0.0.1",
            "description": "created at 2024",
            "created_at": a_crate["created_at"],
            }
        ))
        .send()
        .unwrap();
    let a_crate: Value  = response.json().unwrap();
    assert_eq!(a_crate, json!(
        {
            "id": a_crate["id"],
            "rustacean_id": a_crate["rustacean_id"],
            "code": "println()",
            "name": "ragu",
            "version": "v0.0.1",
            "description": "created at 2024",
            "created_at": a_crate["created_at"],
        }
    ));

    common::delete_crate(&client, a_crate);
    common::delete_rustacean(&client, rustacean);
}

#[test]
fn test_view_crate() {
    let client = Client::new();
    let rustacean = common::create_rustacean(&client);
    let a_crate: Value  = common::create_crate(&client, &rustacean);
    let response = client.get(format!("{}/crates/{}",common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    let a_crate: Value  = response.json().unwrap();
    assert_eq!(a_crate, json!(
        {
            "id": a_crate["id"],
            "rustacean_id": a_crate["rustacean_id"],
            "code": "println()",
            "name": "ramarao",
            "version": "v0.0.1",
            "description": "created at 2024",
            "created_at": a_crate["created_at"],
        }
    ));

    common::delete_crate(&client, a_crate);
    common::delete_rustacean(&client, rustacean);
}



#[test]
fn test_delte_crate() {
    let client = Client::new();
    let rustacean = common::create_rustacean(&client);
    let a_crate: Value  = common::create_crate(&client, &rustacean);
    let response = client.delete(format!("{}/crates/{}",common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    common::delete_rustacean(&client, rustacean);
    
}


#[test]
fn test_view_crate_handle() {
    let client = Client::new();
    let response = client.get(format!("{}/crates/{}",common::APP_HOST, 88888))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND)
   
}