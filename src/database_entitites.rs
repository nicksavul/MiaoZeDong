// This file contains definitions for entities stored in the database


pub trait DatabaseEntity {
    const TABLE_NAME: &'static str;
}

// =================================================================================================

pub struct User {
    id: u64, // User unique ID
    username: String, // username
    guest_rating: u64, // rating as guest
    host_rating: u64 // rating as host
}

impl DatabaseEntity for User {
    const TABLE_NAME: &'static str = "users";
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
}

// =================================================================================================

pub struct Party {
    id: u64, // Unique party ID
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
}

// =================================================================================================

pub struct Attending {
    id: u64, // Unique ID of the attending
    attendee_id: u64, // ID linking attending to the attendee
    party_id: u64, // ID linking attending to the party
}

impl DatabaseEntity for Attending {
    const TABLE_NAME: &'static str = "attendings";
}

