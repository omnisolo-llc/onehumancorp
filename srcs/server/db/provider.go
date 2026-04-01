package db

import (
	"context"
	"database/sql"
	"fmt"
	"io/fs"
	"log/slog"
	"os"
	"sort"
	"strings"

	"github.com/jackc/pgx/v5/pgxpool"
	_ "modernc.org/sqlite"
)

// Provider abstracts the database connection to support both PostgreSQL (via pgxpool) and SQLite (via database/sql).
type Provider struct {
	Type   string
	PgPool *pgxpool.Pool
	Sqlite *sql.DB
}

// NewProvider creates a new Provider based on DATABASE_URL.
func NewProvider(ctx context.Context) (*Provider, error) {
	dsn := os.Getenv("DATABASE_URL")

	if dsn == "" || strings.HasPrefix(dsn, "sqlite://") {
		// Default to SQLite
		path := strings.TrimPrefix(dsn, "sqlite://")
		if path == "" {
			path = os.Getenv("HOME") + "/.openclaw/ohc.db"
		}

		db, err := sql.Open("sqlite", path)
		if err != nil {
			return nil, fmt.Errorf("db: connect to sqlite: %w", err)
		}
		if err := db.PingContext(ctx); err != nil {
			db.Close()
			return nil, fmt.Errorf("db: ping sqlite: %w", err)
		}

		slog.Info("db: connected to sqlite", "path", path)
		return &Provider{
			Type:   "sqlite",
			Sqlite: db,
		}, nil
	}

	// PostgreSQL
	pool, err := pgxpool.New(ctx, dsn)
	if err != nil {
		return nil, fmt.Errorf("db: connect to postgres: %w", err)
	}
	if err := pool.Ping(ctx); err != nil {
		pool.Close()
		return nil, fmt.Errorf("db: ping postgres: %w", err)
	}

	slog.Info("db: connected to postgres", "dsn", redactDSN(dsn))
	return &Provider{
		Type:   "postgres",
		PgPool: pool,
	}, nil
}

// Close closes the underlying connection.
func (p *Provider) Close() {
	if p.PgPool != nil {
		p.PgPool.Close()
	}
	if p.Sqlite != nil {
		p.Sqlite.Close()
	}
}

// RunMigrations executes all embedded SQL migrations, sorted lexicographically.
// Each migration is run inside a transaction.
func (p *Provider) RunMigrations(ctx context.Context) error {
	dir := "migrations"
	if p.Type == "sqlite" {
		dir = "migrations_sqlite"
	}

	entries, err := fs.ReadDir(migrationsFS, dir)
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

	if p.Type == "postgres" {
		return p.runPostgresMigrations(ctx, files)
	} else {
		return p.runSqliteMigrations(ctx, files)
	}
}

func (p *Provider) runPostgresMigrations(ctx context.Context, files []string) error {
	if _, err := p.PgPool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS schema_migrations (
			filename TEXT PRIMARY KEY,
			applied_at TIMESTAMPTZ DEFAULT NOW()
		)
	`); err != nil {
		return fmt.Errorf("db: create schema_migrations: %w", err)
	}

	for _, f := range files {
		var count int
		if err := p.PgPool.QueryRow(ctx, "SELECT COUNT(*) FROM schema_migrations WHERE filename = $1", f).Scan(&count); err != nil {
			return fmt.Errorf("db: check migration %s: %w", f, err)
		}
		if count > 0 {
			continue
		}

		sqlBytes, err := fs.ReadFile(migrationsFS, "migrations/"+f)
		if err != nil {
			return fmt.Errorf("db: read migration %s: %w", f, err)
		}

		tx, err := p.PgPool.Begin(ctx)
		if err != nil {
			return fmt.Errorf("db: begin tx for %s: %w", f, err)
		}

		if _, err := tx.Exec(ctx, string(sqlBytes)); err != nil {
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

func (p *Provider) runSqliteMigrations(ctx context.Context, files []string) error {
	if _, err := p.Sqlite.ExecContext(ctx, `
		CREATE TABLE IF NOT EXISTS schema_migrations (
			filename TEXT PRIMARY KEY,
			applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`); err != nil {
		return fmt.Errorf("db: create schema_migrations: %w", err)
	}

	for _, f := range files {
		var count int
		if err := p.Sqlite.QueryRowContext(ctx, "SELECT COUNT(*) FROM schema_migrations WHERE filename = ?", f).Scan(&count); err != nil {
			return fmt.Errorf("db: check migration %s: %w", f, err)
		}
		if count > 0 {
			continue
		}

		sqlBytes, err := fs.ReadFile(migrationsFS, "migrations_sqlite/"+f)
		if err != nil {
			return fmt.Errorf("db: read migration %s: %w", f, err)
		}

		query := string(sqlBytes)

		tx, err := p.Sqlite.BeginTx(ctx, nil)
		if err != nil {
			return fmt.Errorf("db: begin tx for %s: %w", f, err)
		}

		if _, err := tx.ExecContext(ctx, query); err != nil {
			_ = tx.Rollback()
			return fmt.Errorf("db: exec migration %s: %w", f, err)
		}

		if _, err := tx.ExecContext(ctx, "INSERT INTO schema_migrations (filename) VALUES (?)", f); err != nil {
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
