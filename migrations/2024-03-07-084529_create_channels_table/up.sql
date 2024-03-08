-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE channels (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR(32) NOT NULL
);

ALTER TABLE channels ADD PRIMARY KEY (id);