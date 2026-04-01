package db

import (
	"context"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

// PgProvider implements the Provider interface using pgxpool.
type PgProvider struct {
	pool *pgxpool.Pool
}

func NewPgProvider(pool *pgxpool.Pool) *PgProvider {
	return &PgProvider{pool: pool}
}

func (p *PgProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	start := time.Now()
	tag, err := p.pool.Exec(ctx, sql, arguments...)
	trackQuery(ctx, "Exec", err, time.Since(start))
	if err != nil {
		return 0, err
	}
	return tag.RowsAffected(), nil
}

func (p *PgProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (Rows, error) {
	start := time.Now()
	rows, err := p.pool.Query(ctx, sql, optionsAndArgs...)
	trackQuery(ctx, "Query", err, time.Since(start))
	if err != nil {
		return nil, err
	}
	return &PgRows{rows: rows}, nil
}

func (p *PgProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) Row {
	start := time.Now()
	row := p.pool.QueryRow(ctx, sql, optionsAndArgs...)
	trackQuery(ctx, "QueryRow", nil, time.Since(start))
	return &PgRow{row: row}
}

func (p *PgProvider) Begin(ctx context.Context) (Tx, error) {
	start := time.Now()
	tx, err := p.pool.Begin(ctx)
	trackQuery(ctx, "Begin", err, time.Since(start))
	if err != nil {
		return nil, err
	}
	return &PgTx{tx: tx}, nil
}

func (p *PgProvider) IsSQLite() bool {
	return false
}

func (p *PgProvider) Close() {
	p.pool.Close()
}

// PgRows implements Rows using pgx.Rows.
type PgRows struct {
	rows pgx.Rows
}

func (r *PgRows) Next() bool {
	return r.rows.Next()
}

func (r *PgRows) Scan(dest ...any) error {
	return r.rows.Scan(dest...)
}

func (r *PgRows) Close() {
	r.rows.Close()
}

func (r *PgRows) Err() error {
	return r.rows.Err()
}

// PgRow implements Row using pgx.Row.
type PgRow struct {
	row pgx.Row
}

func (r *PgRow) Scan(dest ...any) error {
	return r.row.Scan(dest...)
}

// PgTx implements Tx using pgx.Tx.
type PgTx struct {
	tx pgx.Tx
}

func (t *PgTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	start := time.Now()
	tag, err := t.tx.Exec(ctx, sql, arguments...)
	trackQuery(ctx, "Tx.Exec", err, time.Since(start))
	if err != nil {
		return 0, err
	}
	return tag.RowsAffected(), nil
}

func (t *PgTx) Query(ctx context.Context, sql string, optionsAndArgs ...any) (Rows, error) {
	start := time.Now()
	rows, err := t.tx.Query(ctx, sql, optionsAndArgs...)
	trackQuery(ctx, "Tx.Query", err, time.Since(start))
	if err != nil {
		return nil, err
	}
	return &PgRows{rows: rows}, nil
}

func (t *PgTx) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) Row {
	start := time.Now()
	row := t.tx.QueryRow(ctx, sql, optionsAndArgs...)
	trackQuery(ctx, "Tx.QueryRow", nil, time.Since(start))
	return &PgRow{row: row}
}

func (t *PgTx) Commit(ctx context.Context) error {
	return t.tx.Commit(ctx)
}

func (t *PgTx) Rollback(ctx context.Context) error {
	return t.tx.Rollback(ctx)
}
