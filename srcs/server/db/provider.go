package db

import (
	"context"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/db")
	queryDuration, _ = meter.Float64Histogram(
		"db.client.operation.duration",
		metric.WithDescription("Duration of database operations"),
		metric.WithUnit("s"),
	)
	queryErrors, _ = meter.Int64Counter(
		"db.client.operation.errors",
		metric.WithDescription("Number of database operation errors"),
	)
)

func trackQuery(ctx context.Context, operation string, err error, duration time.Duration) {
	attrs := []attribute.KeyValue{
		attribute.String("operation", operation),
	}
	if err != nil {
		attrs = append(attrs, attribute.Bool("error", true))
		queryErrors.Add(ctx, 1, metric.WithAttributes(attrs...))
	}
	queryDuration.Record(ctx, duration.Seconds(), metric.WithAttributes(attrs...))
}

// Provider abstracts the database connection pool.
// It supports basic query and transaction operations, allowing the underlying
// driver to be either PostgreSQL (pgx) or SQLite (database/sql).
type Provider interface {
	Exec(ctx context.Context, sql string, arguments ...any) (int64, error)
	Query(ctx context.Context, sql string, optionsAndArgs ...any) (Rows, error)
	QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) Row
	Begin(ctx context.Context) (Tx, error)
	Close()
	Ping(ctx context.Context) error
	IsSQLite() bool
	AcquireTask(ctx context.Context, organizationID, agentID string) (*TaskRecord, error)
}

// TaskRecord represents a task fetched from the queue.
type TaskRecord struct {
	ID           string
	ParentTaskID *string
	AgentID      *string
	Status       string
	Payload      *string
	CreatedAt    time.Time
	UpdatedAt    time.Time
}

// Rows abstracts multiple rows returned from a query.
type Rows interface {
	Next() bool
	Scan(dest ...any) error
	Close()
	Columns() ([]string, error)
	Err() error
}

// Row abstracts a single row returned from a query.
type Row interface {
	Scan(dest ...any) error
}

// Tx abstracts a database transaction.
type Tx interface {
	Exec(ctx context.Context, sql string, arguments ...any) (int64, error)
	Query(ctx context.Context, sql string, optionsAndArgs ...any) (Rows, error)
	QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) Row
	Commit(ctx context.Context) error
	Rollback(ctx context.Context) error
}
