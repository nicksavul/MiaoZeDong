use rocket::data::Outcome;
use rocket::http::ContentType;
use rocket::Response;
use rocket::serde::json::Json;
use crate::database_entitites::{Party, User, DatabaseEntity, Attending};
use crate::database_logic::DB_CONNECTION;

// TODO: Add proper "if exists" validation for update calls
// Ensure that, only existing users can be altered
// Only upcoming parties can be altered
// TODO: Add proper user verification so only owner can edit parties, etc.



#[get("/users")]
pub fn endpoint_get_all_users() -> Json<Vec<User>> {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    Json(User::get_all(&mut dbcon)
        .unwrap())

}



#[get("/users/<id>/parties")]
pub fn endpoint_get_parties_of_user(id: u64) -> Json<Vec<Party>> {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    Json(User::get_by_id(&mut dbcon, id as i64)
        .unwrap().get_upcoming_parties(&mut dbcon).unwrap())
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
        panic!("fuck this shit non-unique username"); // TODO: Add proper error code return rather than panic
    }
    user.create(&mut dbcon).unwrap();
    Json(user)

}

#[put("/users/<id>", format = "json", data = "<data>")]
pub fn endpoint_update_user(id: u64, data: Json<User>) -> Json<User> {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    let mut user: User = data.0;
    user.update(&mut dbcon).unwrap();
    Json(user)

}


#[get("/ping")]
pub fn ping() -> &'static str {
    "pong!"
}





#[get("/parties/<id>/users")]
pub fn endpoint_get_users_of_party(id: u64) -> Json<Vec<User>> {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    Json(dbg!(Party::get_by_id(&mut dbcon, id as i64).unwrap())
        .get_attendees(&mut dbcon).unwrap())
}


#[post("/parties/<pid>/users/<uid>")]
pub fn endpoint_set_attending(pid: u64, uid: u64) -> Json<Attending> {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    let mut attending: Attending = Attending::new(pid as i64, uid as i64);
    attending.create(&mut dbcon).unwrap();
    Json(attending)
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


#[put("/parties/<id>", format = "json", data = "<data>")]
pub fn endpoint_update_party(id: u64, data: Json<Party>) -> Json<Party> {
    let mut dbcon = DB_CONNECTION.lock().unwrap();
    let mut party: Party = data.0;
    party.update(&mut dbcon).unwrap();
    Json(party)
}


