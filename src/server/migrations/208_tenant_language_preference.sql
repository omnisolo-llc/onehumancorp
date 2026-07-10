ALTER TABLE tenants ADD COLUMN IF NOT EXISTS language_preference TEXT DEFAULT 'English';
