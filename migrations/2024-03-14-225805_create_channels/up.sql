-- Channels
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE channels (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(32) NOT NULL
)