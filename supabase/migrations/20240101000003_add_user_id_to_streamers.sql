-- Add user_id to streamers table
ALTER TABLE streamers ADD COLUMN user_id VARCHAR;
ALTER TABLE streamers ADD CONSTRAINT streamers_user_id_key UNIQUE (user_id);
