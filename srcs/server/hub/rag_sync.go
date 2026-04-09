package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
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
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncService struct {
	db *db.DB
}

func NewRAGSyncService(db *db.DB) RAGSyncService {
	return &ragSyncService{
		db: db,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("querying pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var embeddingStr sql.NullString
		var syncStatus sql.NullString
		var lastSyncAt sql.NullTime

		err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &syncStatus, &lastSyncAt)
		if err != nil {
			return nil, fmt.Errorf("scanning record: %w", err)
		}

		if embeddingStr.Valid && embeddingStr.String != "" {
			var vec []float32
			if err := json.Unmarshal([]byte(embeddingStr.String), &vec); err != nil {
				// We can just skip invalid embeddings if necessary, but returning error for strictness
				return nil, fmt.Errorf("unmarshaling embedding vector: %w", err)
			}
			r.Vector = vec
		}

		if syncStatus.Valid {
			r.SyncStatus = SyncStatus(syncStatus.String)
		} else {
			r.SyncStatus = SyncStatusPending
		}

		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}

		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("iterating records: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2`

	now := time.Now()
	for _, id := range ids {
		_, err := s.db.Exec(ctx, query, now, id)
		if err != nil {
			return fmt.Errorf("updating sync status for id %s: %w", id, err)
		}
	}

	if RecordsSyncedCounter != nil {
		RecordsSyncedCounter.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	// Basic last write wins logic by upserting
	for _, r := range records {
		var vecStr string
		if len(r.Vector) > 0 {
			b, err := json.Marshal(r.Vector)
			if err != nil {
				return fmt.Errorf("marshaling vector: %w", err)
			}
			vecStr = string(b)
		}

		// Using simple UPSERT compatible syntax
		query := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
		`
		var embeddingArg interface{} = nil
		if vecStr != "" {
			embeddingArg = vecStr
		}

		_, err := s.db.Exec(ctx, query, r.ID, r.Context, embeddingArg, string(r.SyncStatus), r.LastSyncAt)
		if err != nil {
			if SyncErrorsCounter != nil {
				SyncErrorsCounter.Add(ctx, 1)
			}
			return fmt.Errorf("upserting incoming record id %s: %w", r.ID, err)
		}
	}

	return nil
}

var (
	meter                = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RecordsSyncedCounter metric.Int64Counter
	SyncErrorsCounter    metric.Int64Counter
)

func init() {
	var err error
	RecordsSyncedCounter, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		panic(err)
	}
	SyncErrorsCounter, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		panic(err)
	}
}
