mod database_entitites;
mod database_logic;
use database_logic::DB_CONNECTION;

fn main() {
    println!("operating on {} connextion", DB_CONNECTION.lock().unwrap().db_name(0).unwrap());
}

