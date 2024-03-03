-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4() NOT NULL,
    username VARCHAR(64) NOT NULL
    /*salted_hash bytea NOT NULL*/
    /*
    I do not want to store such sensitive data inside users since these entries are quite often get deserialized into
    user objects. I know that rust is memory safe, but I would rather prefer to not have passwords in the rocket
    memory if i don't need them.
    */
);

ALTER TABLE users ADD PRIMARY KEY (id);

CREATE UNIQUE INDEX users_username_idx ON users (username);
CREATE UNIQUE INDEX users_id_idx ON users (id);

