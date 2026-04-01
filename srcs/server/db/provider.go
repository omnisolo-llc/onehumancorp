package db

import (
	"context"
	"os"
	"strings"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter          = otel.Meter("github.com/onehumancorp/mono/srcs/server/db")
	queryDuration  metric.Float64Histogram
	queryErrorCount metric.Int64Counter
)

func init() {
	var err error
	queryDuration, err = meter.Float64Histogram(
		"db.query.duration",
		metric.WithDescription("Duration of database queries"),
		metric.WithUnit("s"),
	)
	if err != nil {
		panic(err)
	}

	queryErrorCount, err = meter.Int64Counter(
		"db.query.errors",
		metric.WithDescription("Number of database query errors"),
	)
	if err != nil {
		panic(err)
	}
}

// Rows is the interface for iterating over rows.
type Rows interface {
	Close()
	Next() bool
	Scan(dest ...any) error
	Err() error
}

// Row is the interface for scanning a single row.
type Row interface {
	Scan(dest ...any) error
}

// Tx represents a database transaction.
type Tx interface {
	Exec(ctx context.Context, sql string, arguments ...any) (int64, error)
	Query(ctx context.Context, sql string, args ...any) (Rows, error)
	QueryRow(ctx context.Context, sql string, args ...any) Row
	Commit(ctx context.Context) error
	Rollback(ctx context.Context) error
}

// Provider represents an abstract database connection pool.
type Provider interface {
	Exec(ctx context.Context, sql string, arguments ...any) (int64, error)
	Query(ctx context.Context, sql string, args ...any) (Rows, error)
	QueryRow(ctx context.Context, sql string, args ...any) Row
	Begin(ctx context.Context) (Tx, error)
	Close()
	RunMigrations(ctx context.Context) error
}

func recordQuery(ctx context.Context, dbType string, err error, start time.Time) {
	duration := time.Since(start).Seconds()
	attrs := metric.WithAttributes(attribute.String("db.type", dbType))
	queryDuration.Record(ctx, duration, attrs)
	if err != nil {
		queryErrorCount.Add(ctx, 1, attrs)
	}
}

// NewProvider returns a Provider implementation.
// If DATABASE_URL starts with sqlite:// or is empty, it returns a SQLite provider.
// Otherwise, it returns a PostgreSQL provider.
func NewProvider(ctx context.Context) (Provider, error) {
	dsn := os.Getenv("DATABASE_URL")
	if dsn == "" || strings.HasPrefix(dsn, "sqlite://") {
		return NewSQLite(ctx, dsn)
	}
	return NewPostgres(ctx, dsn)
}
