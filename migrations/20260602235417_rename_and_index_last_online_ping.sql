ALTER TABLE streamers ADD COLUMN IF NOT EXISTS last_online_ping TIMESTAMP WITH TIME ZONE;

DROP INDEX IF EXISTS idx_streamers_last_online_ping;
CREATE INDEX idx_streamers_last_online_ping ON streamers (last_online_ping DESC NULLS LAST);
