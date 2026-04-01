package db

import (
	"context"
	"embed"
	"fmt"
	"io/fs"
	"log/slog"
	"os"
	"sort"
	"strings"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

var (
	//go:embed migrations/*.sql
	migrationsFS embed.FS
)

// PostgresProvider implements DatabaseProvider for PostgreSQL.
type PostgresProvider struct {
	pool *pgxpool.Pool
}

// New creates a new Postgres connection pool from DATABASE_URL.
// Returns nil if DATABASE_URL is not set (enabling zero-dep local mode).
func New(ctx context.Context) (*PostgresProvider, error) {
	dsn := os.Getenv("DATABASE_URL")
	if dsn == "" {
		return nil, nil // no Postgres configured — use in-memory fallback
	}

	pool, err := pgxpool.New(ctx, dsn)
	if err != nil {
		return nil, fmt.Errorf("db: connect to postgres: %w", err)
	}

	if err := pool.Ping(ctx); err != nil {
		pool.Close()
		return nil, fmt.Errorf("db: ping postgres: %w", err)
	}

	slog.Info("db: connected to postgres", "dsn", redactDSN(dsn))
	return &PostgresProvider{pool: pool}, nil
}

// Exec executes a query without returning any rows.
func (p *PostgresProvider) Exec(ctx context.Context, query string, args ...any) (int64, error) {
	cmdTag, err := p.pool.Exec(ctx, query, args...)
	if err != nil {
		return 0, err
	}
	return cmdTag.RowsAffected(), nil
}

// Query executes a query that returns rows.
func (p *PostgresProvider) Query(ctx context.Context, query string, args ...any) (Rows, error) {
	rows, err := p.pool.Query(ctx, query, args...)
	if err != nil {
		return nil, err
	}
	return &pgxRows{rows: rows}, nil
}

// QueryRow executes a query that is expected to return at most one row.
func (p *PostgresProvider) QueryRow(ctx context.Context, query string, args ...any) Row {
	return p.pool.QueryRow(ctx, query, args...)
}

// Begin starts a transaction.
func (p *PostgresProvider) Begin(ctx context.Context) (Tx, error) {
	tx, err := p.pool.Begin(ctx)
	if err != nil {
		return nil, err
	}
	return &pgxTx{tx: tx}, nil
}

// Close closes the connection pool.
func (p *PostgresProvider) Close() {
	if p.pool != nil {
		p.pool.Close()
	}
}

// IsSQLite returns false for PostgresProvider.
func (p *PostgresProvider) IsSQLite() bool {
	return false
}

// pgxRows wraps pgx.Rows to satisfy db.Rows interface.
type pgxRows struct {
	rows pgx.Rows
}

func (r *pgxRows) Next() bool {
	return r.rows.Next()
}

func (r *pgxRows) Scan(dest ...any) error {
	return r.rows.Scan(dest...)
}

func (r *pgxRows) Close() {
	r.rows.Close()
}

func (r *pgxRows) Err() error {
	return r.rows.Err()
}

// pgxTx wraps pgx.Tx to satisfy db.Tx interface.
type pgxTx struct {
	tx pgx.Tx
}

func (t *pgxTx) Exec(ctx context.Context, query string, args ...any) (int64, error) {
	cmdTag, err := t.tx.Exec(ctx, query, args...)
	if err != nil {
		return 0, err
	}
	return cmdTag.RowsAffected(), nil
}

func (t *pgxTx) Query(ctx context.Context, query string, args ...any) (Rows, error) {
	rows, err := t.tx.Query(ctx, query, args...)
	if err != nil {
		return nil, err
	}
	return &pgxRows{rows: rows}, nil
}

func (t *pgxTx) QueryRow(ctx context.Context, query string, args ...any) Row {
	return t.tx.QueryRow(ctx, query, args...)
}

func (t *pgxTx) Commit(ctx context.Context) error {
	return t.tx.Commit(ctx)
}

func (t *pgxTx) Rollback(ctx context.Context) error {
	return t.tx.Rollback(ctx)
}

// RunMigrations executes all embedded SQL migrations, sorted
// lexicographically.  Each migration is run inside a transaction.
// A simple `schema_migrations` table tracks which files have already been
// applied.
func (p *PostgresProvider) RunMigrations(ctx context.Context) error {
	// Ensure tracking table exists.
	if _, err := p.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS schema_migrations (
			filename TEXT PRIMARY KEY,
			applied_at TIMESTAMPTZ DEFAULT NOW()
		)
	`); err != nil {
		return fmt.Errorf("db: create schema_migrations: %w", err)
	}

	entries, err := fs.ReadDir(migrationsFS, "migrations")
	if err != nil {
		return fmt.Errorf("db: read embedded migrations: %w", err)
	}

	var files []string
	for _, e := range entries {
		if !e.IsDir() && strings.HasSuffix(e.Name(), ".sql") {
			files = append(files, e.Name())
		}
	}
	sort.Strings(files)

	for _, f := range files {
		// Check if already applied.
		var count int
		if err := p.QueryRow(ctx, "SELECT COUNT(*) FROM schema_migrations WHERE filename = $1", f).Scan(&count); err != nil {
			return fmt.Errorf("db: check migration %s: %w", f, err)
		}
		if count > 0 {
			continue
		}

		sql, err := fs.ReadFile(migrationsFS, "migrations/"+f)
		if err != nil {
			return fmt.Errorf("db: read migration %s: %w", f, err)
		}

		tx, err := p.Begin(ctx)
		if err != nil {
			return fmt.Errorf("db: begin tx for %s: %w", f, err)
		}

		if _, err := tx.Exec(ctx, string(sql)); err != nil {
			_ = tx.Rollback(ctx)
			return fmt.Errorf("db: exec migration %s: %w", f, err)
		}

		if _, err := tx.Exec(ctx, "INSERT INTO schema_migrations (filename) VALUES ($1)", f); err != nil {
			_ = tx.Rollback(ctx)
			return fmt.Errorf("db: record migration %s: %w", f, err)
		}

		if err := tx.Commit(ctx); err != nil {
			return fmt.Errorf("db: commit migration %s: %w", f, err)
		}

		slog.Info("db: applied migration", "file", f)
	}

	return nil
}

// redactDSN hides the password from a DSN for safe logging.
func redactDSN(dsn string) string {
	if i := strings.Index(dsn, "://"); i >= 0 {
		rest := dsn[i+3:]
		if at := strings.Index(rest, "@"); at >= 0 {
			if colon := strings.Index(rest[:at], ":"); colon >= 0 {
				return dsn[:i+3] + rest[:colon+1] + "****" + rest[at:]
			}
		}
	}
	return dsn
}
