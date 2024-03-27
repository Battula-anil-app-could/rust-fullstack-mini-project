use argon2::{password_hash::{rand_core::OsRng, SaltString}, PasswordHasher};
use diesel_async::{AsyncPgConnection, AsyncConnection};

use crate::{auth::password_hash, modules::NewUser, repositorys::{RolesRepository, UserRepository}};



pub async fn load_bd_con() -> AsyncPgConnection{
    let db_url = "postgresql://postgres:postgres@localhost/app_db";

    AsyncPgConnection::establish(&db_url).await
    .expect("can not load database")
}



pub async fn create_user(username: String, password: String, role_codes: Vec<String>){
    let mut c = load_bd_con().await;
    let password_hash = password_hash(password).unwrap();
    let new_user = NewUser{
        username,
        password: password_hash,
    };

    let user = UserRepository::create(&mut c, new_user, role_codes).await.unwrap();

    println!("user created successfully {:#?}", user);

    let roles = RolesRepository::find_by_user(&mut c, &user).await.unwrap();
    println!("roles of user {:#?}", roles);
}


pub async fn list_users(){
    let mut c = load_bd_con().await;

    println!("is calling");
    let users = UserRepository::list_of_users(&mut c).await.unwrap();

    for user in users{
        println!("{:?}", user);
    }
}

pub async fn delete_user(id: i32){
    let mut c = load_bd_con().await;
    

    UserRepository::delete_user(&mut c, id).await.unwrap();
}
