-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE channels (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR(32) NOT NULL,
    admin_id UUID NOT NULL
);

ALTER TABLE channels ADD PRIMARY KEY (id);
ALTER TABLE channels ADD FOREIGN KEY (admin_id) REFERENCES users (id)