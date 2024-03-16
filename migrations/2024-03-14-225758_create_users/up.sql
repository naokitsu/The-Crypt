-- Users
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users
(
    id        UUID        NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    name      VARCHAR(32) NOT NULL,
    developer BOOLEAN     NOT NULL             DEFAULT FALSE
);