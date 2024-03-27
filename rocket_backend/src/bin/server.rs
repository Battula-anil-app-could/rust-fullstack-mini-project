extern crate rocket_backend;
use rocket_db_pools::*;


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            rocket_backend::rocket_routes::options,
            rocket_backend::rocket_routes::authorization::login,
            rocket_backend::rocket_routes::authorization::suing_up_user,
            rocket_backend::rocket_routes::rustaceans::get_rustaceans,
            rocket_backend::rocket_routes::rustaceans::get_rustacean,
            rocket_backend::rocket_routes::rustaceans::create_rustacean,
            rocket_backend::rocket_routes::rustaceans::update_rustacean,
            rocket_backend::rocket_routes::rustaceans::delete_rustacean,
            
            rocket_backend::rocket_routes::crates::get_crates,
            rocket_backend::rocket_routes::crates::get_crate,
            rocket_backend::rocket_routes::crates::create_crate,
            rocket_backend::rocket_routes::crates::update_crate,
            rocket_backend::rocket_routes::crates::delete_crate,
        ])
        .attach(rocket_backend::rocket_routes::Cors)
        .attach(rocket_backend::rocket_routes::DbConn::init())
        .launch()
        .await;
}
