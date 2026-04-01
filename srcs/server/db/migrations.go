package db

import "embed"

//go:embed migrations/*.sql migrations_sqlite/*.sql
var migrationsFS embed.FS
