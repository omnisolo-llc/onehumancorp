package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
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
	dbProvider db.Provider
}

// NewRAGSyncService creates a new instance of RAGSyncService
func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncService{
		dbProvider: dbProvider,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit <= 0 {
		limit = 10
	}

	query := `
		SELECT key, value, sync_status, last_sync_at
		FROM swarm_memory
		WHERE sync_status = 'pending' OR sync_status IS NULL
		ORDER BY updated_at ASC
		LIMIT $1
	`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var val string
		var status sql.NullString
		var lastSync sql.NullTime

		if err := rows.Scan(&r.ID, &val, &status, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}

		r.Context = val
		if status.Valid {
			r.SyncStatus = SyncStatus(status.String)
		} else {
			r.SyncStatus = SyncStatusPending
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}

		// Also try to fetch vector embedding if it exists
		vectorQuery := `SELECT vector_embedding FROM swarm_memory_embeddings WHERE memory_id = $1`
		vRow := s.dbProvider.QueryRow(ctx, vectorQuery, r.ID)
		var vBytes []byte
		err := vRow.Scan(&vBytes)
		if err == nil && len(vBytes) > 0 {
			var vec []float32
			if err := json.Unmarshal(vBytes, &vec); err == nil {
				r.Vector = vec
			}
		}

		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}
	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now().UTC()
	for _, id := range ids {
		query := `
			UPDATE swarm_memory
			SET sync_status = $1, last_sync_at = $2
			WHERE key = $3
		`
		_, err := tx.Exec(ctx, query, string(SyncStatusSynced), now, id)
		if err != nil {
			telemetry.RecordRAGSyncErrors(ctx, 1)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRAGRecordsSynced(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now().UTC()
	for _, r := range records {
		query := `
			INSERT INTO swarm_memory (key, value, sync_status, last_sync_at, updated_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (key) DO UPDATE
			SET value = excluded.value,
				sync_status = excluded.sync_status,
				last_sync_at = excluded.last_sync_at,
				updated_at = excluded.updated_at
		`
		_, err := tx.Exec(ctx, query, r.ID, r.Context, string(SyncStatusSynced), now, now)
		if err != nil {
			telemetry.RecordRAGSyncErrors(ctx, 1)
			return fmt.Errorf("failed to upsert record %s: %w", r.ID, err)
		}

		if len(r.Vector) > 0 {
			vBytes, err := json.Marshal(r.Vector)
			if err == nil {
				vQuery := `
					INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, created_at)
					VALUES ($1, $2, $3, $4)
					ON CONFLICT (memory_id) DO UPDATE
					SET context = excluded.context,
						vector_embedding = excluded.vector_embedding
				`
				_, err = tx.Exec(ctx, vQuery, r.ID, r.Context, vBytes, now)
				if err != nil {
					telemetry.RecordRAGSyncErrors(ctx, 1)
					return fmt.Errorf("failed to upsert embedding for %s: %w", r.ID, err)
				}
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRAGRecordsSynced(ctx, int64(len(records)))
	return nil
}
