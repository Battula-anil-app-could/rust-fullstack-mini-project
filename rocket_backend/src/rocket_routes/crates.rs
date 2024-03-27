use crate::modules::{self, *};
use crate::repositorys::CratesRepository;
use rocket::serde::json::{json, Json};
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::*;
use rocket::http::Status;
use rocket::serde::json::Value;
use crate::rocket_routes::DbConn;

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    CratesRepository::find_all(&mut db, 100).await 
    .map(|crates| json!(crates))
    .map_err(|e| {rocket::error!("{}", e); Custom(Status::InternalServerError, json!("Error"))})

}

#[rocket::get("/crates/<id>")]
pub async fn get_crate(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {  
    CratesRepository::find(&mut db, id).await 
    .map(|crate_a| json!(crate_a))
    .map_err(|e| {
        if e.to_string() == "Record not found"{
            rocket::error!("{}", e); Custom(Status::NotFound, json!("Error"))
        }else{
            rocket::error!("{}", e); Custom(Status::InternalServerError, json!("Error"))
        }
        
    })
}

#[rocket::post("/crates",format = "json", data = "<new_crate>")]
pub async fn create_crate(mut db: Connection<DbConn>, new_crate: Json<NewCreate>) -> Result<Custom<Value>, Custom<Value>> {  
    CratesRepository::create(&mut db, new_crate.into_inner()).await 
    .map(|crate_a| Custom(Status::Created, json!(crate_a)))
    .map_err(|e| {rocket::error!("{}", e); Custom(Status::InternalServerError, json!("Error"))})
}

#[rocket::put("/crates/<id>",format = "json", data = "<crate_a>")]
pub async fn update_crate(mut db: Connection<DbConn>, id:i32, crate_a: Json<modules::Crates>) -> Result<Value, Custom<Value>> {  
    CratesRepository::update(&mut db, id, crate_a.into_inner()).await 
    .map(|crate_a| json!(crate_a))
    .map_err(|e| {rocket::error!("{}", e); Custom(Status::InternalServerError, json!("Error"))})
}
    
#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {  
    CratesRepository::delete(&mut db, id).await 
    .map(|_| NoContent)
    .map_err(|e| {rocket::error!("{}", e); Custom(Status::InternalServerError, json!("Error"))})
}