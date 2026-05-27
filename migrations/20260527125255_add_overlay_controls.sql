-- Add overlay controls columns
ALTER TABLE streamers
    ADD COLUMN overlay_paused BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN overlay_sound_enabled BOOLEAN NOT NULL DEFAULT TRUE;
