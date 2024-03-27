use gloo_net::{Error, http::Request};
use serde_json::json;
use serde::{Serialize, Deserialize};

use super::APP_HOST;
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub created_at: String,
}
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct MainUser{
    pub id: i32,
    pub username: String,
    pub created_at: String,
}
pub async fn api_login(username: String, password: String) -> Result<User, Error>{
    let response = Request::post(&format!("{}/login", APP_HOST))
    .json(
        &json!(
            {
                "username": username,
                "password": password,
            }
        )
    )?
    .send().await?;

    Ok(response.json::<User>().await?)
   
    
}