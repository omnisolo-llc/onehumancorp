package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"time"

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

var (
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func InitMetrics(meter metric.Meter) error {
	var err error
	if ragRecordsSyncedTotal == nil {
		ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total")
		if err != nil {
			return err
		}
	}
	if ragSyncErrorsTotal == nil {
		ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total")
		if err != nil {
			return err
		}
	}
	return nil
}

type ragSyncService struct {
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) RAGSyncService {
	return &ragSyncService{
		db: db,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, "SELECT id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		var syncStatus string
		var vectorBytes []byte

		if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &syncStatus, &lastSyncAt); err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return nil, err
		}
		rec.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}

		if len(vectorBytes) > 0 {
			var vec []float32
			if err := json.Unmarshal(vectorBytes, &vec); err == nil {
				rec.Vector = vec
			}
		}

		records = append(records, rec)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// For standard sql we can't easily use = ANY($2). In a production scenario
	// we'd build the query dynamically or use IN (?, ?).
	// For simplicity in bridging both drivers here, we will iterate.
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	now := time.Now().UTC()
	for _, id := range ids {
		_, err := tx.ExecContext(ctx, "UPDATE swarm_memory SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2", now, id)
		if err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(); err != nil {
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
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	now := time.Now().UTC()
	for _, rec := range records {
		vectorBytes, err := json.Marshal(rec.Vector)
		if err != nil {
			vectorBytes = []byte{}
		}

		_, err = tx.ExecContext(ctx, "INSERT INTO swarm_memory (id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', $4) ON CONFLICT (id) DO UPDATE SET context = $2, vector_embedding = $3, sync_status = 'synced', last_sync_at = $4", rec.ID, rec.Context, string(vectorBytes), now)
		if err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(); err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}

	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return nil
}
