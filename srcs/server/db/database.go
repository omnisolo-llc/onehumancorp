package db

import (
	"context"
	"database/sql"
	"embed"
	"fmt"
	"io/fs"
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

// DB wraps a Provider with migration support.
type DB struct {
	Provider
}

// New creates a new Provider from DATABASE_URL.
// If DATABASE_URL is empty and OHC_STANDALONE=true, it defaults to a local SQLite database at .agent-task/swarm.db
// Otherwise, if DATABASE_URL is empty, it defaults to ~/.openclaw/ohc_state.db
// If DATABASE_URL starts with sqlite:// it uses SQLite.
// Otherwise it assumes PostgreSQL.
func New(ctx context.Context) (*DB, error) {
	dsn := os.Getenv("DATABASE_URL")
	if dsn == "" || strings.HasPrefix(dsn, "sqlite://") {
		var dbPath string
		if dsn == "" {
			if os.Getenv("OHC_STANDALONE") == "true" {
				agentTaskDir := ".agent-task"
				if err := os.MkdirAll(agentTaskDir, 0755); err != nil {
					return nil, fmt.Errorf("db: create .agent-task dir: %w", err)
				}
				dbPath = filepath.Join(agentTaskDir, "swarm.db")
			} else {
				homeDir, err := os.UserHomeDir()
				if err != nil {
					return nil, fmt.Errorf("db: find home dir: %w", err)
				}
				openclawDir := filepath.Join(homeDir, ".openclaw")
				if err := os.MkdirAll(openclawDir, 0755); err != nil {
					return nil, fmt.Errorf("db: create .openclaw dir: %w", err)
				}
				dbPath = filepath.Join(openclawDir, "ohc_state.db")
			}
		} else {
			dbPath = strings.TrimPrefix(dsn, "sqlite://")
		}

		sqliteDSN := dbPath
		if dbPath != ":memory:" && !strings.Contains(dbPath, "mode=memory") {
			if !strings.Contains(sqliteDSN, "?") {
				sqliteDSN += "?"
			} else {
				sqliteDSN += "&"
			}
			sqliteDSN += "_pragma=journal_mode(WAL)&_pragma=busy_timeout(15000)&_pragma=foreign_keys(1)"
		}

		db, err := sql.Open("sqlite", sqliteDSN)
		if err != nil {
			return nil, fmt.Errorf("db: connect to sqlite: %w", err)
		}
		db.SetMaxOpenConns(1)

		if err := db.PingContext(ctx); err != nil {
			db.Close()
			return nil, fmt.Errorf("db: ping sqlite: %w", err)
		}

		slog.Info("db: connected to sqlite", "path", dbPath)
		return &DB{Provider: NewSqliteProvider(db)}, nil
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
	return &DB{Provider: NewPgProvider(pool)}, nil
}

// RunMigrations executes all embedded SQL migrations, sorted
// lexicographically.  Each migration is run inside a transaction.
// A simple `schema_migrations` table tracks which files have already been
// applied.
func (p *DB) RunMigrations(ctx context.Context) error {
	// Ensure tracking table exists.
	// We use standard timestamp to be compatible with both
	if _, err := p.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS schema_migrations (
			filename TEXT PRIMARY KEY,
			applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
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
		// Some drivers need ? others $1. Let's try replacing $1 with ?. Actually pgx supports $1, database/sql with sqlite supports ? and sometimes $1.
		// Since we control both, sqlite supports $1 if named properly, but modernc/sqlite supports $1 natively in queries!
		// But just to be safe let's query with parameter properly.
		row := p.QueryRow(ctx, "SELECT COUNT(*) FROM schema_migrations WHERE filename = $1", f)
		err := row.Scan(&count)
		if err != nil {
			// modernc.sqlite might not like $1 by default without specific pragma, let's fallback to ? if error
			if strings.Contains(err.Error(), "syntax error") || strings.Contains(err.Error(), "parameter") {
				row = p.QueryRow(ctx, "SELECT COUNT(*) FROM schema_migrations WHERE filename = ?", f)
				err = row.Scan(&count)
			}
			if err != nil {
				return fmt.Errorf("db: check migration %s: %w", f, err)
			}
		}
		if count > 0 {
			continue
		}

		sqlBytes, err := fs.ReadFile(migrationsFS, "migrations/"+f)
		if err != nil {
			return fmt.Errorf("db: read migration %s: %w", f, err)
		}

		sqlStr := string(sqlBytes)

		// If using sqlite, we might need to replace pg-specific types or handle syntax
		_, isSqlite := p.Provider.(*SqliteProvider)
		if isSqlite {
			// Simple replacements for basic SQLite compatibility if needed, though most standard SQL works.
			// Bigserial -> INTEGER PRIMARY KEY AUTOINCREMENT
			// TIMESTAMPTZ -> DATETIME
			// JSONB -> TEXT
			// BYTEA -> BLOB
			sqlStr = strings.ReplaceAll(sqlStr, "BIGSERIAL PRIMARY KEY", "INTEGER PRIMARY KEY AUTOINCREMENT")
			sqlStr = strings.ReplaceAll(sqlStr, "TIMESTAMPTZ", "DATETIME")
			sqlStr = strings.ReplaceAll(sqlStr, "JSONB", "TEXT")
			sqlStr = strings.ReplaceAll(sqlStr, "BYTEA", "BLOB")
			// We need to remove the array syntax `TEXT[] NOT NULL DEFAULT '{}'`
			// Because SQLite does not support arrays.
			// Replaced with TEXT DEFAULT '[]' for JSON array storage
			sqlStr = strings.ReplaceAll(sqlStr, "TEXT[] NOT NULL DEFAULT '{}'", "TEXT NOT NULL DEFAULT '[]'")
			sqlStr = strings.ReplaceAll(sqlStr, "NOW()", "CURRENT_TIMESTAMP")

			// Replace specific Postgres Array insert syntax used in migrations
			sqlStr = strings.ReplaceAll(sqlStr, "ARRAY['*']", "'[\"*\"]'")
			sqlStr = strings.ReplaceAll(sqlStr, "ARRAY['read', 'write']", "'[\"read\", \"write\"]'")
			sqlStr = strings.ReplaceAll(sqlStr, "ARRAY['read']", "'[\"read\"]'")
		}

		tx, err := p.Begin(ctx)
		if err != nil {
			return fmt.Errorf("db: begin tx for %s: %w", f, err)
		}

		if _, err := tx.Exec(ctx, sqlStr); err != nil {
			_ = tx.Rollback(ctx)
			return fmt.Errorf("db: exec migration %s: %w", f, err)
		}

		_, err = tx.Exec(ctx, "INSERT INTO schema_migrations (filename) VALUES ($1)", f)
		if err != nil {
			_, err2 := tx.Exec(ctx, "INSERT INTO schema_migrations (filename) VALUES (?)", f)
			if err2 != nil {
				_ = tx.Rollback(ctx)
				return fmt.Errorf("db: record migration %s: %w", f, err)
			}
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
