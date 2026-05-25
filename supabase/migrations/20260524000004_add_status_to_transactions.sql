-- Add status column for overlay display flow
ALTER TABLE transactions
    ADD COLUMN IF NOT EXISTS status VARCHAR NOT NULL DEFAULT 'DISPLAYED';

-- Backfill any NULLs from older deployments (defensive)
UPDATE transactions
SET status = 'DISPLAYED'
WHERE status IS NULL;

-- Index to fetch READY_FOR_DISPLAY quickly
CREATE INDEX IF NOT EXISTS idx_transactions_streamer_status_id
    ON transactions(streamer_id, status, id ASC);

