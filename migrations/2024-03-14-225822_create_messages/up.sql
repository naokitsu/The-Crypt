-- Messages
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE messages (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    channel_id UUID NOT NULL REFERENCES channels(id),
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX messages_channel_id_idx ON messages(channel_id);
CREATE INDEX messages_created_at_idx ON messages(created_at);