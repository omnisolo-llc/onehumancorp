package hub

import (
	"context"
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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type sqlRAGSyncService struct {
	db db.Provider
}

func NewSQLRAGSyncService(db db.Provider) RAGSyncService {
	return &sqlRAGSyncService{db: db}
}

func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vecStr *string
		var status *string
		var lastSync *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &vecStr, &status, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}
		if vecStr != nil {
			if err := json.Unmarshal([]byte(*vecStr), &r.Vector); err != nil {
				// ignore parse error or handle it
			}
		}
		if status != nil {
			r.SyncStatus = SyncStatus(*status)
		} else {
			r.SyncStatus = SyncStatusPending
		}
		if lastSync != nil {
			r.LastSyncAt = *lastSync
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
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
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1"
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
		ragRecordsSyncedTotal.Add(ctx, 1)
	}

	return tx.Commit(ctx)
}

func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var vecStr *string
		if len(r.Vector) > 0 {
			b, err := json.Marshal(r.Vector)
			if err == nil {
				vs := string(b)
				vecStr = &vs
			}
		}

		var query string
		if s.db.IsSQLite() {
			// Upsert in SQLite
			query = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT(id) DO UPDATE SET
				content = excluded.content,
				embedding = excluded.embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
			`
			_, err = tx.Exec(ctx, query, r.ID, r.Context, vecStr)
		} else {
			// Upsert in Postgres
			query = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, CASE WHEN $3::text IS NULL THEN NULL ELSE $3::text::vector END, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT(id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
			`
			_, err = tx.Exec(ctx, query, r.ID, r.Context, vecStr)
		}

		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert incoming record id %s: %w", r.ID, err)
		}
		ragRecordsSyncedTotal.Add(ctx, 1)
	}

	return tx.Commit(ctx)
}

var (
	meter                 = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced successfully"),
	)
	if err != nil {
		panic(err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		panic(err)
	}
}
