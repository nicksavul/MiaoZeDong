CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT,
                                  username TEXT NOT NULL,
                                  guest_rating BIGINT DEFAULT NULL,
                                  host_rating BIGINT DEFAULT NULL);

CREATE TABLE IF NOT EXISTS user_profile_pictures (id INTEGER PRIMARY KEY,
                                                  jpg_pfp BLOB DEFAULT NULL);

CREATE TABLE IF NOT EXISTS authorized_users (id INTEGER PRIMARY KEY,
                                             sec_hash TEXT NOT NULL,
                                             auth_key TEXT NOT NULL,
                                             last_accessed BIGINT NOT NULL);

CREATE TABLE IF NOT EXISTS parties (id INTEGER PRIMARY KEY AUTOINCREMENT,
                                  host_id BIGINT NOT NULL,
                                  title TEXT,
                                  description TEXT,
                                  latitude REAL NOT NULL,
                                  longitude REAL NOT NULL,
                                  capacity BIGINT NOT NULL,
                                  attendees BIGINT DEFAULT 1,
                                  start_time BIGINT NOT NULL,
                                  visibility BIGINT NOT NULL DEFAULT 0);

CREATE TABLE IF NOT EXISTS attendings (id INTEGER PRIMARY KEY AUTOINCREMENT,
                                       attendee_id BIGINT NOT NULL,
                                       party_id BIGINT NOT NULL);