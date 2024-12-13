
mod database_entitites;
mod database_logic;
mod api_routes;

use database_logic::DB_CONNECTION;
use database_entitites::{User, Party, DatabaseEntity};
use api_routes::*;

#[macro_use] extern crate rocket;



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![endpoint_get_user,
        endpoint_get_all_users,
        endpoint_get_party,
        endpoint_get_all_parties,
        endpoint_create_party,
        endpoint_create_user,
        endpoint_update_user,
        endpoint_update_party,
        endpoint_get_parties_of_user,
        endpoint_get_users_of_party,
        endpoint_set_attending])
}

