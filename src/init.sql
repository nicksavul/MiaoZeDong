CREATE TABLE IF NOT EXISTS users (id BIGINT PRIMARY KEY);

CREATE TABLE IF NOT EXISTS authorized_users (id BIGINT PRIMARY KEY,
                                             sec_hash TEXT NOT NULL,
                                             auth_key TEXT NOT NULL,
                                             last_accessed BIGINT NOT NULL);

CREATE TABLE IF NOT EXISTS parties (id BIGINT PRIMARY KEY,
                                  host_id BIGINT NOT NULL,
                                  title TEXT,
                                  description TEXT,
                                  latitude REAL NOT NULL,
                                  longitude REAL NOT NULL,
                                  capacity BIGINT NOT NULL,
                                  attendees BIGINT DEFAULT 1,
                                  start_time BIGINT NOT NULL);

CREATE TABLE IF NOT EXISTS attendings (id BIGINT PRIMARY KEY,
                                       attendee_id BIGINT NOT NULL,
                                       party_id BIGINT NOT NULL);