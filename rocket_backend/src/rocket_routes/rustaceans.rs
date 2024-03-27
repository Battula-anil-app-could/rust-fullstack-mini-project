
use crate::modules::{self, *};
use crate::repositorys::RustaceanRepository;
use rocket::serde::json::{json, Json};
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::*;
use rocket::http::Status;
use rocket::serde::json::Value;
use crate::rocket_routes::DbConn;



// #[rocket::delete("/rustaceans")]
// pub async fn delete_all_rustaceans(mut db: Connection<DbConn>) -> Result<NoContent, Custom<Value>> {
//     let client = Client::new();

//     let response = client.get("http://127.0.0.1:8000/rustaceans").send()
//         .map_err(|_| Custom(Status::InternalServerError, json!("Failed to fetch rustaceans")))?;
    
//     if response.status().is_success() {
//         let rustaceans = get_all_rustaceans(db);
                

//         for rustacean in rustaceans {
//             let rustacean: Rustacean = serde_json::from_value(rustacean.clone());
//             let id = rustacean["id"].as_i64().ok_or_else(|| {
//                 Custom(Status::InternalServerError, json!("Rustacean ID missing or invalid"))
//             })? as i32;

//             delete_rustaceans(db, id).await?;
//         }
//     } else {
//         return Err(Custom(Status::InternalServerError, json!("Failed to fetch rustaceans")));
//     }

//     Ok(NoContent)
// }

// async fn delete_rustaceans(mut db:AsyncPgConnection, id: i32) -> Result<(), Custom<Value>> {

//     RustaceanRepository::delete(&mut db, id).await
//         .map_err(|_| Custom(Status::InternalServerError, json!("Failed to delete rustacean")))?;

//     Ok(())
// }

// async fn get_all_rustaceans(mut db:Connection<DbConn>) -> Result<Value, Custom<Value>>{
//     RustaceanRepository::find_multiple(&mut db, 100).await 
//     .map(|rusatceans| json!(rusatceans))
//     .map_err(|e| {
//         rocket::error!("{}", e);
//         Custom(Status::InternalServerError, json!("Error"))
//      })   
// }
#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_multiple(&mut db, 500).await 
    .map(|rusatceans| json!(rusatceans))
    .map_err(|e| {
        rocket::error!("{e}");
        Custom(Status::InternalServerError, json!("Error"))
    })

}

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {  
    RustaceanRepository::find(&mut db, id).await 
    .map(|rusatcean| json!(rusatcean))
    .map_err(|e| {
        if e.to_string() == "Record not found"{
            rocket::error!("{}", e);
            Custom(Status::NotFound, json!("Error"))
        }else{
            rocket::error!("{}", e);
            Custom(Status::InternalServerError, json!("Error"))
        }
        
    })
}

#[rocket::post("/rustaceans",format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(mut db: Connection<DbConn>, new_rustacean: Json<NewRustancean>) -> Result<Custom<Value>, Custom<Value>> {  
    RustaceanRepository::create(&mut db, new_rustacean.into_inner()).await 
    .map(|rusatcean| Custom(Status::Created, json!(rusatcean)))
    .map_err(|e| {
        rocket::error!("{}", e);
        Custom(Status::InternalServerError, json!("Error"))
    })
}


#[rocket::put("/rustaceans/<id>",format = "json", data = "<rustacean>")]
pub async fn update_rustacean(mut db: Connection<DbConn>, id:i32, rustacean: Json<modules::Rustacean>) -> Result<Value, Custom<Value>> {  
    RustaceanRepository::update(&mut db, id, rustacean.into_inner()).await 
    .map(|rusatcean| json!(rusatcean))
    .map_err(|e| {
        rocket::error!("{}", e);
        Custom(Status::InternalServerError, json!("Error"))
    })
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {  
    RustaceanRepository::delete(&mut db, id).await 
    .map(|_| NoContent)
        .map_err(|e| {rocket::error!("{}", e);Custom(Status::InternalServerError, json!("Error"))})

}



 


