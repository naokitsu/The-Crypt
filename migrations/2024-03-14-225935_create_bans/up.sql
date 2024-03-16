CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE bans
(
    user_id    UUID NOT NULL REFERENCES users (id),
    channel_id UUID NOT NULL REFERENCES channels (id)
);

ALTER TABLE bans
    ADD PRIMARY KEY (user_id, channel_id);
CREATE INDEX bans_user_id_idx ON bans (user_id);
CREATE INDEX bans_channel_id_idx ON bans (channel_id)