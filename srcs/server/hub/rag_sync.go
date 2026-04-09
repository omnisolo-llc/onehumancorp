package hub

import (
	"context"
	"database/sql"
	"fmt"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
	"strings"
	"time"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var meter = otel.Meter("srcs/server/hub")

var (
	RecordsSyncedTotal, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced to the cloud"),
	)
	SyncErrorsTotal, _ = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
)

// Concrete implementation
type RAGSyncProvider struct {
	db *sql.DB
}

func NewRAGSyncProvider(db *sql.DB) *RAGSyncProvider {
	return &RAGSyncProvider{db: db}
}

func (p *RAGSyncProvider) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
	rows, err := p.db.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSync sql.NullTime
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.SyncStatus, &lastSync); err != nil {
			return nil, err
		}
		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}
		records = append(records, rec)
	}
	return records, nil
}

func (p *RAGSyncProvider) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Construct placeholders dynamically for IN clause
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args[i] = id
	}

	query := fmt.Sprintf(`UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id IN (%s)`, strings.Join(placeholders, ","))
	_, err := p.db.ExecContext(ctx, query, args...)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return err
	}
	RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (p *RAGSyncProvider) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := p.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	// Using bulk insert strategy or simple loop depending on constraints
	// Here we use simple loop for robustness in hybrid (SQLite/Postgres) scenarios
	for _, rec := range records {
		// Upsert behavior
		query := `
            INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
            VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
            ON CONFLICT(memory_id) DO UPDATE SET
            context = excluded.context,
            sync_status = 'synced',
            last_sync_at = CURRENT_TIMESTAMP
        `
		_, err := tx.ExecContext(ctx, query, rec.ID, rec.Context)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}
	if err := tx.Commit(); err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return err
	}
	RecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
