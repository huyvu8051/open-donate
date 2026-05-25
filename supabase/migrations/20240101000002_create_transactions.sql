-- Create transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    streamer_id INTEGER NOT NULL REFERENCES streamers(id) ON DELETE CASCADE,
    donor_name VARCHAR NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    message TEXT,
    payment_method VARCHAR NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Index for faster queries by streamer_id + ordering
CREATE INDEX IF NOT EXISTS idx_transactions_streamer_id ON transactions(streamer_id, id DESC);
