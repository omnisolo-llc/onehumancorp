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
	meter                   = otel.Meter("github.com/onehumancorp/mono/ohc")
	recordsSyncedTotal      metric.Int64Counter
	syncErrorsTotal         metric.Int64Counter
)

func init() {
	var err error
	recordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		panic(err)
	}
	syncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		panic(err)
	}
}

type DefaultRAGSyncService struct{
	db *db.DB
}

func NewDefaultRAGSyncService(database *db.DB) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: database}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if s.db == nil {
        return nil, fmt.Errorf("database not initialized")
    }

	query := `SELECT id, context, vector, sync_status, last_sync_at FROM rag_memories WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorStr *string
		var lastSyncAt *time.Time
        var syncStatus *string

		err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &syncStatus, &lastSyncAt)
		if err != nil {
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}

        if syncStatus != nil {
            rec.SyncStatus = SyncStatus(*syncStatus)
        } else {
            rec.SyncStatus = SyncStatusPending
        }

        if lastSyncAt != nil {
            rec.LastSyncAt = *lastSyncAt
        }

		if vectorStr != nil && *vectorStr != "" {
			err = json.Unmarshal([]byte(*vectorStr), &rec.Vector)
			if err != nil {
				return nil, fmt.Errorf("failed to unmarshal vector string: %w", err)
			}
		}

		records = append(records, rec)
	}

	if err = rows.Err(); err != nil {
        return nil, fmt.Errorf("rows iteration error: %w", err)
    }

	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if s.db == nil {
        return fmt.Errorf("database not initialized")
    }

    if len(ids) == 0 {
        return nil
    }

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `UPDATE rag_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2`

	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			return fmt.Errorf("failed to execute mark synced update for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit mark synced transaction: %w", err)
	}

	recordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if s.db == nil {
        return fmt.Errorf("database not initialized")
    }

    if len(records) == 0 {
        return nil
    }

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

    var query string
    if s.db.IsSQLite() {
         query = `
		    INSERT INTO rag_memories (id, context, vector, sync_status, last_sync_at)
		    VALUES ($1, $2, $3, $4, $5)
		    ON CONFLICT(id) DO UPDATE SET
			    context=excluded.context,
			    vector=excluded.vector,
			    sync_status=excluded.sync_status,
			    last_sync_at=excluded.last_sync_at
	    `
    } else {
        query = `
		    INSERT INTO rag_memories (id, context, vector, sync_status, last_sync_at)
		    VALUES ($1, $2, $3, $4, $5)
		    ON CONFLICT(id) DO UPDATE SET
			    context=EXCLUDED.context,
			    vector=EXCLUDED.vector,
			    sync_status=EXCLUDED.sync_status,
			    last_sync_at=EXCLUDED.last_sync_at
	    `
    }

	now := time.Now()
	for _, rec := range records {
		vectorBytes, err := json.Marshal(rec.Vector)
		if err != nil {
            syncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to marshal vector for id %s: %w", rec.ID, err)
		}

		_, err = tx.Exec(ctx, query, rec.ID, rec.Context, string(vectorBytes), "synced", now)
		if err != nil {
            syncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to execute upsert for id %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit process incoming sync transaction: %w", err)
	}

	return nil
}
