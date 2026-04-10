package hub

import (
	"context"
	"time"
	"database/sql"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
	"github.com/onehumancorp/mono/srcs/server/db"
)

var (
	RecordsSyncedTotal metric.Int64Counter
	SyncErrorsTotal    metric.Int64Counter
)

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/ohc")

	var err error
	RecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
	if err != nil {
		// Intentionally ignoring error for initialization
	}

	SyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
	if err != nil {
		// Intentionally ignoring error for initialization
	}
}

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID           string
	Context      string
	Vector       []byte
	SyncStatus   SyncStatus
	LastSyncAt   time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DBBridgeSyncService struct {
	provider db.Provider
}

func NewDBBridgeSyncService(provider db.Provider) *DBBridgeSyncService {
	return &DBBridgeSyncService{provider: provider}
}

func (s *DBBridgeSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var records []RAGSyncRecord
	// We use the database provider to abstract away sqlite vs pgvector queries

	rows, err := s.provider.Query(ctx, `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM consolidated_memory
		WHERE sync_status = 'pending'
		LIMIT $1
	`, limit)
	if err != nil {
		if err == sql.ErrNoRows {
			return records, nil
		}
		return nil, err
	}
	defer rows.Close()

	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		// Handle potential type mismatches with pgvector directly returning string or []byte
		err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt)
		if err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}

	return records, nil
}

func (s *DBBridgeSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		_, err := s.provider.Exec(ctx, `
			UPDATE consolidated_memory
			SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			WHERE id = $1
		`, id)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DBBridgeSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, r := range records {
		_, err := s.provider.Exec(ctx, `
			INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type, sync_status, last_sync_at)
			VALUES ($1, 'standalone', $2, $3, 'hybrid_sync', 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`, r.ID, r.Context, r.Vector)

		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	RecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
