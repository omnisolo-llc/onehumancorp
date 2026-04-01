package db

import (
	"context"
)

// DatabaseProvider defines a unified interface for database interactions,
// abstracting both PostgreSQL (Cloud) and SQLite (Standalone) implementations.
// It uses standard context-aware methods for execution and querying.
type DatabaseProvider interface {
	Exec(ctx context.Context, query string, args ...any) (int64, error)
	Query(ctx context.Context, query string, args ...any) (Rows, error)
	QueryRow(ctx context.Context, query string, args ...any) Row
	Begin(ctx context.Context) (Tx, error)
	Close()
	RunMigrations(ctx context.Context) error
	IsSQLite() bool
}

// Rows interface abstracts sql.Rows and pgx.Rows
type Rows interface {
	Next() bool
	Scan(dest ...any) error
	Close()
	Err() error
}

// Row interface abstracts sql.Row and pgx.Row
type Row interface {
	Scan(dest ...any) error
}

// Tx interface abstracts sql.Tx and pgx.Tx
type Tx interface {
	Exec(ctx context.Context, query string, args ...any) (int64, error)
	Query(ctx context.Context, query string, args ...any) (Rows, error)
	QueryRow(ctx context.Context, query string, args ...any) Row
	Commit(ctx context.Context) error
	Rollback(ctx context.Context) error
}
