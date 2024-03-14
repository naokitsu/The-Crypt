-- Members
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE member_role AS ENUM ('owner', 'admin', 'member');
CREATE TABLE members (
    user_id UUID NOT NULL REFERENCES users(id),
    channel_id UUID NOT NULL REFERENCES channels(id),
    role member_role NOT NULL DEFAULT 'member'
);

ALTER TABLE members ADD PRIMARY KEY (user_id, channel_id);
CREATE INDEX members_user_id_idx ON members(user_id);
CREATE INDEX members_channel_id_idx ON members(channel_id);