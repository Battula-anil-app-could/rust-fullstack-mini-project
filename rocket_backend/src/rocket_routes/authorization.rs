use crate::auth::{authorize_user, password_hash, Credentials};
use crate::modules::{NewUser, User};
use crate::repositorys::{ UserRepository};
use crate::schema::users;
use rocket::serde::json::{json, Json};
use rocket::response::status::{self, Custom};
use rocket::Rocket;
use rocket_db_pools::diesel::QueryResult;
use rocket_db_pools::*;
use rocket::http::Status;
use rocket::serde::json::Value;
use crate::rocket_routes::DbConn;
use diesel_async::RunQueryDsl;



#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(mut db: Connection<DbConn>, credentials: Json<Credentials>) -> Result<Value, Custom<Value>>{
    let mut c = DbConn;
    UserRepository::find_by_user_name(&mut db, &credentials.username).await
    .map(|user| {
        
        if let Ok(user) = authorize_user(&user, credentials.into_inner()){
            return json!({"id": user.id, "username": user.username, "created_at": user.created_at})
        }

        json!("password error")
        
        }   
    )
    .map_err(|e| {
        rocket::error!("{e}");
        Custom(Status::InternalServerError, json!("wrong username or password"))
    })

}

#[rocket::post("/signup", format="json", data="<new_user>")]
pub async fn suing_up_user(mut db: Connection<DbConn>, new_user: Json<NewUser>) ->Result<Custom<Value>, Custom<Value>>{

    let user_from_client = new_user.into_inner();

    let password_hash = password_hash(user_from_client.password).unwrap();
    let new_user = NewUser{
        username: user_from_client.username,
        password: password_hash,
    };


    diesel::insert_into(users::table).values(new_user).get_result::<User>(&mut db).await
        .map(|user|{
            Custom(Status::Created, json!("created user successfully"))
        })
        .map_err(|e|{
            rocket::error!("{}", e);
            Custom(Status::InternalServerError, json!("Error while creating user"))
        })
}