// This file contains definitions for entities stored in the database
// TODO: typedef ID


use std::sync::MutexGuard;
use const_format::{concatcp, formatcp};
use rocket::futures::StreamExt;
use rocket::serde::{Deserialize, Serialize};
use rusqlite::{Connection, Params, ParamsFromIter, Row};

pub trait DatabaseEntity : Sized {
    const TABLE_NAME: &'static str;
    const CREATE_STATEMENT: &'static str;
    const UPDATE_STATEMENT: &'static str;

    fn as_params(&self) -> impl Params;
    fn set_id(&mut self, id: Option<i64>);
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self>;


    fn create(&mut self, connection: &mut MutexGuard<Connection>) -> rusqlite::Result<i64> {
        self.set_id(None);
        connection.execute(Self::CREATE_STATEMENT, self.as_params())?;
        let assigned_id: i64 = connection.last_insert_rowid();
        self.set_id(Some(assigned_id));
        Ok(assigned_id)
    }
    
    fn update(&mut self, connection: &mut MutexGuard<Connection>) -> rusqlite::Result<usize> {
        connection.execute(Self::UPDATE_STATEMENT, self.as_params())
    }

    fn get_all(connection: &mut MutexGuard<Connection>) -> rusqlite::Result<Vec<Self>> {
        let mut statement = connection.prepare(&format!("SELECT * FROM {};", Self::TABLE_NAME))?;
        let mut query = statement.query_map((), |row| {Self::from_row(row)})?;
        let mut all: Vec<Self> = Vec::new();

        for item in query {
            all.push(item?);
        }

        Ok(all)
    }

    fn get_by_id(connection: &mut MutexGuard<Connection>, id: i64) -> rusqlite::Result<Self> {
        let mut statement = connection.prepare(&format!("SELECT * FROM {} WHERE id = ?1;", Self::TABLE_NAME))?;
        statement.query_row((id,), |row| {Self::from_row(row)})

    }

}

// =================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Option<i64>, // User unique ID
    username: String, // username
    guest_rating: Option<u64>, // rating as guest
    host_rating: Option<u64> // rating as host
}

impl DatabaseEntity for User {
    const TABLE_NAME: &'static str = "users";
    const CREATE_STATEMENT: &'static str = formatcp!("INSERT INTO {} values (?1, ?2, ?3, ?4);", User::TABLE_NAME);
    const UPDATE_STATEMENT: &'static str = formatcp!("UPDATE {} SET username = ?2, guest_rating = ?3, host_rating = ?4 WHERE id = ?1",
        User::TABLE_NAME);


    fn as_params(&self) -> impl Params {
        (self.id,
         self.username.clone(),
         self.guest_rating,
         self.host_rating)
    }

    fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }

    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            username: row.get(1)?,
            guest_rating: row.get(2)?,
            host_rating: row.get(3)?
        })
    }
}

impl User {
    pub fn unique_username(&self, connection: &mut MutexGuard<Connection>) -> bool {
       connection.query_row(&format!("SELECT COUNT(*) FROM {} WHERE username = ?1", Self::TABLE_NAME),
                            (&self.username, ),
                            |r| r.get::<_, i64>(0)).unwrap() == 0
    }



    pub fn get_upcoming_parties(&self, connection: &mut MutexGuard<Connection>) -> rusqlite::Result<Vec<Party>> {
        let mut parties: Vec<Party> = Vec::new();
        let mut statement = connection.prepare("SELECT * FROM parties LEFT JOIN attendings ON party.id = attendings.party_id where attendings.attendee_id = ?1;")?;
        let mut query = statement.query_map([self.id], |r| Party::from_row(r))?;
        // TODO: add upcoming check
        for row in query {
            parties.push(row?);
        }
        Ok(parties)
    }
}

// =================================================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizedUser {
    id: Option<i64>, // ID linking authorized user to known user
    sec_hash: String, // md5 hash of User-Agent||IpAddr
    auth_key: String, // Randomly generated 255 character string used to authorize API requests
    last_accessed: u64, // Unix timestamp of last time this authorized user had accessed API
}

impl DatabaseEntity for AuthorizedUser {
    const TABLE_NAME: &'static str = "authorized_users";
    const CREATE_STATEMENT: &'static str = formatcp!("INSERT INTO {} values (?1, ?2, ?3, ?4);", AuthorizedUser::TABLE_NAME);
    const UPDATE_STATEMENT: &'static str = formatcp!("UPDATE {} SET sec_hash = ?2, auth_key = ?3, last_accessed = ?4 WHERE id = ?1",
        AuthorizedUser::TABLE_NAME);


    fn as_params(&self) -> impl Params {
        (self.id,
        self.sec_hash.clone(),
        self.auth_key.clone(),
        self.last_accessed)
    }

    fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }

    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            sec_hash: row.get(1)?,
            auth_key: row.get(2)?,
            last_accessed: row.get(3)?
        })
    }

}

// =================================================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Party {
    id: Option<i64>, // Unique party ID
    host_id: i64, // User ID of party creator
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
    const UPDATE_STATEMENT: &'static str = formatcp!("UPDATE {} SET host_id = ?2, title = ?3, description = ?4, latitude = ?5, longitude = ?6, capacity = ?7, attendees = ?8, start_time = ?9, visibility = ?10 WHERE id = ?1",
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

    fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }

    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            host_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            latitude: row.get(4)?,
            longitude: row.get(5)?,
            capacity: row.get(6)?,
            attendees: row.get(7)?,
            start_time: row.get(8)?,
            visibility: row.get(9)?,
        })
    }
}

