#![feature(lazy_cell)]
#![feature(c_str_literals)]

use std::sync::{LazyLock, Mutex};
use rusqlite::Connection;

pub static DB_PATH: &str = "C:\\Users\\grass\\Desktop\\codes\\Rust\\MiaoZeDong\\db\\database";
pub static DB_CONNECTION: LazyLock<Mutex<Connection>> = LazyLock::new(|| {
    Mutex::new(init_database(Connection::open(DB_PATH).unwrap()))
});

fn init_database(mut database_connection: Connection) -> Connection {
    let init_code: &str = include_str!("init.sql");
    database_connection.execute_batch(init_code).unwrap();
    database_connection
}