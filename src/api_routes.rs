use rocket::data::Outcome;
use rocket::http::ContentType;
use rocket::Response;
use rocket::serde::json::Json;
use crate::database_entitites::{Party, User, DatabaseEntity};
use crate::database_logic::DB_CONNECTION;



#[get("/users")]
pub fn endpoint_get_all_users() -> Json<Vec<User>> {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    Json(User::get_all(&mut dbcon)
        .unwrap())

}

#[get("/users/<id>")]
pub fn endpoint_get_user(id: u64) -> Json<User> {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    Json(User::get_by_id(&mut dbcon, id as i64).unwrap())

}


#[post("/users", format = "json", data = "<data>")]
pub fn endpoint_create_user(data: Json<User>) -> Json<User> {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    let mut user: User = data.0;


    if !user.unique_username(&mut dbcon) {
        panic!("fuck this shit non-unique username");
    }
    user.create(&mut dbcon).unwrap();
    Json(user)

}


#[get("/parties")]
pub fn endpoint_get_all_parties() -> Json<Vec<Party>> {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    Json(Party::get_all(&mut dbcon)
        .unwrap())

}

#[get("/parties/<id>")]
pub fn endpoint_get_party(id: u64) -> Json<Party> {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    Json(Party::get_by_id(&mut dbcon, id as i64).unwrap())
}

#[post("/parties", format = "json", data = "<data>")]
pub fn endpoint_create_party(data: Json<Party>) -> Json<Party> {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    let mut party: Party = data.0;
    party.create(&mut dbcon).unwrap();
    Json(party)
}