impl Party {
    pub fn
    get_attendees(&self, dbconn: &mut MutexGuard<Connection>) -> rusqlite::Result<Vec<User>> {
        let mut attendees: Vec<User> = Vec::new();
        let mut statement = dbconn.prepare("SELECT * FROM users LEFT JOIN attendings ON users.id = attendings.attendee_id where attendings.party_id = ?1;")?;
        let mut query = statement.query_map([self.id], |r| User::from_row(r))?;

        for row in query {
            attendees.push(row?);
        }

        Ok(attendees)
    }

}

// =================================================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct Attending {
    id: Option<i64>, // Unique ID of the attending
    attendee_id: i64, // ID linking attending to the attendee
    party_id: i64, // ID linking attending to the party
}

impl DatabaseEntity for Attending {
    const TABLE_NAME: &'static str = "attendings";
    const CREATE_STATEMENT: &'static str = formatcp!("INSERT INTO {} values (?1, ?2, ?3);", Attending::TABLE_NAME);
    const UPDATE_STATEMENT: &'static str = formatcp!("UPDATE {} SET attendee_id = ?2, party_id = ?3 WHERE id = ?1",
        Attending::TABLE_NAME);

    fn as_params(&self) -> impl Params {
        (
            self.id,
            self.attendee_id,
            self.party_id
        )
    }

    fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }

    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            attendee_id: row.get(1)?,
            party_id: row.get(2)?,
        })
    }

}

impl Attending {
    pub fn new(pid: i64, uid: i64) -> Self {
        Self {
            id: None,
            party_id: pid,
            attendee_id: uid
        }
    }
}


#[cfg(test)]
mod test {
    use crate::database_entitites::{Attending, DatabaseEntity, Party, User};
    use crate::database_logic::DB_CONNECTION;

    #[test]
    fn test_creating_items() {
        let mut john_doe: User = User {
            id: None,
            username: "John Doe".to_string(),
            guest_rating: None,
            host_rating: None,
        };
        
        let mut john_doe_party: Party = Party {
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

        let mut zuckerberg: User = User {
            id: None,
            username: "Zucc".to_string(),
            guest_rating: None,
            host_rating: None,
        };

        let mut zucc_goes_to_does_party: Attending = Attending {
            id: None,
            attendee_id: 0,
            party_id: 0,
        };

        let mut dbcon = DB_CONNECTION.lock().unwrap();
        println!("John Doe was registered under id {}", john_doe.create(&mut dbcon).unwrap());
        john_doe_party.host_id = john_doe.id.unwrap();
        println!("John Doe's party was registered under id {}", john_doe_party.create(&mut dbcon).unwrap());
        println!("Zuckerberg was registered under id {}", zuckerberg.create(&mut dbcon).unwrap());
        zucc_goes_to_does_party.party_id = john_doe_party.id.unwrap();
        zucc_goes_to_does_party.attendee_id = zuckerberg.id.unwrap();
        println!("Zuckerberg attends john does's party under id {}", zucc_goes_to_does_party.create(&mut dbcon).unwrap());


    }

    #[test]
    fn test_create_and_update() {
        let mut dbcon = DB_CONNECTION.lock().unwrap();
        let mut van_damme: User = User {
            id: None,
            username: "Jean-Claude Van Damme".to_string(),
            guest_rating: Some(0),
            host_rating: Some(0),
        };
        van_damme.create(&mut dbcon).unwrap();
        van_damme.username = "Better Than Chuck Norris!".to_string();
        van_damme.host_rating = Some(9999);
        van_damme.update(&mut dbcon).unwrap();

        let mut van_dammes_party: Party = Party {
            id: None,
            host_id: van_damme.id.unwrap(),
            title: "Best Party Every!".to_string(),
            description: "Fuck Steven Seagal!".to_string(),
            latitude: 0.0,
            longitude: 0.0,
            capacity: 0,
            attendees: 1,
            start_time: 0,
            visibility: 1,
        };

        van_dammes_party.create(&mut dbcon).unwrap();
        van_dammes_party.description.extend(" And fuck Putin!".chars());
        van_dammes_party.update(&mut dbcon).unwrap();

        let mut great_xi_world_leader_jinping: User = User {
            id: None,
            username: "Winnie The Pooh".to_string(),
            guest_rating: Some(0),
            host_rating: Some(1000000),
        };

        let mut dio: User = User {
            id: None,
            username: "But it was me, DIO!".to_string(),
            guest_rating: Some(0),
            host_rating: Some(1000000),
        };

        great_xi_world_leader_jinping.create(&mut dbcon).unwrap();
        let mut xi_goes_out: Attending = Attending {
            id: None,
            attendee_id: great_xi_world_leader_jinping.id.unwrap(),
            party_id: van_dammes_party.id.unwrap(),
        };



        xi_goes_out.create(&mut dbcon).unwrap();

        dio.create(&mut dbcon).unwrap();
        xi_goes_out.attendee_id = dio.id.unwrap();
        xi_goes_out.update(&mut dbcon).unwrap();

    }

    #[test]
    fn test_from_row() {
        let mut dbcon = DB_CONNECTION.lock().unwrap();
        println!("{:?}", dbcon.query_row("SELECT * FROM users WHERE id = 1;", (), |r| {User::from_row(r)}).unwrap());
    }


    #[test]
    fn test_get_all() {
        let mut dbcon = DB_CONNECTION.lock().unwrap();
        User::get_all(&mut dbcon).unwrap().iter().for_each(|r| println!("user: {:?}", r));
        Party::get_all(&mut dbcon).unwrap().iter().for_each(|r| println!("party: {:?}", r));
        Attending::get_all(&mut dbcon).unwrap().iter().for_each(|r| println!("attending: {:?}", r));
    }
}
