use clap::{value_parser, Arg, Command};

extern  crate rocket_backend;
#[tokio::main]
async fn main(){
    let matches = Command::new("rocket")
    .about("user commond prompt")
    .arg_required_else_help(true)
    .subcommand(
        Command::new("user")
        .about("user interface")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("create")
            .about("create user")
            .arg_required_else_help(true)
            .arg(Arg::new("username").required(true))
            .arg(Arg::new("password").required(true))
            .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))

        )
        .subcommand(
            Command::new("list")
            .about("list of users")
            
        )
        .subcommand(
            Command::new("delete")
            .about("delete user")
            .arg_required_else_help(true)
            .arg(Arg::new("id").required(true).value_parser(value_parser!(i32)))
            
        )
    ).get_matches();

    match  matches.subcommand() {
        Some(("user", sub_matches)) => match  sub_matches.subcommand() {
            Some(("create", sub_matches)) => rocket_backend::rocket_routes::commonds::create_user(
                sub_matches.get_one::<String>("username").unwrap().to_owned(),
                sub_matches.get_one::<String>("password").unwrap().to_owned(),
                sub_matches.get_many::<String>("roles").unwrap().map(|v| v.to_owned()).collect()
                ).await,
            Some(("list", _)) => rocket_backend::rocket_routes::commonds::list_users().await,
            Some(("delete", sub_matches)) => rocket_backend::rocket_routes::commonds::delete_user(
                sub_matches.get_one::<i32>("id").unwrap().to_owned()
            ).await,
            _=>{},

        },
        _=>{},
    }
}