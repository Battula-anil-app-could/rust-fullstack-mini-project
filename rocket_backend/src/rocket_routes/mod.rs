use rocket::fairing::{Fairing, Info, Kind};
use rocket_db_pools::*;
use rocket_sync_db_pools::database;
use crate::rocket_routes::rocket::{Response, Request};


pub mod rustaceans;
pub mod crates;
pub mod commonds;
pub mod authorization;


#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);



#[rocket::options("/<_route_args..>")]
pub fn options(_route_args: Option<std::path::PathBuf>){

}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors{
    fn info(&self) -> Info {
        Info {
            name: "Cors for all responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("Access-Control-Allow-Origin", "*");
        res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
        res.set_raw_header("Access-Control-Allow-Headers", "*");
        res.set_raw_header("Access-Control-Allow-Credentials", "*");
    }
    
}