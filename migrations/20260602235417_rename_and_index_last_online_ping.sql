DO $$
BEGIN
  IF EXISTS(SELECT *
    FROM information_schema.columns
    WHERE table_name='streamers' and column_name='last_overlay_ping')
  THEN
      ALTER TABLE streamers RENAME COLUMN last_overlay_ping TO last_online_ping;
  END IF;
END $$;

ALTER TABLE streamers ADD COLUMN IF NOT EXISTS last_online_ping TIMESTAMP WITH TIME ZONE;

DROP INDEX IF EXISTS idx_streamers_last_online_ping;
CREATE INDEX idx_streamers_last_online_ping ON streamers (last_online_ping DESC NULLS LAST);
