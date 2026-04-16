-- +goose Up
-- +goose StatementBegin
-- SQLite has a limited ALTER TABLE, but adding a column is supported.
-- However, we only need to add it if it doesn't already exist, which standard SQL doesn't do simply without PL/pgSQL block,
-- but in our SQLite wrapper/Goose we can just add the column for SQLite specifically since Postgres was fine.
-- Wait, actually `013_shared_tasks.sql` DID have `locked_until` but `20260410_kairos_tasks.sql` dropped and recreated the table and FORGOT `locked_until`. So Postgres ALSO lost `locked_until` in migration 20260410_kairos_tasks.sql!
ALTER TABLE swarm_tasks ADD COLUMN locked_until TIMESTAMP;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE swarm_tasks DROP COLUMN locked_until;
-- +goose StatementEnd
