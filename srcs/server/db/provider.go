package db

import (
	"context"
	"database/sql"
	"embed"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"sort"
	"strings"

	"github.com/jackc/pgx/v5/pgxpool"
	_ "modernc.org/sqlite"
)

var (
	//go:embed migrations/*.sql
	migrationsFS embed.FS
)

// Provider abstracts the underlying database connection, supporting both
// PostgreSQL (pgxpool) and SQLite (database/sql).
type Provider struct {
	PgPool  *pgxpool.Pool
	Sqlite  *sql.DB
	Dialect string
}

// NewProvider initializes the correct database based on the environment.
// It prioritizes DATABASE_URL. If empty or prefixed with sqlite://, it uses SQLite.
// Otherwise, it attempts a PostgreSQL connection.
func NewProvider(ctx context.Context) (*Provider, error) {
	dsn := os.Getenv("DATABASE_URL")
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"

	if dsn == "" || strings.HasPrefix(dsn, "sqlite://") {
		// Use SQLite
		var dbPath string
		if dsn != "" {
			dbPath = strings.TrimPrefix(dsn, "sqlite://")
		} else if isStandalone {
			dbPath = filepath.Join(".agent-task", "swarm.db")
		} else {
            return nil, nil // No database
        }

		// Ensure directory exists
		if dir := filepath.Dir(dbPath); dir != "." {
			if err := os.MkdirAll(dir, 0755); err != nil {
				return nil, fmt.Errorf("db: create sqlite directory: %w", err)
			}
		}

		db, err := sql.Open("sqlite", dbPath)
		if err != nil {
			return nil, fmt.Errorf("db: connect to sqlite: %w", err)
		}
		if err := db.Ping(); err != nil {
			db.Close()
			return nil, fmt.Errorf("db: ping sqlite: %w", err)
		}

		slog.Info("db: connected to sqlite", "path", dbPath)
		return &Provider{Sqlite: db, Dialect: "sqlite"}, nil
	}

	// Use PostgreSQL
	pool, err := pgxpool.New(ctx, dsn)
	if err != nil {
		return nil, fmt.Errorf("db: connect to postgres: %w", err)
	}
	if err := pool.Ping(ctx); err != nil {
		pool.Close()
		return nil, fmt.Errorf("db: ping postgres: %w", err)
	}

	slog.Info("db: connected to postgres", "dsn", redactDSN(dsn))
	return &Provider{PgPool: pool, Dialect: "postgres"}, nil
}

// Close closes the active connection pool.
func (p *Provider) Close() {
	if p.PgPool != nil {
		p.PgPool.Close()
	}
	if p.Sqlite != nil {
		p.Sqlite.Close()
	}
}

func (p *Provider) RunMigrations(ctx context.Context) error {
    if p.Dialect == "sqlite" {
        return p.runSqliteMigrations(ctx)
    }
    return p.runPostgresMigrations(ctx)
}

func (p *Provider) runPostgresMigrations(ctx context.Context) error {
	// Ensure tracking table exists.
	if _, err := p.PgPool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS schema_migrations (
			filename TEXT PRIMARY KEY,
			applied_at TIMESTAMPTZ DEFAULT NOW()
		)
	`); err != nil {
		return fmt.Errorf("db: create schema_migrations: %w", err)
	}

	entries, err := migrationsFS.ReadDir("migrations")
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
		if err := p.PgPool.QueryRow(ctx, "SELECT COUNT(*) FROM schema_migrations WHERE filename = $1", f).Scan(&count); err != nil {
			return fmt.Errorf("db: check migration %s: %w", f, err)
		}
		if count > 0 {
			continue
		}

		sql, err := migrationsFS.ReadFile("migrations/"+f)
		if err != nil {
			return fmt.Errorf("db: read migration %s: %w", f, err)
		}

		tx, err := p.PgPool.Begin(ctx)
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

func (p *Provider) runSqliteMigrations(ctx context.Context) error {
	if _, err := p.Sqlite.ExecContext(ctx, `
		CREATE TABLE IF NOT EXISTS schema_migrations (
			filename TEXT PRIMARY KEY,
			applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`); err != nil {
		return fmt.Errorf("db: create schema_migrations: %w", err)
	}

	entries, err := migrationsFS.ReadDir("migrations")
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
		if err := p.Sqlite.QueryRowContext(ctx, "SELECT COUNT(*) FROM schema_migrations WHERE filename = ?", f).Scan(&count); err != nil {
			return fmt.Errorf("db: check migration %s: %w", f, err)
		}
		if count > 0 {
			continue
		}

		content, err := migrationsFS.ReadFile("migrations/"+f)
		if err != nil {
			return fmt.Errorf("db: read migration %s: %w", f, err)
		}
        sqlStr := string(content)

        // SQLite dialect adjustments
        sqlStr = strings.ReplaceAll(sqlStr, "BIGSERIAL PRIMARY KEY", "INTEGER PRIMARY KEY AUTOINCREMENT")
        sqlStr = strings.ReplaceAll(sqlStr, "TIMESTAMPTZ", "DATETIME")
        sqlStr = strings.ReplaceAll(sqlStr, "NOW()", "CURRENT_TIMESTAMP")
        sqlStr = strings.ReplaceAll(sqlStr, "JSONB", "TEXT")
        sqlStr = strings.ReplaceAll(sqlStr, "BYTEA", "BLOB")
        sqlStr = strings.ReplaceAll(sqlStr, "TEXT[]", "TEXT")
        sqlStr = strings.ReplaceAll(sqlStr, "DOUBLE PRECISION", "REAL")
        sqlStr = strings.ReplaceAll(sqlStr, "ARRAY['*']", "'[\"*\"]'")
        sqlStr = strings.ReplaceAll(sqlStr, "ARRAY['read', 'write']", "'[\"read\", \"write\"]'")
        sqlStr = strings.ReplaceAll(sqlStr, "ARRAY['read']", "'[\"read\"]'")

		tx, err := p.Sqlite.BeginTx(ctx, nil)
		if err != nil {
			return fmt.Errorf("db: begin tx for %s: %w", f, err)
		}

		if _, err := tx.ExecContext(ctx, sqlStr); err != nil {
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
