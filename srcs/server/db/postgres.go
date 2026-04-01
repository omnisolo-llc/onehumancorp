package db

import (
	"context"
	"database/sql"
	"embed"
	"fmt"
	"io/fs"
	"log/slog"
	"os"
	"sort"
	"strings"

	_ "github.com/jackc/pgx/v5/stdlib"
)

var (
	//go:embed migrations/*.sql
	migrationsFS embed.FS
)

// Pool wraps a Provider with migration support.
type Pool struct {
	Provider
}

// New creates a new Postgres connection pool from DATABASE_URL.
// Returns nil if DATABASE_URL is not set (enabling zero-dep local mode).
func New(ctx context.Context) (*Pool, error) {
	dsn := os.Getenv("DATABASE_URL")
	if dsn == "" {
		return nil, nil // no Postgres configured — use in-memory fallback
	}

	db, err := sql.Open("pgx", dsn)
	if err != nil {
		return nil, fmt.Errorf("db: connect to postgres: %w", err)
	}

	if err := db.PingContext(ctx); err != nil {
		db.Close()
		return nil, fmt.Errorf("db: ping postgres: %w", err)
	}

	slog.Info("db: connected to postgres", "dsn", redactDSN(dsn))
	return &Pool{Provider: NewSQLProvider(db, true)}, nil
}

// NewSQLite creates a new SQLite connection pool.
func NewSQLite(ctx context.Context, path string) (*Pool, error) {
	db, err := sql.Open("sqlite", path)
	if err != nil {
		return nil, fmt.Errorf("db: connect to sqlite: %w", err)
	}

	// Basic configuration for SQLite correctness in concurrent environments.
	db.SetMaxOpenConns(1) // Avoid SQLITE_BUSY by serializing writes if needed, or rely on WAL.
	if _, err := db.ExecContext(ctx, "PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;"); err != nil {
		db.Close()
		return nil, fmt.Errorf("db: config sqlite: %w", err)
	}

	if err := db.PingContext(ctx); err != nil {
		db.Close()
		return nil, fmt.Errorf("db: ping sqlite: %w", err)
	}

	slog.Info("db: connected to sqlite", "path", path)
	return &Pool{Provider: NewSQLProvider(db, false)}, nil
}

// RunMigrations executes all embedded SQL migrations.
func (p *Pool) RunMigrations(ctx context.Context) error {
	// Ensure tracking table exists.
	if _, err := p.ExecContext(ctx, `
		CREATE TABLE IF NOT EXISTS schema_migrations (
			filename TEXT PRIMARY KEY,
			applied_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
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
		if err := p.QueryRowContext(ctx, "SELECT COUNT(*) FROM schema_migrations WHERE filename = $1", f).Scan(&count); err != nil {
			return fmt.Errorf("db: check migration %s: %w", f, err)
		}
		if count > 0 {
			continue
		}

		sqlStrBytes, err := fs.ReadFile(migrationsFS, "migrations/"+f)
		if err != nil {
			return fmt.Errorf("db: read migration %s: %w", f, err)
		}

		sqlStr := string(sqlStrBytes)

		// Very simple adaptation for SQLite for arrays or bigserial since modernc.org/sqlite doesn't understand them.
		if !p.IsPostgres() {
			sqlStr = strings.ReplaceAll(sqlStr, "BIGSERIAL", "INTEGER")
			sqlStr = strings.ReplaceAll(sqlStr, "TIMESTAMPTZ", "DATETIME")
			sqlStr = strings.ReplaceAll(sqlStr, "TEXT[]", "TEXT") // We'll store JSON for TEXT[]
		}

		tx, err := p.BeginTx(ctx, nil)
		if err != nil {
			return fmt.Errorf("db: begin tx for %s: %w", f, err)
		}

		if _, err := tx.ExecContext(ctx, sqlStr); err != nil {
			_ = tx.Rollback()
			return fmt.Errorf("db: exec migration %s: %w", f, err)
		}

		if _, err := tx.ExecContext(ctx, "INSERT INTO schema_migrations (filename) VALUES ($1)", f); err != nil {
			_ = tx.Rollback()
			return fmt.Errorf("db: record migration %s: %w", f, err)
		}

		if err := tx.Commit(); err != nil {
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
