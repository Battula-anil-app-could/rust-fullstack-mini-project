
use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;

pub const APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn create_rustacean(client: &Client) -> Value{
    let response = client.post(format!("{}/rustaceans", APP_HOST))
    .json(&json!({
            "name": "subbu",
            "email": "subbu@gmail.com"
    }

    ))
    .send()
    .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

pub fn delete_rustacean(client: &Client, rustacean: Value) -> (){
    let response = client.delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT)
}




pub fn create_crate(client: &Client, rustacean: &Value) -> Value{
    
    let response = client.post(format!("{}/crates", APP_HOST))
        .json(&json!(
            {
               "rustacean_id": rustacean["id"],
               "code": "println()",
               "name": "ramarao",
               "version": "v0.0.1",
               "description": "created at 2024"
            }
        ))
        .send() 
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()

    
}

pub fn delete_crate(client: &Client, a_crate: Value){
    let response = client.delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
