// This file contains definitions for entities stored in the database


use std::sync::MutexGuard;
use const_format::{concatcp, formatcp};
use rusqlite::{Connection, Params};

pub trait DatabaseEntity {
    const TABLE_NAME: &'static str;
    const CREATE_STATEMENT: &'static str;

    fn as_params(&self) -> impl Params;
    fn insert(&self, connection: &mut MutexGuard<Connection>) -> rusqlite::Result<usize> {
        connection.execute(Self::CREATE_STATEMENT, self.as_params())
    }
}

// =================================================================================================

pub struct User {
    id: Option<u64>, // User unique ID
    username: String, // username
    guest_rating: Option<u64>, // rating as guest
    host_rating: Option<u64> // rating as host
}

impl DatabaseEntity for User {
    const TABLE_NAME: &'static str = "users";
    const CREATE_STATEMENT: &'static str = formatcp!("INSERT INTO {} values (?1, ?2, ?3, ?4);", User::TABLE_NAME);


    fn as_params(&self) -> impl Params {
        (self.id,
         self.username.clone(),
         self.guest_rating,
         self.host_rating)
    }
}

// =================================================================================================

pub struct AuthorizedUser {
    id: u64, // ID linking authorized user to known user
    sec_hash: String, // md5 hash of User-Agent||IpAddr
    auth_key: String, // Randomly generated 255 character string used to authorize API requests
    last_accessed: u64, // Unix timestamp of last time this authorized user had accessed API
}

impl DatabaseEntity for AuthorizedUser {
    const TABLE_NAME: &'static str = "authorized_users";
    const CREATE_STATEMENT: &'static str = formatcp!("INSERT INTO {} values (?1, ?2, ?3, ?4);", AuthorizedUser::TABLE_NAME);

    fn as_params(&self) -> impl Params {
        (self.id,
        self.sec_hash.clone(),
        self.auth_key.clone(),
        self.last_accessed)
    }



}

// =================================================================================================

pub struct Party {
    id: Option<u64>, // Unique party ID
    host_id: u64, // User ID of party creator
    title: String, // Title of the party
    description: String, // SHort description of the party
    latitude: f64, // Latitude of the party location
    longitude: f64, // Longitude of party location
    capacity: u64, // Party capacity
    attendees: u64, // Number of attendees
    start_time: u64, // starting time of the party
    visibility: u64, // type of party visibility
}

impl DatabaseEntity for Party {
    const TABLE_NAME: &'static str = "parties";
    const CREATE_STATEMENT: &'static str = formatcp!("INSERT INTO {} values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10);",
        Party::TABLE_NAME);

    fn as_params(&self) -> impl Params {
        (   self.id,
            self.host_id,
            self.title.clone(),
            self.description.clone(),
            self.latitude,
            self.longitude,
            self.capacity,
            self.attendees,
            self.start_time,
            self.visibility
        )
    }
}

// =================================================================================================

pub struct Attending {
    id: Option<u64>, // Unique ID of the attending
    attendee_id: u64, // ID linking attending to the attendee
    party_id: u64, // ID linking attending to the party
}

impl DatabaseEntity for Attending {
    const TABLE_NAME: &'static str = "attendings";
    const CREATE_STATEMENT: &'static str = formatcp!("INSERT INTO {} values (?1, ?2, ?3);", Attending::TABLE_NAME);

    fn as_params(&self) -> impl Params {
        (
            self.id,
            self.attendee_id,
            self.party_id
        )
    }
}


#[cfg(test)]
mod test {
    use crate::database_entitites::{Attending, DatabaseEntity, Party, User};
    use crate::database_logic::DB_CONNECTION;

    #[test]
    fn test_as_params() {
        let john_doe: User = User {
            id: None,
            username: "John Doe".to_string(),
            guest_rating: None,
            host_rating: None,
        };
        
        let john_doe_party: Party = Party {
            id: None,
            host_id: 0,
            title: "".to_string(),
            description: "".to_string(),
            latitude: 0.0,
            longitude: 0.0,
            capacity: 0,
            attendees: 0,
            start_time: 0,
            visibility: 0,
        };

        let zuckerberg: User = User {
            id: None,
            username: "Zucc".to_string(),
            guest_rating: None,
            host_rating: None,
        };

        let zucc_goes_to_does_party: Attending = Attending {
            id: None,
            attendee_id: 0,
            party_id: 0,
        };

        let mut dbcon = DB_CONNECTION.lock().unwrap();
        john_doe.insert(&mut dbcon);
        john_doe_party.insert(&mut dbcon);
        zuckerberg.insert(&mut dbcon);
        zucc_goes_to_does_party.insert(&mut dbcon);


    }
}
