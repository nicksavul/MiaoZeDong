
mod database_entitites;
mod database_logic;
use database_logic::DB_CONNECTION;
use database_entitites::{User, Party, DatabaseEntity};

#[macro_use] extern crate rocket;

#[get("/users")]
fn endpoint_get_all_users() -> String {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    User::get_all(&mut dbcon)
        .unwrap()
        .iter()
        .map(|usr| { format!("{:?}\n", usr) })
        .collect::<String>()

}

#[get("/users/<id>")]
fn endpoint_get_user(id: u64) -> String {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    format!("{:?}", User::get_by_id(&mut dbcon, id as i64).unwrap())

}


#[get("/parties")]
fn endpoint_get_all_parties() -> String {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    Party::get_all(&mut dbcon)
        .unwrap()
        .iter()
        .map(|usr| { format!("{:?}\n", usr) })
        .collect::<String>()

}

#[get("/parties/<id>")]
fn endpoint_get_party(id: u64) -> String {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    format!("{:?}", Party::get_by_id(&mut dbcon, id as i64).unwrap())

}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![endpoint_get_user, endpoint_get_all_users, endpoint_get_party, endpoint_get_all_parties])
}

