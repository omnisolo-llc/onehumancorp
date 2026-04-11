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
			WHERE status = 'PENDING' AND organization_id = 'system'
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

func (r *PgRows) Columns() ([]string, error) {
	var cols []string
	for _, fd := range r.rows.FieldDescriptions() {
		cols = append(cols, string(fd.Name))
	}
	return cols, nil
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

func (p *PgProvider) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    // PG is typically the cloud side; this method might be unused or just a stub, but let's implement it for completeness.
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
	rows, err := p.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var status string
		var lastSync *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &status, &lastSync); err != nil {
			return nil, err
		}
		r.SyncStatus = RAGSyncStatus(status)
		if lastSync != nil {
			r.LastSyncAt = *lastSync
		}
		records = append(records, r)
	}
	return records, nil
}

func (p *PgProvider) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	// For simplicity in Postgres provider, run multiple updates or use ANY array if driver supports it.
    // Given the constraints and typical SQL, using a loop inside a transaction is robust.
	tx, err := p.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	stmt := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = NOW() WHERE memory_id = $1`
	for _, id := range ids {
		_, err := tx.Exec(ctx, stmt, id)
		if err != nil {
			return err
		}
	}
	return tx.Commit(ctx)
}

func (p *PgProvider) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}
	tx, err := p.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	stmt := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, 'synced', NOW())
		ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = 'synced',
			last_sync_at = NOW()
	`
	for _, r := range records {
		_, err := tx.Exec(ctx, stmt, r.ID, r.Context, r.Vector)
		if err != nil {
			return err
		}
	}
	return tx.Commit(ctx)
}
