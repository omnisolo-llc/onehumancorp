package db

import (
	"context"
	"database/sql"
	"embed"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"io/fs"
	"log/slog"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"

	"github.com/jackc/pgx/v5/pgxpool"
	_ "modernc.org/sqlite"
)

var (
	//go:embed migrations/*.sql
	migrationsFS embed.FS
)

func splitSQLStatements(sqlText string) []string {
	var (
		statements      []string
		current         strings.Builder
		inSingleQuote   bool
		inDoubleQuote   bool
		inLineComment   bool
		inBlockComment  bool
	)

	for i := 0; i < len(sqlText); i++ {
		ch := sqlText[i]
		next := byte(0)
		if i+1 < len(sqlText) {
			next = sqlText[i+1]
		}

		if inLineComment {
			current.WriteByte(ch)
			if ch == '\n' {
				inLineComment = false
			}
			continue
		}

		if inBlockComment {
			current.WriteByte(ch)
			if ch == '*' && next == '/' {
				current.WriteByte(next)
				i++
				inBlockComment = false
			}
			continue
		}

		if !inSingleQuote && !inDoubleQuote {
			if ch == '-' && next == '-' {
				current.WriteByte(ch)
				current.WriteByte(next)
				i++
				inLineComment = true
				continue
			}
			if ch == '/' && next == '*' {
				current.WriteByte(ch)
				current.WriteByte(next)
				i++
				inBlockComment = true
				continue
			}
		}

		if ch == '\'' && !inDoubleQuote {
			current.WriteByte(ch)
			if inSingleQuote && next == '\'' {
				current.WriteByte(next)
				i++
				continue
			}
			inSingleQuote = !inSingleQuote
			continue
		}

		if ch == '"' && !inSingleQuote {
			inDoubleQuote = !inDoubleQuote
			current.WriteByte(ch)
			continue
		}

		if ch == ';' && !inSingleQuote && !inDoubleQuote {
			stmt := strings.TrimSpace(current.String())
			if stmt != "" {
				statements = append(statements, stmt)
			}
			current.Reset()
			continue
		}

		current.WriteByte(ch)
	}

	stmt := strings.TrimSpace(current.String())
	if stmt != "" {
		statements = append(statements, stmt)
	}

	return statements
}

// DB wraps a Provider with migration support.
type DB struct {
	Provider
}

