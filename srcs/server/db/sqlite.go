package db

import (
	"context"
	"database/sql"
	"fmt"
	"io/fs"
	"log/slog"
	"sort"
	"strings"
	"time"

	_ "modernc.org/sqlite"
)

// SqlitePool wraps a database/sql.DB with Provider support.
type SqlitePool struct {
	db *sql.DB
}

type sqlRows struct {
	*sql.Rows
}

func (r sqlRows) Close() {
	r.Rows.Close()
}

func (r sqlRows) Next() bool {
	return r.Rows.Next()
}

func (r sqlRows) Scan(dest ...any) error {
	return r.Rows.Scan(dest...)
}

func (r sqlRows) Err() error {
	return r.Rows.Err()
}

type sqlRow struct {
	*sql.Row
}

func (r sqlRow) Scan(dest ...any) error {
	return r.Row.Scan(dest...)
}

type sqlTx struct {
	*sql.Tx
}

func (tx sqlTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	start := time.Now()
	res, err := tx.Tx.ExecContext(ctx, sql, arguments...)
	recordQuery(ctx, "sqlite", err, start)
	if err != nil {
		return 0, err
	}
	return res.RowsAffected()
}

func (tx sqlTx) Query(ctx context.Context, sql string, args ...any) (Rows, error) {
	start := time.Now()
	rows, err := tx.Tx.QueryContext(ctx, sql, args...)
	recordQuery(ctx, "sqlite", err, start)
	return sqlRows{rows}, err
}

func (tx sqlTx) QueryRow(ctx context.Context, sql string, args ...any) Row {
	start := time.Now()
	row := tx.Tx.QueryRowContext(ctx, sql, args...)
	recordQuery(ctx, "sqlite", nil, start)
	return sqlRow{row}
}

func (tx sqlTx) Commit(ctx context.Context) error {
	return tx.Tx.Commit()
}

func (tx sqlTx) Rollback(ctx context.Context) error {
	return tx.Tx.Rollback()
}

// NewSQLite creates a new SQLite connection pool.
func NewSQLite(ctx context.Context, dsn string) (Provider, error) {
	if strings.HasPrefix(dsn, "sqlite://") {
		dsn = strings.TrimPrefix(dsn, "sqlite://")
	}
	if dsn == "" {
		dsn = ":memory:"
	}

	db, err := sql.Open("sqlite", dsn)
	if err != nil {
		return nil, fmt.Errorf("db: connect to sqlite: %w", err)
	}

	if err := db.PingContext(ctx); err != nil {
		db.Close()
		return nil, fmt.Errorf("db: ping sqlite: %w", err)
	}

	// SQLite-specific connection settings to avoid busy locks
	db.SetMaxOpenConns(1)
	db.SetMaxIdleConns(1)

	slog.Info("db: connected to sqlite", "dsn", dsn)
	return &SqlitePool{db: db}, nil
}

func (p *SqlitePool) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	start := time.Now()
	res, err := p.db.ExecContext(ctx, sql, arguments...)
	recordQuery(ctx, "sqlite", err, start)
	if err != nil {
		return 0, err
	}
	return res.RowsAffected()
}

func (p *SqlitePool) Query(ctx context.Context, sql string, args ...any) (Rows, error) {
	start := time.Now()
	rows, err := p.db.QueryContext(ctx, sql, args...)
	recordQuery(ctx, "sqlite", err, start)
	return sqlRows{rows}, err
}

func (p *SqlitePool) QueryRow(ctx context.Context, sql string, args ...any) Row {
	start := time.Now()
	row := p.db.QueryRowContext(ctx, sql, args...)
	recordQuery(ctx, "sqlite", nil, start)
	return sqlRow{row}
}

func (p *SqlitePool) Begin(ctx context.Context) (Tx, error) {
	tx, err := p.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	return sqlTx{tx}, nil
}

func (p *SqlitePool) Close() {
	p.db.Close()
}

// applySQLiteDialect modifies postgres syntax to sqlite syntax
func applySQLiteDialect(sql string) string {
	sql = strings.ReplaceAll(sql, "TIMESTAMPTZ", "DATETIME")
	sql = strings.ReplaceAll(sql, "NOW()", "CURRENT_TIMESTAMP")
	sql = strings.ReplaceAll(sql, "JSONB", "JSON")
	sql = strings.ReplaceAll(sql, "DOUBLE PRECISION", "REAL")
	sql = strings.ReplaceAll(sql, "BIGSERIAL", "INTEGER")

	// Array handling fallback (JSON) for simple TEXT[] setup
	sql = strings.ReplaceAll(sql, "TEXT[] NOT NULL DEFAULT '{}'", "TEXT NOT NULL DEFAULT '[]'")
	sql = strings.ReplaceAll(sql, "BYTEA", "BLOB")
	return sql
}

func (p *SqlitePool) RunMigrations(ctx context.Context) error {
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
		var count int
		err := p.QueryRow(ctx, "SELECT COUNT(*) FROM schema_migrations WHERE filename = $1", f).Scan(&count)
		if err != nil {
			return fmt.Errorf("db: check migration %s: %w", f, err)
		}
		if count > 0 {
			continue
		}

		sqlBytes, err := fs.ReadFile(migrationsFS, "migrations/"+f)
		if err != nil {
			return fmt.Errorf("db: read migration %s: %w", f, err)
		}

		sqlStmt := applySQLiteDialect(string(sqlBytes))

		tx, err := p.Begin(ctx)
		if err != nil {
			return fmt.Errorf("db: begin tx for %s: %w", f, err)
		}

		// Since SQLite exec doesn't run multiple statements correctly from one string always in database/sql,
		// we should split by ; and execute them one by one.
		stmts := strings.Split(sqlStmt, ";")
		for _, stmt := range stmts {
			stmt = strings.TrimSpace(stmt)
			if stmt == "" {
				continue
			}
			if _, err := tx.Exec(ctx, stmt); err != nil {
				_ = tx.Rollback(ctx)
				return fmt.Errorf("db: exec migration %s statement %s: %w", f, stmt, err)
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
