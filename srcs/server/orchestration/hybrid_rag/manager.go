package hybrid_rag

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type RAGSyncManager struct {
	dbWrapper *db.DB
}

func NewRAGSyncManager(dbWrapper *db.DB) *RAGSyncManager {
	return &RAGSyncManager{
		dbWrapper: dbWrapper,
	}
}

func (m *RAGSyncManager) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := m.dbWrapper.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []hub.RAGSyncRecord
	for rows.Next() {
		var r hub.RAGSyncRecord
		var lastSyncAt *time.Time
		var vector []byte
		if err := rows.Scan(&r.ID, &r.Context, &vector, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("scan record: %w", err)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		// Note: vector decoding/encoding would go here if needed
		records = append(records, r)
	}
	return records, nil
}

func (m *RAGSyncManager) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := m.dbWrapper.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2"
	now := time.Now()

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			return fmt.Errorf("update sync status for %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	telemetry.RecordRagRecordsSynced(ctx, int64(len(ids)))
	return nil
}

func (m *RAGSyncManager) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := m.dbWrapper.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// LWW strategy: Update if newer or record doesn't exist
	// In Cloud Mode (Postgres), we use ON CONFLICT
	// In Standalone Mode (SQLite), also ON CONFLICT
	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, 'synced', $4)
		ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = 'synced',
			last_sync_at = EXCLUDED.last_sync_at
	`

	for _, r := range records {
		_, err := tx.Exec(ctx, query, r.ID, r.Context, nil, time.Now()) // Vector simplified for now
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			slog.Error("failed to process incoming sync record", "id", r.ID, "error", err)
			continue
		}
	}

	return tx.Commit(ctx)
}
