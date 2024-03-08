-- Your SQL goes here
CREATE TYPE user_role AS ENUM ('admin', 'member');

CREATE TABLE user_channel (
    user_id UUID NOT NULL,
    channel_id UUID NOT NULL,
    role user_role NOT NULL
);

ALTER TABLE user_channel ADD PRIMARY KEY (user_id, channel_id);
ALTER TABLE user_channel ADD FOREIGN KEY (user_id) REFERENCES users (id);
ALTER TABLE user_channel ADD FOREIGN KEY (channel_id) REFERENCES channels (id);

CREATE INDEX user_channel_user_id_idx ON user_channel (user_id);
CREATE INDEX user_channel_channel_id_idx ON user_channel (channel_id);