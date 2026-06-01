-- Add overlay controls columns
ALTER TABLE streamers
    ADD COLUMN IF NOT EXISTS overlay_paused BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS overlay_sound_enabled BOOLEAN NOT NULL DEFAULT TRUE;
