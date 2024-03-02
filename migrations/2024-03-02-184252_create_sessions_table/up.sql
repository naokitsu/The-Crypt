-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE sessions (
                       id VARCHAR(72) NOT NULL,
                       user_id UUID NOT NULL
);

ALTER TABLE sessions ADD PRIMARY KEY (id);
ALTER TABLE sessions ADD FOREIGN KEY (user_id) REFERENCES users (id);

CREATE UNIQUE INDEX sessions_id_idx ON sessions (id);

