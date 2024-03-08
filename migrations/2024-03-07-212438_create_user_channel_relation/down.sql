-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS user_channel_relation_user_id_idx;

DROP INDEX IF EXISTS user_channel_relation_channel_id_idx;
DROP TABLE IF EXISTS user_channel;

DROP TYPE IF EXISTS user_role;

