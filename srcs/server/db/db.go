package db

import _ "embed"

//go:embed migrations/014_shared_tasks.sql
var Migration014SharedTasks string
