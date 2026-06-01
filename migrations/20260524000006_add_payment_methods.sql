ALTER TABLE streamers ADD COLUMN IF NOT EXISTS payment_methods TEXT[] DEFAULT '{"Mock Auto", "Mock Manual"}';
