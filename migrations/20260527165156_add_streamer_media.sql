CREATE TABLE IF NOT EXISTS streamer_media (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    streamer_id INTEGER NOT NULL REFERENCES streamers(id) ON DELETE CASCADE,
    file_name VARCHAR NOT NULL,
    file_url VARCHAR NOT NULL,
    size_bytes INT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

ALTER TABLE streamers ADD COLUMN IF NOT EXISTS selected_media_id UUID REFERENCES streamer_media(id) ON DELETE SET NULL;
ALTER TABLE streamers ADD COLUMN IF NOT EXISTS fallback_media_file VARCHAR NOT NULL DEFAULT '/default_donate.mp3';
