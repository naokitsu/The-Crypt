-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS users_username_idx;
DROP INDEX IF EXISTS users_id_idx;
DROP TABLE IF EXISTS users;
DROP EXTENSION IF EXISTS "uuid-ossp";