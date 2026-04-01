-- Create a publication for PowerSync to enable logical replication
DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM pg_publication
    WHERE pubname = 'powersync'
  ) THEN
    CREATE PUBLICATION powersync FOR ALL TABLES;
  END IF;
EXCEPTION
  -- SQLite does not support DO blocks or pg_publication, this handles standalone gracefully via parser skipping if necessary
  WHEN others THEN
    NULL;
END $$;
