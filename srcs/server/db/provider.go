package db

import (
	"context"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgconn"
)

// DatabaseProvider defines the interface for database operations,
// supporting both PostgreSQL and SQLite implementations.
type DatabaseProvider interface {
	Exec(ctx context.Context, sql string, arguments ...any) (pgconn.CommandTag, error)
	Query(ctx context.Context, sql string, optionsAndArgs ...any) (pgx.Rows, error)
	QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) pgx.Row
	Begin(ctx context.Context) (pgx.Tx, error)
	Close()
	Ping(ctx context.Context) error
	RunMigrations(ctx context.Context) error
}

// Ensure Pool implements DatabaseProvider
var _ DatabaseProvider = (*Pool)(nil)

// ErrNoRows is a common error for when a query returns no rows.
var ErrNoRows = pgx.ErrNoRows
