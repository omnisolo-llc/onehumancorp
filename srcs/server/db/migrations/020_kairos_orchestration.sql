-- In SQLite, ALTER TABLE ADD COLUMN does not support IF NOT EXISTS.
-- Since shared_tasks lacks mission_id and the prompt expects it for autodream consolidation mapping,
-- we add it. We will accept that this might fail if ran twice, but migrations run sequentially.
ALTER TABLE shared_tasks ADD COLUMN mission_id TEXT;
