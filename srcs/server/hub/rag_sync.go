package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"
	"time"

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

type SyncService struct {
	db *sql.DB
}

func NewSyncService(db *sql.DB) *SyncService {
	return &SyncService{db: db}
}

func (s *SyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		ragSyncErrorsCounter.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vectorStr string
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSyncAt); err != nil {
			ragSyncErrorsCounter.Add(ctx, 1)
			return nil, err
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}

		if vectorStr != "" {
			if strings.HasPrefix(vectorStr, "[") {
				if err := json.Unmarshal([]byte(vectorStr), &r.Vector); err != nil {
					ragSyncErrorsCounter.Add(ctx, 1)
					return nil, err
				}
			}
		}

		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *SyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id IN ("
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		args[i] = id
		if i > 0 {
			query += ", "
		}
		query += fmt.Sprintf("$%d", i+1)
	}
	query += ")"

	_, err := s.db.ExecContext(ctx, query, args...)
	if err != nil {
		ragSyncErrorsCounter.Add(ctx, 1)
		return err
	}
	ragRecordsSyncedCounter.Add(ctx, int64(len(ids)))
	return nil
}

func (s *SyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		var vectorStr string
		if r.Vector != nil {
			b, err := json.Marshal(r.Vector)
			if err != nil {
				ragSyncErrorsCounter.Add(ctx, 1)
				return err
			}
			vectorStr = string(b)
		}

		query := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT (id) DO UPDATE SET
		content = EXCLUDED.content,
		embedding = EXCLUDED.embedding,
		sync_status = EXCLUDED.sync_status,
		last_sync_at = EXCLUDED.last_sync_at`

		_, err := s.db.ExecContext(ctx, query, r.ID, r.Context, vectorStr, r.SyncStatus, time.Now())
		if err != nil {
			ragSyncErrorsCounter.Add(ctx, 1)
			return err
		}
	}
	return nil
}

var (
	meter                   = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedCounter metric.Int64Counter
	ragSyncErrorsCounter    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedCounter, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synchronized"),
	)
	if err != nil {
		panic(err)
	}

	ragSyncErrorsCounter, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		panic(err)
	}
}
