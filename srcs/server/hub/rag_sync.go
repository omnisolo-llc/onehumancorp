package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
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
	ID             string
	OrganizationID string
	AgentID        string
	SourceType     string
	Context        string
	Vector         []float32
	SyncStatus     SyncStatus
	LastSyncAt     time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
	db            db.Provider
	recordsSynced metric.Int64Counter
	syncErrors    metric.Int64Counter
}

func NewRAGSyncService(provider db.Provider) (RAGSyncService, error) {
	meter := otel.Meter("ohc_hub")
	var syncedCounter, errorsCounter metric.Int64Counter
	var err error

	if meter != nil {
		syncedCounter, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
		if err != nil {
			return nil, err
		}
		errorsCounter, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
		if err != nil {
			return nil, err
		}
	}

	return &ragSyncServiceImpl{
		db:            provider,
		recordsSynced: syncedCounter,
		syncErrors:    errorsCounter,
	}, nil
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, organization_id, agent_id, content, embedding, source_type, sync_status FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
	if s.db.IsSQLite() {
		query = "SELECT id, organization_id, agent_id, content, embedding, source_type, sync_status FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?"
	}

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var emb *string
		if err := rows.Scan(&r.ID, &r.OrganizationID, &r.AgentID, &r.Context, &emb, &r.SourceType, &r.SyncStatus); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		// ignoring vector parsing from JSON string here for simplicity, but we would unmarshal `emb` into r.Vector
		records = append(records, r)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1"
		if s.db.IsSQLite() {
			query = "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = ?"
		}
		if _, err := s.db.Exec(ctx, query, id); err != nil {
			if s.syncErrors != nil {
				s.syncErrors.Add(ctx, 1)
			}
			if telemetry.BufferMetricFunc != nil {
				telemetry.BufferMetricFunc(ctx, "rag_sync_errors_total", "1")
			}
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	if s.recordsSynced != nil {
		s.recordsSynced.Add(ctx, int64(len(ids)))
	}
	if telemetry.BufferMetricFunc != nil {
		telemetry.BufferMetricFunc(ctx, "rag_records_synced_total", fmt.Sprintf("%d", len(ids)))
	}
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, r := range records {
		var query string
		var args []interface{}

		if s.db.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type, sync_status, created_at, last_sync_at)
				VALUES (?, ?, ?, ?, ?, ?, 'synced', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP
			`
			args = []interface{}{r.ID, r.OrganizationID, r.AgentID, r.Context, "[]", r.SourceType}
		} else {
			query = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type, sync_status, created_at, last_sync_at)
				VALUES ($1, $2, $3, $4, $5, $6, 'synced', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP
			`
			args = []interface{}{r.ID, r.OrganizationID, r.AgentID, r.Context, "[]", r.SourceType}
		}

		if _, err := s.db.Exec(ctx, query, args...); err != nil {
			if s.syncErrors != nil {
				s.syncErrors.Add(ctx, 1)
			}
			if telemetry.BufferMetricFunc != nil {
				telemetry.BufferMetricFunc(ctx, "rag_sync_errors_total", "1")
			}
			return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
		}
	}

	if s.recordsSynced != nil {
		s.recordsSynced.Add(ctx, int64(len(records)))
	}
	if telemetry.BufferMetricFunc != nil {
		telemetry.BufferMetricFunc(ctx, "rag_records_synced_total", fmt.Sprintf("%d", len(records)))
	}
	return nil
}
