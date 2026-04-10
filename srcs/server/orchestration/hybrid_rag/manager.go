package hybrid_rag

import (
	"context"
	"database/sql"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type RAGSyncManager struct {
	dbProvider db.Provider
	orgID      string
}

func NewRAGSyncManager(dbProvider db.Provider, orgID string) *RAGSyncManager {
	if orgID == "" {
		orgID = "system"
	}
	return &RAGSyncManager{
		dbProvider: dbProvider,
		orgID:      orgID,
	}
}

func (m *RAGSyncManager) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	query := `SELECT memory_id, context, sync_status, last_sync_at
	          FROM swarm_memory_embeddings
	          WHERE sync_status = $1 AND organization_id = $2
	          LIMIT $3`

	rows, err := m.dbProvider.Query(ctx, query, hub.SyncStatusPending, m.orgID, limit)
	if err != nil {
		return nil, fmt.Errorf("query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []hub.RAGSyncRecord
	for rows.Next() {
		var r hub.RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("scan record: %w", err)
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}
	return records, nil
}

func (m *RAGSyncManager) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Simple loop update for compatibility across providers
	for _, id := range ids {
		query := `UPDATE swarm_memory_embeddings
		          SET sync_status = $1, last_sync_at = $2
		          WHERE memory_id = $3 AND organization_id = $4`
		_, err := m.dbProvider.Exec(ctx, query, hub.SyncStatusSynced, time.Now().UTC(), id, m.orgID)
		if err != nil {
			return fmt.Errorf("update record %s: %w", id, err)
		}
	}

	telemetry.RecordRAGRecordsSynced(ctx, int64(len(ids)))
	return nil
}

func (m *RAGSyncManager) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	for _, r := range records {
		query := `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at, organization_id)
		          VALUES ($1, $2, $3, $4, $5)
		          ON CONFLICT(memory_id) DO UPDATE SET
		          context = EXCLUDED.context,
		          sync_status = EXCLUDED.sync_status,
		          last_sync_at = EXCLUDED.last_sync_at`

		_, err := m.dbProvider.Exec(ctx, query, r.ID, r.Context, hub.SyncStatusSynced, time.Now().UTC(), m.orgID)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return fmt.Errorf("upsert incoming record %s: %w", r.ID, err)
		}
	}
	return nil
}
