
use chrono::NaiveDateTime;
use diesel::{prelude::*};
use crate::schema::*;
use serde::*;




#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

pub struct Rustacean{
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = rustaceans)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewRustancean{
    pub name: String,
    pub email: String, 
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Crates{
    pub id:i32,
    pub rustacean_id: i32,
    pub code:String,
    pub name:String,
    pub version: String,
    pub description: Option<String>,
    pub created_at : NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crates)]
pub struct NewCreate{
    pub rustacean_id: i32,
    pub code:String,
    pub name:String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Queryable,  Debug, Identifiable, Serialize, Deserialize, Clone)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub password: String,
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
}


#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser{
    pub username: String,
    pub password: String,
}

#[derive(Queryable,Identifiable, Debug)]
pub struct Role{
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = roles)]
pub struct NewRole{
    pub code: String,
    pub name: String,
}

#[derive(Queryable,Associations, Identifiable,Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = user_roles)]
pub struct UserRole{
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32
}


#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = user_roles)]

pub struct  NeewUserRole{
    pub user_id: i32,
    pub role_id: i32
}