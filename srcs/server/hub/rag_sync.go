package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
	ragSyncErrorsTotal, _    = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
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
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncService struct {
	db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &ragSyncService{db: db}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, embedding, sync_status, COALESCE(last_sync_timestamp, '0001-01-01T00:00:00Z') FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var embeddingStr sql.NullString
		var lastSync interface{}
		if err := rows.Scan(&record.ID, &record.Context, &embeddingStr, &record.SyncStatus, &lastSync); err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return nil, err
		}
		if embeddingStr.Valid && embeddingStr.String != "" {
			json.Unmarshal([]byte(embeddingStr.String), &record.Vector)
		}
		switch v := lastSync.(type) {
		case time.Time:
			record.LastSyncAt = v
		case string:
			if parsed, err := time.Parse(time.RFC3339, v); err == nil {
				record.LastSyncAt = parsed
			}
		case []byte:
			if parsed, err := time.Parse(time.RFC3339, string(v)); err == nil {
				record.LastSyncAt = parsed
			}
		}
		records = append(records, record)
	}
	if err = rows.Err(); err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, err
	}
	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	// Create parameter placeholders
	placeholders := make([]string, len(ids))
	args := make([]any, len(ids))
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args[i] = id
	}

	query := fmt.Sprintf("UPDATE autodream_memories SET sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP WHERE id IN (%s)", strings.Join(placeholders, ","))
	_, err := s.db.Exec(ctx, query, args...)
	if err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, record := range records {
		embeddingBytes, _ := json.Marshal(record.Vector)
		query := `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_timestamp)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = 'synced',
				last_sync_timestamp = EXCLUDED.last_sync_timestamp
		`
		_, err := s.db.Exec(ctx, query, record.ID, record.Context, string(embeddingBytes), record.LastSyncAt)
		if err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}
	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return nil
}
