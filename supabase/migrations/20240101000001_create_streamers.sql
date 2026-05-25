-- Create streamers table
CREATE TABLE IF NOT EXISTS streamers (
    id SERIAL PRIMARY KEY,
    username VARCHAR UNIQUE NOT NULL,
    display_name VARCHAR NOT NULL,
    avatar_url VARCHAR NOT NULL,
    bio TEXT NOT NULL,
    is_live BOOLEAN NOT NULL DEFAULT FALSE
);
