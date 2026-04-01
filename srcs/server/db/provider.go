package db

import (
	"context"
	"database/sql"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

// Provider abstracts database operations to support both
// PostgreSQL and SQLite gracefully.
type Provider interface {
	ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error)
	QueryContext(ctx context.Context, query string, args ...any) (*sql.Rows, error)
	QueryRowContext(ctx context.Context, query string, args ...any) *sql.Row
	BeginTx(ctx context.Context, opts *sql.TxOptions) (*sql.Tx, error)
	Close() error
	IsPostgres() bool
}

// sqlProvider wraps *sql.DB to implement Provider
type sqlProvider struct {
	*sql.DB
	isPg bool
	meter metric.Meter
	queryLatency metric.Float64Histogram
	errorCounter metric.Int64Counter
}

func NewSQLProvider(db *sql.DB, isPg bool) Provider {
	meter := otel.Meter("db_provider")
	latency, _ := meter.Float64Histogram("db_query_latency_ms", metric.WithDescription("Latency of DB queries"))
	errors, _ := meter.Int64Counter("db_query_errors", metric.WithDescription("Count of DB errors"))

	return &sqlProvider{
		DB: db,
		isPg: isPg,
		meter: meter,
		queryLatency: latency,
		errorCounter: errors,
	}
}

func (p *sqlProvider) IsPostgres() bool {
	return p.isPg
}

func (p *sqlProvider) recordMetrics(start time.Time, err error, op string) {
	latency := float64(time.Since(start).Milliseconds())
	attrs := metric.WithAttributes(attribute.String("operation", op))
	p.queryLatency.Record(context.Background(), latency, attrs)
	if err != nil {
		p.errorCounter.Add(context.Background(), 1, attrs)
	}
}

func (p *sqlProvider) ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error) {
	start := time.Now()
	res, err := p.DB.ExecContext(ctx, query, args...)
	p.recordMetrics(start, err, "exec")
	return res, err
}

func (p *sqlProvider) QueryContext(ctx context.Context, query string, args ...any) (*sql.Rows, error) {
	start := time.Now()
	rows, err := p.DB.QueryContext(ctx, query, args...)
	p.recordMetrics(start, err, "query")
	return rows, err
}

func (p *sqlProvider) QueryRowContext(ctx context.Context, query string, args ...any) *sql.Row {
	start := time.Now()
	row := p.DB.QueryRowContext(ctx, query, args...)
	p.recordMetrics(start, row.Err(), "query_row")
	return row
}

func (p *sqlProvider) BeginTx(ctx context.Context, opts *sql.TxOptions) (*sql.Tx, error) {
	start := time.Now()
	tx, err := p.DB.BeginTx(ctx, opts)
	p.recordMetrics(start, err, "begin_tx")
	return tx, err
}
