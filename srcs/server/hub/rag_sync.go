package hub

import (
	"context"
	"database/sql"
	"encoding/json"

	"strings"
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

type DefaultRAGSyncService struct {
	provider        db.Provider
	meter           metric.Meter
	syncedTotal     metric.Int64Counter
	syncErrorsTotal metric.Int64Counter
}

func NewRAGSyncService(provider db.Provider) (*DefaultRAGSyncService, error) {
	meter := otel.GetMeterProvider().Meter("github.com/onehumancorp/mono/srcs/server/hub")
	syncedTotal, err := meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records successfully synced"))
	if err != nil {
		return nil, err
	}
	syncErrorsTotal, err := meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
	if err != nil {
		return nil, err
	}

	return &DefaultRAGSyncService{
		provider:        provider,
		meter:           meter,
		syncedTotal:     syncedTotal,
		syncErrorsTotal: syncErrorsTotal,
	}, nil
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		ORDER BY created_at ASC
		LIMIT $2
	`
	rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		s.syncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		var vectorStr sql.NullString
		var statusStr string

		if err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &statusStr, &lastSyncAt); err != nil {
			s.syncErrorsTotal.Add(ctx, 1)
			return nil, err
		}

		rec.SyncStatus = SyncStatus(statusStr)
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}

		if vectorStr.Valid && vectorStr.String != "" {
			// Parsing "[1.0, 2.0, 3.0]" or JSON arrays. pgvector uses `[1.0,2.0]` format, which matches JSON
			var vec []float32
			// Replace brackets for SQLite cases if it happens to just be CSV, but pgvector/json uses []
			cleanedStr := vectorStr.String
			if !strings.HasPrefix(cleanedStr, "[") {
				cleanedStr = "[" + cleanedStr + "]"
			}
			if err := json.Unmarshal([]byte(cleanedStr), &vec); err == nil {
				rec.Vector = vec
			}
		}
		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		s.syncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	query := `
		UPDATE autodream_memories
		SET sync_status = $1, last_sync_at = $2
		WHERE id = $3
	`

	successCount := 0
	for _, id := range ids {
		_, err := tx.Exec(ctx, query, string(SyncStatusSynced), now, id)
		if err != nil {
			s.syncErrorsTotal.Add(ctx, 1)
			return err
		}
		successCount++
	}

	if err := tx.Commit(ctx); err != nil {
		s.syncErrorsTotal.Add(ctx, 1)
		return err
	}

	s.syncedTotal.Add(ctx, int64(successCount))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		s.syncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`

	successCount := 0
	for _, rec := range records {
		var vectorStr *string
		if len(rec.Vector) > 0 {
			// Need to convert float32 to json string
			bytes, err := json.Marshal(rec.Vector)
			if err == nil {
				s := string(bytes)
				vectorStr = &s
			}
		}

		status := string(rec.SyncStatus)
		if status == "" {
			status = string(SyncStatusSynced)
		}

		var lastSync interface{}
		if !rec.LastSyncAt.IsZero() {
			lastSync = rec.LastSyncAt
		} else {
			lastSync = time.Now()
		}

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vectorStr, status, lastSync)
		if err != nil {
			s.syncErrorsTotal.Add(ctx, 1)
			return err
		}
		successCount++
	}

	if err := tx.Commit(ctx); err != nil {
		s.syncErrorsTotal.Add(ctx, 1)
		return err
	}

	s.syncedTotal.Add(ctx, int64(successCount))
	return nil
}
