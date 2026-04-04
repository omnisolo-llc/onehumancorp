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

func (p *PgProvider) Close() {
	p.pool.Close()
}

func (p *PgProvider) AcquireTask(ctx context.Context, agentID string) (*TaskRecord, error) {
	start := time.Now()
	tx, err := p.Begin(ctx)
	if err != nil {
		trackQuery(ctx, "AcquireTask", err, time.Since(start))
		return nil, err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE tasks
		SET status = 'RUNNING', agent_id = $1, updated_at = NOW()
		WHERE id = (
			SELECT id FROM tasks
			WHERE status = 'PENDING'
			ORDER BY created_at ASC
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		)
		RETURNING id, parent_task_id, agent_id, status, payload, created_at, updated_at
	`

	var t TaskRecord
	err = tx.QueryRow(ctx, query, agentID).Scan(
		&t.ID, &t.ParentTaskID, &t.AgentID, &t.Status, &t.Payload, &t.CreatedAt, &t.UpdatedAt,
	)
	if err != nil {
		// No rows is fine, just return nil
		if err.Error() == "no rows in result set" {
			trackQuery(ctx, "AcquireTask", nil, time.Since(start))
			return nil, nil
		}
		trackQuery(ctx, "AcquireTask", err, time.Since(start))
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		trackQuery(ctx, "AcquireTask_Commit", err, time.Since(start))
		return nil, err
	}

	trackQuery(ctx, "AcquireTask", nil, time.Since(start))
	return &t, nil
}

func (p *PgProvider) IsSQLite() bool {
	return false
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
