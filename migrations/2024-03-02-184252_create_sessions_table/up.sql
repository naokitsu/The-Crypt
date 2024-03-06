-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE sessions (
    user_id UUID NOT NULL,
    key bytea NOT NULL,
    nonce bytea NOT NULL
);

ALTER TABLE sessions ADD PRIMARY KEY (key);
ALTER TABLE sessions ADD FOREIGN KEY (user_id) REFERENCES users (id);

CREATE UNIQUE INDEX sessions_id_idx ON sessions (key);

