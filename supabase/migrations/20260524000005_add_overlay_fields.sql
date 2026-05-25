ALTER TABLE streamers ADD COLUMN overlay_token VARCHAR(255) UNIQUE;
ALTER TABLE streamers ADD COLUMN active_overlay_session VARCHAR(255);

-- Generate initial tokens for existing streamers using pgcrypto gen_random_uuid()
UPDATE streamers SET overlay_token = gen_random_uuid() WHERE overlay_token IS NULL;
ALTER TABLE streamers ALTER COLUMN overlay_token SET NOT NULL;
