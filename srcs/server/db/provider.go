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

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/jackc/pgx/v5/stdlib"
	_ "modernc.org/sqlite"
)

// Provider abstracts the underlying database connection.
type Provider interface {
	ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error)
	QueryContext(ctx context.Context, query string, args ...any) (*sql.Rows, error)
	QueryRowContext(ctx context.Context, query string, args ...any) *sql.Row
	BeginTx(ctx context.Context, opts *sql.TxOptions) (*sql.Tx, error)
	Close() error
	RunMigrations(ctx context.Context) error
	IsPostgres() bool
}

// Ensure implementations satisfy Provider
var _ Provider = (*pgProvider)(nil)
var _ Provider = (*sqliteProvider)(nil)

type pgProvider struct {
	pool *pgxpool.Pool
	db   *sql.DB
}

func (p *pgProvider) ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error) {
	return p.db.ExecContext(ctx, query, args...)
}

func (p *pgProvider) QueryContext(ctx context.Context, query string, args ...any) (*sql.Rows, error) {
	return p.db.QueryContext(ctx, query, args...)
}

func (p *pgProvider) QueryRowContext(ctx context.Context, query string, args ...any) *sql.Row {
	return p.db.QueryRowContext(ctx, query, args...)
}

func (p *pgProvider) BeginTx(ctx context.Context, opts *sql.TxOptions) (*sql.Tx, error) {
	return p.db.BeginTx(ctx, opts)
}

func (p *pgProvider) Close() error {
	p.pool.Close()
	return p.db.Close()
}

func (p *pgProvider) IsPostgres() bool {
	return true
}

func (p *pgProvider) Pool() *pgxpool.Pool {
	return p.pool
}

type sqliteProvider struct {
	db *sql.DB
}

func (p *sqliteProvider) ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error) {
	return p.db.ExecContext(ctx, query, args...)
}

func (p *sqliteProvider) QueryContext(ctx context.Context, query string, args ...any) (*sql.Rows, error) {
	return p.db.QueryContext(ctx, query, args...)
}

func (p *sqliteProvider) QueryRowContext(ctx context.Context, query string, args ...any) *sql.Row {
	return p.db.QueryRowContext(ctx, query, args...)
}

func (p *sqliteProvider) BeginTx(ctx context.Context, opts *sql.TxOptions) (*sql.Tx, error) {
	return p.db.BeginTx(ctx, opts)
}

func (p *sqliteProvider) Close() error {
	return p.db.Close()
}

func (p *sqliteProvider) IsPostgres() bool {
	return false
}

// NewProvider creates a generic database provider.
func NewProvider(ctx context.Context) (Provider, error) {
	dsn := os.Getenv("DATABASE_URL")
	standalone := os.Getenv("OHC_STANDALONE") == "true"

	if dsn == "" || strings.HasPrefix(dsn, "sqlite://") {
		if standalone || strings.HasPrefix(dsn, "sqlite://") {
			// Fallback to SQLite
			dbPath := ".agent-task/swarm.db"
			if strings.HasPrefix(dsn, "sqlite://") {
				dbPath = strings.TrimPrefix(dsn, "sqlite://")
			}

			db, err := sql.Open("sqlite", dbPath)
			if err != nil {
				return nil, fmt.Errorf("db: connect to sqlite: %w", err)
			}
			if err := db.PingContext(ctx); err != nil {
				db.Close()
				return nil, fmt.Errorf("db: ping sqlite: %w", err)
			}
			slog.Info("db: connected to sqlite", "path", dbPath)
			return &sqliteProvider{db: db}, nil
		}
		return nil, nil // No postgres configured - in memory fallback
	}

	// Postgres connection
	pool, err := pgxpool.New(ctx, dsn)
	if err != nil {
		return nil, fmt.Errorf("db: connect to postgres: %w", err)
	}
	if err := pool.Ping(ctx); err != nil {
		pool.Close()
		return nil, fmt.Errorf("db: ping postgres: %w", err)
	}

	db := stdlib.OpenDBFromPool(pool)
	if err := db.PingContext(ctx); err != nil {
		db.Close()
		pool.Close()
		return nil, fmt.Errorf("db: ping postgres sql.DB: %w", err)
	}

	slog.Info("db: connected to postgres", "dsn", redactDSN(dsn))
	return &pgProvider{pool: pool, db: db}, nil
}

var (
	//go:embed migrations/*.sql
	migrationsFS embed.FS
)

func (p *pgProvider) RunMigrations(ctx context.Context) error {
	// Ensure tracking table exists.
	if _, err := p.ExecContext(ctx, `
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
		var count int
		if err := p.QueryRowContext(ctx, "SELECT COUNT(*) FROM schema_migrations WHERE filename = $1", f).Scan(&count); err != nil {
			return fmt.Errorf("db: check migration %s: %w", f, err)
		}
		if count > 0 {
			continue
		}

		sqlStr, err := fs.ReadFile(migrationsFS, "migrations/"+f)
		if err != nil {
			return fmt.Errorf("db: read migration %s: %w", f, err)
		}

		tx, err := p.BeginTx(ctx, nil)
		if err != nil {
			return fmt.Errorf("db: begin tx for %s: %w", f, err)
		}

		if _, err := tx.ExecContext(ctx, string(sqlStr)); err != nil {
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

		slog.Info("db: applied pg migration", "file", f)
	}

	return nil
}

var (
	//go:embed migrations_sqlite/*.sql
	migrationsSqliteFS embed.FS
)

func (p *sqliteProvider) RunMigrations(ctx context.Context) error {
	// Ensure tracking table exists.
	if _, err := p.ExecContext(ctx, `
		CREATE TABLE IF NOT EXISTS schema_migrations (
			filename TEXT PRIMARY KEY,
			applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`); err != nil {
		return fmt.Errorf("db: create schema_migrations: %w", err)
	}

	entries, err := fs.ReadDir(migrationsSqliteFS, "migrations_sqlite")
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
		var count int
		if err := p.QueryRowContext(ctx, "SELECT COUNT(*) FROM schema_migrations WHERE filename = ?", f).Scan(&count); err != nil {
			return fmt.Errorf("db: check migration %s: %w", f, err)
		}
		if count > 0 {
			continue
		}

		sqlStr, err := fs.ReadFile(migrationsSqliteFS, "migrations_sqlite/"+f)
		if err != nil {
			return fmt.Errorf("db: read migration %s: %w", f, err)
		}

		tx, err := p.BeginTx(ctx, nil)
		if err != nil {
			return fmt.Errorf("db: begin tx for %s: %w", f, err)
		}

		if _, err := tx.ExecContext(ctx, string(sqlStr)); err != nil {
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

		slog.Info("db: applied sqlite migration", "file", f)
	}

	return nil
}

func (p *sqliteProvider) DB() *sql.DB {
	return p.db
}

func (p *pgProvider) DB() *sql.DB {
	return p.db
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
