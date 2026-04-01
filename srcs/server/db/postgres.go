package db

import (
	"context"
	"embed"
	"fmt"
	"io/fs"
	"log/slog"
	"sort"
	"strings"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

var (
	//go:embed migrations/*.sql
	migrationsFS embed.FS
)

// PgPool wraps a pgxpool.Pool with Provider support.
type PgPool struct {
	pool *pgxpool.Pool
}

type pgRows struct {
	pgx.Rows
}

func (r pgRows) Close() {
	r.Rows.Close()
}

func (r pgRows) Next() bool {
	return r.Rows.Next()
}

func (r pgRows) Scan(dest ...any) error {
	return r.Rows.Scan(dest...)
}

func (r pgRows) Err() error {
	return r.Rows.Err()
}

type pgRow struct {
	pgx.Row
}

func (r pgRow) Scan(dest ...any) error {
	return r.Row.Scan(dest...)
}

type pgTx struct {
	pgx.Tx
}

func (tx pgTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	start := time.Now()
	tag, err := tx.Tx.Exec(ctx, sql, arguments...)
	recordQuery(ctx, "postgres", err, start)
	return tag.RowsAffected(), err
}

func (tx pgTx) Query(ctx context.Context, sql string, args ...any) (Rows, error) {
	start := time.Now()
	rows, err := tx.Tx.Query(ctx, sql, args...)
	recordQuery(ctx, "postgres", err, start)
	return pgRows{rows}, err
}

func (tx pgTx) QueryRow(ctx context.Context, sql string, args ...any) Row {
	start := time.Now()
	row := tx.Tx.QueryRow(ctx, sql, args...)
	recordQuery(ctx, "postgres", nil, start)
	return pgRow{row}
}

func (tx pgTx) Commit(ctx context.Context) error {
	return tx.Tx.Commit(ctx)
}

func (tx pgTx) Rollback(ctx context.Context) error {
	return tx.Tx.Rollback(ctx)
}

// NewPostgres creates a new Postgres connection pool from a DSN.
func NewPostgres(ctx context.Context, dsn string) (Provider, error) {
	pool, err := pgxpool.New(ctx, dsn)
	if err != nil {
		return nil, fmt.Errorf("db: connect to postgres: %w", err)
	}

	if err := pool.Ping(ctx); err != nil {
		pool.Close()
		return nil, fmt.Errorf("db: ping postgres: %w", err)
	}

	slog.Info("db: connected to postgres", "dsn", redactDSN(dsn))
	return &PgPool{pool: pool}, nil
}

func (p *PgPool) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	start := time.Now()
	tag, err := p.pool.Exec(ctx, sql, arguments...)
	recordQuery(ctx, "postgres", err, start)
	return tag.RowsAffected(), err
}

func (p *PgPool) Query(ctx context.Context, sql string, args ...any) (Rows, error) {
	start := time.Now()
	rows, err := p.pool.Query(ctx, sql, args...)
	recordQuery(ctx, "postgres", err, start)
	return pgRows{rows}, err
}

func (p *PgPool) QueryRow(ctx context.Context, sql string, args ...any) Row {
	start := time.Now()
	row := p.pool.QueryRow(ctx, sql, args...)
	recordQuery(ctx, "postgres", nil, start)
	return pgRow{row}
}

func (p *PgPool) Begin(ctx context.Context) (Tx, error) {
	tx, err := p.pool.Begin(ctx)
	if err != nil {
		return nil, err
	}
	return pgTx{tx}, nil
}

func (p *PgPool) Close() {
	p.pool.Close()
}

func (p *PgPool) RunMigrations(ctx context.Context) error {
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

		tx, err := p.Begin(ctx)
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
