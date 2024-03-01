-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4() NOT NULL,
    username VARCHAR(64) NOT NULL,
    salt VARCHAR(8) NOT NULL,
    password_hash VARCHAR(16) NOT NULL
);

ALTER TABLE users ADD PRIMARY KEY (id);

CREATE UNIQUE INDEX users_username_idx ON users (username);
CREATE UNIQUE INDEX users_id_idx ON users (id);

