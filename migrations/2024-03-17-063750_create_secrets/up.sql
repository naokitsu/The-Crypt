CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE secrets (
    user_id UUID NOT NULL PRIMARY KEY REFERENCES users (id),
    salted_hash bytea NOT NULL
);