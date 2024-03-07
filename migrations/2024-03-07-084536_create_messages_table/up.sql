-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE messages (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    channel_id UUID NOT NULL,
    content VARCHAR(1024) NOT NULL
);

ALTER TABLE messages ADD PRIMARY KEY (id);
ALTER TABLE messages ADD FOREIGN KEY (user_id) REFERENCES users (id);
ALTER TABLE messages ADD FOREIGN KEY (channel_id) REFERENCES channels (id)