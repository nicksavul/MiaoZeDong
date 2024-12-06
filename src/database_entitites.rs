// This file contains definitions for entities stored in the database



pub struct User {
    id: u64, // User unique ID
}

pub struct AuthorizedUser {
    id: u64, // ID linking authorized user to known user
    sec_hash: String, // md5 hash of User-Agent||IpAddr
    auth_key: String, // Randomly generated 255 character string used to authorize API requests
    last_accessed: u64, // Unix timestamp of last time this authorized user had accessed API
}

pub struct Party {
    id: u64, // Unique party ID
    host_id: u64, // User ID of party creator
    title: String, // Title of the party
    description: String, // SHort description of the party
    latitude: u64, // Latitude of the party location
    longitude: u64, // Longitude of party location
    capacity: u64, // Party capacity
    attendees: u64, // Number of attendees
}

pub struct Attending {
    attending_id: u64, // Unique ID of the attending
    attendee_id: u64, // ID linking attending to the attendee
    party_id: u64, // ID linking attending to the party
}


