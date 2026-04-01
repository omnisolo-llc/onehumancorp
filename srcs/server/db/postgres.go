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
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgconn"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
)

var (
	//go:embed migrations/*.sql
	migrationsFS embed.FS
)

// Pool wraps a pgxpool.Pool with migration support.
type Pool struct {
	*pgxpool.Pool
}

// New creates a new Postgres connection pool from DATABASE_URL.
// Returns nil if DATABASE_URL is not set (enabling zero-dep local mode).
func New(ctx context.Context) (*Pool, error) {
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
	return &Pool{Pool: pool}, nil
}

// RunMigrations executes all embedded SQL migrations, sorted
// lexicographically.  Each migration is run inside a transaction.
// A simple `schema_migrations` table tracks which files have already been
// applied.
func (p *Pool) RunMigrations(ctx context.Context) error {
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

func (p *Pool) Exec(ctx context.Context, sql string, arguments ...any) (pgconn.CommandTag, error) {
	start := time.Now()
	ctx, span := otel.Tracer("db").Start(ctx, "Exec")
	span.SetAttributes(attribute.String("db.system", "postgresql"), attribute.String("db.statement", sql))
	defer span.End()

	res, err := p.Pool.Exec(ctx, sql, arguments...)
	telemetry.RecordDBQuery(ctx, "Exec", "postgres", time.Since(start).Seconds())
	if err != nil {
		span.RecordError(err)
	}
	return res, err
}

func (p *Pool) Query(ctx context.Context, sql string, optionsAndArgs ...any) (pgx.Rows, error) {
	start := time.Now()
	ctx, span := otel.Tracer("db").Start(ctx, "Query")
	span.SetAttributes(attribute.String("db.system", "postgresql"), attribute.String("db.statement", sql))
	defer span.End()

	res, err := p.Pool.Query(ctx, sql, optionsAndArgs...)
	telemetry.RecordDBQuery(ctx, "Query", "postgres", time.Since(start).Seconds())
	if err != nil {
		span.RecordError(err)
	}
	return res, err
}

func (p *Pool) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) pgx.Row {
	start := time.Now()
	ctx, span := otel.Tracer("db").Start(ctx, "QueryRow")
	span.SetAttributes(attribute.String("db.system", "postgresql"), attribute.String("db.statement", sql))
	defer span.End()

	res := p.Pool.QueryRow(ctx, sql, optionsAndArgs...)
	telemetry.RecordDBQuery(ctx, "QueryRow", "postgres", time.Since(start).Seconds())
	return res
}
