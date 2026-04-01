package db

import (
	"context"
	"database/sql"
	"fmt"
	"io/fs"
	"log/slog"
	"regexp"
	"sort"
	"strings"

	_ "modernc.org/sqlite"
)

var paramRegex = regexp.MustCompile(`\$\d+`)

// SQLiteProvider implements DatabaseProvider for SQLite.
type SQLiteProvider struct {
	db *sql.DB
}

// NewSQLite creates a new SQLite connection from a given path.
func NewSQLite(dbPath string) (*SQLiteProvider, error) {
	dsn := dbPath
	if dbPath != ":memory:" && !strings.Contains(dbPath, "mode=memory") {
		if !strings.Contains(dsn, "?") {
			dsn += "?"
		} else {
			dsn += "&"
		}
		dsn += "_pragma=journal_mode(WAL)&_pragma=busy_timeout(15000)&_txlock=immediate"
	}

	db, err := sql.Open("sqlite", dsn)
	if err != nil {
		return nil, fmt.Errorf("db: connect to sqlite: %w", err)
	}

	db.SetMaxOpenConns(1)

	if err := db.Ping(); err != nil {
		db.Close()
		return nil, fmt.Errorf("db: ping sqlite: %w", err)
	}

	slog.Info("db: connected to sqlite", "dsn", dsn)
	return &SQLiteProvider{db: db}, nil
}

// Exec executes a query without returning any rows.
func (p *SQLiteProvider) Exec(ctx context.Context, query string, args ...any) (int64, error) {
	// Simple rewrite of postgres placeholders to sqlite placeholders (assuming simple query structures)
	query = convertPlaceholdersToSQLite(query)
	res, err := p.db.ExecContext(ctx, query, args...)
	if err != nil {
		return 0, err
	}
	return res.RowsAffected()
}

// Query executes a query that returns rows.
func (p *SQLiteProvider) Query(ctx context.Context, query string, args ...any) (Rows, error) {
	query = convertPlaceholdersToSQLite(query)
	rows, err := p.db.QueryContext(ctx, query, args...)
	if err != nil {
		return nil, err
	}
	return &sqlRows{rows: rows}, nil
}

// QueryRow executes a query that is expected to return at most one row.
func (p *SQLiteProvider) QueryRow(ctx context.Context, query string, args ...any) Row {
	query = convertPlaceholdersToSQLite(query)
	return p.db.QueryRowContext(ctx, query, args...)
}

// Begin starts a transaction.
func (p *SQLiteProvider) Begin(ctx context.Context) (Tx, error) {
	tx, err := p.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	return &sqlTx{tx: tx}, nil
}

// Close closes the connection pool.
func (p *SQLiteProvider) Close() {
	if p.db != nil {
		p.db.Close()
	}
}

// IsSQLite returns true for SQLiteProvider.
func (p *SQLiteProvider) IsSQLite() bool {
	return true
}

// sqlRows wraps sql.Rows to satisfy db.Rows interface.
type sqlRows struct {
	rows *sql.Rows
}

func (r *sqlRows) Next() bool {
	return r.rows.Next()
}

func (r *sqlRows) Scan(dest ...any) error {
	return r.rows.Scan(dest...)
}

func (r *sqlRows) Close() {
	r.rows.Close()
}

func (r *sqlRows) Err() error {
	return r.rows.Err()
}

// sqlTx wraps sql.Tx to satisfy db.Tx interface.
type sqlTx struct {
	tx *sql.Tx
}

func (t *sqlTx) Exec(ctx context.Context, query string, args ...any) (int64, error) {
	query = convertPlaceholdersToSQLite(query)
	res, err := t.tx.ExecContext(ctx, query, args...)
	if err != nil {
		return 0, err
	}
	return res.RowsAffected()
}

func (t *sqlTx) Query(ctx context.Context, query string, args ...any) (Rows, error) {
	query = convertPlaceholdersToSQLite(query)
	rows, err := t.tx.QueryContext(ctx, query, args...)
	if err != nil {
		return nil, err
	}
	return &sqlRows{rows: rows}, nil
}

