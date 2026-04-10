package hub

import (
	"context"
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

type ragSyncServiceImpl struct {
	dbWrapper            *db.DB
	recordsSyncedCounter metric.Int64Counter
	syncErrorsCounter    metric.Int64Counter
}

func NewRAGSyncService(dbWrapper *db.DB) (RAGSyncService, error) {
	meter := otel.GetMeterProvider().Meter("github.com/onehumancorp/mono/ohc")
	syncedCounter, err := meter.Int64Counter("rag_records_synced_total")
	if err != nil {
		return nil, err
	}
	errorCounter, err := meter.Int64Counter("rag_sync_errors_total")
	if err != nil {
		return nil, err
	}

	return &ragSyncServiceImpl{
		dbWrapper:            dbWrapper,
		recordsSyncedCounter: syncedCounter,
		syncErrorsCounter:    errorCounter,
	}, nil
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.dbWrapper.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var vector []byte
		if err := rows.Scan(&r.ID, &r.Context, &vector, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		r.Vector = vector
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
		_, err := s.dbWrapper.Exec(ctx, query, id)
		if err != nil {
			s.syncErrorsCounter.Add(ctx, 1)
			return err
		}
	}
	s.recordsSyncedCounter.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}
	for _, r := range records {
		query := "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET content = $2, embedding = $3, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP"
		_, err := s.dbWrapper.Exec(ctx, query, r.ID, r.Context, r.Vector)
		if err != nil {
			s.syncErrorsCounter.Add(ctx, 1)
			return err
		}
	}
	s.recordsSyncedCounter.Add(ctx, int64(len(records)))
	return nil
}
