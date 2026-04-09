package hub

import (
	"context"
	"encoding/json"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                 = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"ohc.rag.sync.success.total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		slog.Error("Failed to create ragRecordsSyncedTotal counter", "error", err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"ohc.rag.sync.errors.total",
		metric.WithDescription("Total number of RAG records sync errors"),
	)
	if err != nil {
		slog.Error("Failed to create ragSyncErrorsTotal counter", "error", err)
	}
}

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

type sqlRAGSyncService struct {
	db *db.DB
}

func NewSQLRAGSyncService(database *db.DB) RAGSyncService {
	return &sqlRAGSyncService{db: database}
}

func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		LIMIT $2
	`, string(SyncStatusPending), limit)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "FetchPendingSyncs")))
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var embeddingStr *string
		var syncStatus string
		var lastSyncAt *time.Time

		err := rows.Scan(&rec.ID, &rec.Context, &embeddingStr, &syncStatus, &lastSyncAt)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "FetchPendingSyncs_Scan")))
			return nil, err
		}

		rec.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}

		if embeddingStr != nil && *embeddingStr != "" {
			err = json.Unmarshal([]byte(*embeddingStr), &rec.Vector)
			if err != nil {
				slog.WarnContext(ctx, "Failed to unmarshal vector", "id", rec.ID, "error", err)
				ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "FetchPendingSyncs_Unmarshal")))
			}
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "FetchPendingSyncs_RowsErr")))
		return nil, err
	}

	return records, nil
}

func (s *sqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "MarkSynced_Begin")))
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, `
			UPDATE autodream_memories
			SET sync_status = $1, last_sync_at = $2
			WHERE id = $3
		`, string(SyncStatusSynced), now, id)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "MarkSynced_Exec")))
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "MarkSynced_Commit")))
		return err
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)), metric.WithAttributes(attribute.String("operation", "MarkSynced")))
	return nil
}

func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "ProcessIncomingSync_Begin")))
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		vectorBytes, _ := json.Marshal(rec.Vector)
		vectorStr := string(vectorBytes)

		// SQLite UPSERT
		_, err := tx.Exec(ctx, `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`, rec.ID, rec.Context, vectorStr, string(rec.SyncStatus), rec.LastSyncAt)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "ProcessIncomingSync_Exec")))
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "ProcessIncomingSync_Commit")))
		return err
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(records)), metric.WithAttributes(attribute.String("operation", "ProcessIncomingSync")))
	return nil
}