func (t *sqlTx) QueryRow(ctx context.Context, query string, args ...any) Row {
	query = convertPlaceholdersToSQLite(query)
	return t.tx.QueryRowContext(ctx, query, args...)
}

func (t *sqlTx) Commit(ctx context.Context) error {
	return t.tx.Commit()
}

func (t *sqlTx) Rollback(ctx context.Context) error {
	return t.tx.Rollback()
}

// RunMigrations executes all embedded SQL migrations for SQLite.
// We apply SQLite specific schema changes or reuse common SQL.
func (p *SQLiteProvider) RunMigrations(ctx context.Context) error {
	// Ensure tracking table exists.
	if _, err := p.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS schema_migrations (
			filename TEXT PRIMARY KEY,
			applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
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

		sqlBytes, err := fs.ReadFile(migrationsFS, "migrations/"+f)
		if err != nil {
			return fmt.Errorf("db: read migration %s: %w", f, err)
		}
		sqlContent := string(sqlBytes)

		// SQLite specific adaptations for migrations:
		sqlContent = convertDDLToSQLite(sqlContent)

		tx, err := p.Begin(ctx)
		if err != nil {
			return fmt.Errorf("db: begin tx for %s: %w", f, err)
		}

		// Split by statement since SQLite driver might not support executing multiple statements at once perfectly.
		statements := splitSQLStatements(sqlContent)

		for _, stmt := range statements {
			if strings.TrimSpace(stmt) == "" {
				continue
			}
			if _, err := tx.Exec(ctx, stmt); err != nil {
				_ = tx.Rollback(ctx)
				return fmt.Errorf("db: exec migration %s: %w", f, err)
			}
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

// convertPlaceholdersToSQLite converts PostgreSQL positional parameters ($1, $2, etc.) to generic SQLite '?' placeholders.
func convertPlaceholdersToSQLite(query string) string {
	return paramRegex.ReplaceAllString(query, "?")
}

// convertDDLToSQLite translates some basic PostgreSQL syntax to SQLite compatible syntax.
func convertDDLToSQLite(sqlContent string) string {
	// Timestamps
	sqlContent = strings.ReplaceAll(sqlContent, "TIMESTAMPTZ", "DATETIME")
	sqlContent = strings.ReplaceAll(sqlContent, "NOW()", "CURRENT_TIMESTAMP")

	// Data types
	sqlContent = strings.ReplaceAll(sqlContent, "BIGSERIAL", "INTEGER")
	sqlContent = strings.ReplaceAll(sqlContent, "DOUBLE PRECISION", "REAL")
	sqlContent = strings.ReplaceAll(sqlContent, "JSONB", "TEXT")
	sqlContent = strings.ReplaceAll(sqlContent, "BYTEA", "BLOB")
	sqlContent = strings.ReplaceAll(sqlContent, "TEXT[]", "TEXT") // JSON arrays in sqlite usually

	return sqlContent
}

func splitSQLStatements(sql string) []string {
	var statements []string
	var currentStmt strings.Builder
	inString := false
	inComment := false

	lines := strings.Split(sql, "\n")
	for _, line := range lines {
		trimLine := strings.TrimSpace(line)
		if strings.HasPrefix(trimLine, "--") {
			continue // skip full line comments
		}

		for i := 0; i < len(line); i++ {
			c := line[i]
			if c == '\'' {
				inString = !inString
			}
			if !inString && i < len(line)-1 && line[i:i+2] == "--" {
				inComment = true
			}
			if inComment {
				break
			}
			currentStmt.WriteByte(c)
			if c == ';' && !inString {
				statements = append(statements, currentStmt.String())
				currentStmt.Reset()
			}
		}
		if inComment {
			inComment = false
		} else {
			currentStmt.WriteString("\n")
		}
	}

	if strings.TrimSpace(currentStmt.String()) != "" {
		statements = append(statements, currentStmt.String())
	}
	return statements
}
