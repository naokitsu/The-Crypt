-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE secrets (
    user_id UUID NOT NULL,
    salt bytea NOT NULL
);

ALTER TABLE secrets ADD PRIMARY KEY (user_id);
ALTER TABLE secrets ADD FOREIGN KEY (user_id) REFERENCES users (id);