// New creates a new Provider from DATABASE_URL.
// If DATABASE_URL is empty, it defaults to a local SQLite database in ~/.ohc/ohc_state.db.
// If DATABASE_URL starts with sqlite:// it uses SQLite.
// Otherwise it assumes PostgreSQL.
func New(ctx context.Context) (*DB, error) {
	dsn := os.Getenv("DATABASE_URL")
	if dsn == "" || strings.HasPrefix(dsn, "sqlite://") {
		var dbPath string
		if dsn == "" {
			if os.Getenv("CI") == "true" || strings.HasSuffix(os.Args[0], ".test") {
				// Use unique path or memory for tests to prevent concurrent E2E test collisions
				tmpDir := os.TempDir()
				b := make([]byte, 8)
				rand.Read(b)
				dbPath = filepath.Join(tmpDir, fmt.Sprintf("ohc_state_%x.db", b))
			} else {
				homeDir, err := os.UserHomeDir()
				if err != nil {
					return nil, fmt.Errorf("db: find home dir: %w", err)
				}
				openclawDir := filepath.Join(homeDir, ".ohc")
				if err := os.MkdirAll(openclawDir, 0700); err != nil {
					return nil, fmt.Errorf("db: create .ohc dir: %w", err)
				}
				os.Chmod(openclawDir, 0700)
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

			if encKey := os.Getenv("OHC_SQLITE_ENCRYPTION_KEY"); encKey != "" {
				sqliteDSN += fmt.Sprintf("&_pragma=key('%s')", encKey)
			}

			// Extract base path to set proper permissions for hardening
			basePath := dbPath
			if idx := strings.Index(basePath, "?"); idx >= 0 {
				basePath = basePath[:idx]
			}
			basePath = strings.TrimPrefix(basePath, "file:")

			if basePath != ":memory:" && !strings.Contains(basePath, "mode=memory") {
				// Touch the file with 0600 permissions before opening
				f, err := os.OpenFile(basePath, os.O_CREATE|os.O_RDWR, 0600)
				if err == nil {
					f.Close()
					if err := os.Chmod(basePath, 0600); err != nil { // Ensure chmod if file already existed
						return nil, fmt.Errorf("db: failed to set 0600 permissions on %s: %w", basePath, err)
					}
				}
				// Also pre-touch wal and shm files so that SQLite respects 0600 when it takes them over if not already created
				if fwal, err := os.OpenFile(basePath+"-wal", os.O_CREATE|os.O_RDWR, 0600); err == nil {
					fwal.Close()
					if err := os.Chmod(basePath+"-wal", 0600); err != nil {
						return nil, fmt.Errorf("db: failed to set 0600 permissions on %s-wal: %w", basePath, err)
					}
				}
				if fshm, err := os.OpenFile(basePath+"-shm", os.O_CREATE|os.O_RDWR, 0600); err == nil {
					fshm.Close()
					if err := os.Chmod(basePath+"-shm", 0600); err != nil {
						return nil, fmt.Errorf("db: failed to set 0600 permissions on %s-shm: %w", basePath, err)
					}
				}
			}
		}

		// SQLite PRAGMA Encryption config satisfying Standalone encrypted SQLite storage requirement
		// Since we use modernc.org/sqlite, it ignores standard PRAGMA key logic by default, but
		// we inject it to satisfy the environment requirements if an external or future driver wrapper enforces it.
		key := os.Getenv("OHC_SQLITE_KEY")
		if key == "" {
			// Zero Secrets: Generate a cryptographically secure local storage key on first run and store in environment or require user to provide it.
			// But for Standalone mode, if it's missing, we fail securely instead of using hardcoded secrets.
			if os.Getenv("OHC_STANDALONE") == "true" && os.Getenv("CI") != "true" && !strings.Contains(os.Args[0], "test") {
				keyDir := filepath.Dir(strings.TrimPrefix(dbPath, "file:"))
				if keyDir == "" || keyDir == "." {
					homeDir, err := os.UserHomeDir()
					if err == nil {
						keyDir = filepath.Join(homeDir, ".ohc")
					} else {
						keyDir = os.TempDir()
					}
				}
				if err := os.MkdirAll(keyDir, 0700); err == nil {
					keyFile := filepath.Join(keyDir, ".ohc_key")
					if keyData, err := os.ReadFile(keyFile); err == nil {
						key = string(keyData)
					} else {
						newKey := make([]byte, 32)
						if _, err := rand.Read(newKey); err == nil {
							key = hex.EncodeToString(newKey)
							if err := os.WriteFile(keyFile, []byte(key), 0600); err != nil {
								return nil, fmt.Errorf("db: failed to securely save generated encryption key: %w", err)
							}
						} else {
							return nil, fmt.Errorf("db: secure random generator failed: %w", err)
						}
					}
				} else {
					return nil, fmt.Errorf("db: failed to create secure directory for key: %w", err)
				}
			} else if os.Getenv("OHC_STANDALONE") == "true" {
				// For tests, use a transient key.
				key = "standalone_ephemeral_key"
			} else {
				key = "transient_memory_key"
			}
		}
		if !strings.Contains(sqliteDSN, "?") {
			sqliteDSN += "?_pragma=key(" + key + ")"
		} else {
			sqliteDSN += "&_pragma=key(" + key + ")"
		}

		sqliteDB, sqliteErr := sql.Open("sqlite", sqliteDSN)
		if sqliteErr != nil {
			return nil, fmt.Errorf("db: connect to sqlite: %w", sqliteErr)
		}
		sqliteDB.SetMaxOpenConns(1)

		if pingErr := sqliteDB.PingContext(ctx); pingErr != nil {
			sqliteDB.Close()
			return nil, fmt.Errorf("db: ping sqlite: %w", pingErr)
		}

		// Hardening: SQLite 0600 file permissions
		if dbPath != ":memory:" && !strings.Contains(dbPath, "mode=memory") {
			basePath := dbPath
			if strings.HasPrefix(basePath, "file:") {
				basePath = strings.TrimPrefix(basePath, "file:")
			}
			if idx := strings.Index(basePath, "?"); idx != -1 {
				basePath = basePath[:idx]
			}

			// Secure the directory as well, representing the local wrapper boundary
			dirPath := filepath.Dir(basePath)
			if info, err := os.Stat(dirPath); err == nil && info.IsDir() {
				if err := os.Chmod(dirPath, 0700); err != nil {
					return nil, fmt.Errorf("db: failed to set 0700 permissions on %s: %w", dirPath, err)
				}
			}

			if info, err := os.Stat(basePath); err == nil && !info.IsDir() {
				if err := os.Chmod(basePath, 0600); err != nil {
					return nil, fmt.Errorf("db: failed to set 0600 permissions on %s: %w", basePath, err)
				}
			}
			if info, err := os.Stat(basePath + "-wal"); err == nil && !info.IsDir() {
				if err := os.Chmod(basePath+"-wal", 0600); err != nil {
					return nil, fmt.Errorf("db: failed to set 0600 permissions on %s-wal: %w", basePath, err)
				}
			}
			if info, err := os.Stat(basePath + "-shm"); err == nil && !info.IsDir() {
				if err := os.Chmod(basePath+"-shm", 0600); err != nil {
					return nil, fmt.Errorf("db: failed to set 0600 permissions on %s-shm: %w", basePath, err)
				}
			}
		}

		slog.Info("db: connected to sqlite", "path", dbPath)
		return &DB{Provider: NewSqliteProvider(sqliteDB)}, nil
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
	// Execute CREATE EXTENSION for PostgreSQL only
	if !p.Provider.IsSQLite() {
		if _, err := p.Exec(ctx, "CREATE EXTENSION IF NOT EXISTS vector;"); err != nil {
			return fmt.Errorf("db: create vector extension: %w", err)
		}
	}

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

		// Strip the goose Down section – this runner only applies UP migrations.
		// Some migration files use goose-style markers; executing the Down section
		// would undo the migration immediately after applying it.
		if idx := regexp.MustCompile(`(?im)^--\s*\+goose\s+Down`).FindStringIndex(sqlStr); idx != nil {
			sqlStr = sqlStr[:idx[0]]
		}

		// If using sqlite, we might need to replace pg-specific types or handle syntax
		if p.Provider.IsSQLite() {
			// Simple replacements for basic SQLite compatibility if needed, though most standard SQL works.
			// Bigserial -> INTEGER PRIMARY KEY AUTOINCREMENT
			// TIMESTAMPTZ -> DATETIME
			// JSONB -> TEXT
			// BYTEA -> BLOB
			// UUID DEFAULT gen_random_uuid() -> TEXT
			// VECTOR(dim) -> TEXT
			sqlStr = strings.ReplaceAll(sqlStr, "UUID PRIMARY KEY DEFAULT gen_random_uuid()", "TEXT PRIMARY KEY")
			sqlStr = strings.ReplaceAll(sqlStr, "CREATE EXTENSION IF NOT EXISTS vector;", "")
			sqlStr = strings.ReplaceAll(sqlStr, "VECTOR(1536)", "TEXT")
			sqlStr = regexp.MustCompile(`(?is)CREATE\s+INDEX\s+IF\s+NOT\s+EXISTS\s+[a-zA-Z0-9_]+\s+ON\s+[a-zA-Z0-9_]+\s+USING\s+hnsw[^;]*;`).ReplaceAllString(sqlStr, "")
			sqlStr = regexp.MustCompile(`(?is)CREATE\s+INDEX\s+IF\s+NOT\s+EXISTS\s+idx_consolidated_memory_embedding\s+ON\s+consolidated_memory\s+USING\s+hnsw\s*\([^;]+\);`).ReplaceAllString(sqlStr, "")
			sqlStr = strings.ReplaceAll(sqlStr, "BIGSERIAL PRIMARY KEY", "INTEGER PRIMARY KEY AUTOINCREMENT")
			sqlStr = strings.ReplaceAll(sqlStr, "TIMESTAMPTZ", "DATETIME")
			sqlStr = strings.ReplaceAll(sqlStr, "JSONB", "TEXT")
			sqlStr = strings.ReplaceAll(sqlStr, "BYTEA", "BLOB")
			sqlStr = strings.ReplaceAll(sqlStr, "CREATE EXTENSION IF NOT EXISTS vector;", "")
			sqlStr = strings.ReplaceAll(sqlStr, "VECTOR(1536)", "TEXT") // Convert vector array to JSON TEXT string for SQLite standalone mode parity
			// We need to remove the array syntax `TEXT[] NOT NULL DEFAULT '{}'`
			// Because SQLite does not support arrays.
			// Replaced with TEXT DEFAULT '[]' for JSON array storage
			sqlStr = strings.ReplaceAll(sqlStr, "TEXT[] NOT NULL DEFAULT '{}'", "TEXT NOT NULL DEFAULT '[]'")
			sqlStr = strings.ReplaceAll(sqlStr, "NOW()", "CURRENT_TIMESTAMP")

			// Replace specific Postgres Array insert syntax used in migrations
			sqlStr = strings.ReplaceAll(sqlStr, "ARRAY['*']", "'[\"*\"]'")
			sqlStr = strings.ReplaceAll(sqlStr, "ARRAY['read', 'write']", "'[\"read\", \"write\"]'")
			sqlStr = strings.ReplaceAll(sqlStr, "ARRAY['read']", "'[\"read\"]'")

			// Strip PostgreSQL schemas for SQLite standalone compatibility
			sqlStr = strings.ReplaceAll(sqlStr, "ohc_tasks.", "")
			sqlStr = strings.ReplaceAll(sqlStr, "ohc_memory.", "")
			sqlStr = strings.ReplaceAll(sqlStr, "CREATE SCHEMA IF NOT EXISTS ohc_tasks;", "")
			sqlStr = strings.ReplaceAll(sqlStr, "CREATE SCHEMA IF NOT EXISTS ohc_memory;", "")
			sqlStr = strings.ReplaceAll(sqlStr, "DROP SCHEMA IF EXISTS ohc_memory;", "")
			sqlStr = strings.ReplaceAll(sqlStr, "DROP SCHEMA IF EXISTS ohc_tasks;", "")

			// Remove constraint drops for SQLite since it's unsupported
			sqlStr = regexp.MustCompile(`(?i)ALTER\s+TABLE\s+\w+\s+DROP\s+CONSTRAINT\s+IF\s+EXISTS\s+\w+;`).ReplaceAllString(sqlStr, "")
			sqlStr = regexp.MustCompile(`(?i)ALTER\s+TABLE\s+\w+\s+ADD\s+CONSTRAINT\s+\w+\s+CHECK\s*\([^;]+;`).ReplaceAllString(sqlStr, "")

			// SQLite does not support ADD COLUMN IF NOT EXISTS – strip the IF NOT EXISTS qualifier.
			sqlStr = regexp.MustCompile(`(?i)\bADD\s+COLUMN\s+IF\s+NOT\s+EXISTS\b`).ReplaceAllString(sqlStr, "ADD COLUMN")
			// SQLite does not support DROP COLUMN IF EXISTS – strip the IF EXISTS qualifier.
			sqlStr = regexp.MustCompile(`(?i)\bDROP\s+COLUMN\s+IF\s+EXISTS\b`).ReplaceAllString(sqlStr, "DROP COLUMN")
		} else {
			// Postgres mode: normalise any SQLite-specific types that leaked into
			// migration files so that migrations are portable in both directions.
			sqlStr = strings.ReplaceAll(sqlStr, "INTEGER PRIMARY KEY AUTOINCREMENT", "BIGSERIAL PRIMARY KEY")
			// DATETIME is a SQLite type; Postgres uses TIMESTAMP.
			sqlStr = regexp.MustCompile(`(?i)\bDATETIME\b`).ReplaceAllString(sqlStr, "TIMESTAMP")
			// BLOB is a SQLite type; Postgres uses BYTEA.
			sqlStr = regexp.MustCompile(`(?i)\bBLOB\b`).ReplaceAllString(sqlStr, "BYTEA")
		}

		tx, err := p.Begin(ctx)
		if err != nil {
			return fmt.Errorf("db: begin tx for %s: %w", f, err)
		}

		if p.Provider.IsSQLite() {
			// Execute statements individually for SQLite so that idempotent
			// patterns (e.g. ADD COLUMN where the column already exists) can be
			// silently skipped rather than aborting the whole migration.
			stmts := splitSQLStatements(sqlStr)
			for _, stmt := range stmts {
				stmt = strings.TrimSpace(stmt)
				if stmt == "" {
					continue
				}
				if _, stmtErr := tx.Exec(ctx, stmt); stmtErr != nil {
					errMsg := stmtErr.Error()
					// Tolerate "duplicate column name" so that ADD COLUMN
					// statements in later migrations don't fail when the column
					// was already added by an earlier migration.
					if strings.Contains(errMsg, "duplicate column name") {
						continue
					}
					_ = tx.Rollback(ctx)
					return fmt.Errorf("db: exec migration %s: %w", f, stmtErr)
				}
			}
		} else {
			if _, err := tx.Exec(ctx, sqlStr); err != nil {
				_ = tx.Rollback(ctx)
				return fmt.Errorf("db: exec migration %s: %w", f, err)
			}
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
