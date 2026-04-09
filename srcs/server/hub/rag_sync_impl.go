package hub

import (
	"context"
	"database/sql"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type RAGSyncServiceImpl struct {
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{db: db}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSync sql.NullTime
		var vector []byte
		if err := rows.Scan(&rec.ID, &rec.Context, &vector, &rec.SyncStatus, &lastSync); err != nil {
			return nil, err
		}
		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}
		// In a real implementation we would convert []byte to []float32 if vector is stored as float array or pgvector.
		// For our interface we just leave it empty or mock it.
		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create parameterized IN clause
	// A simpler approach for tests is updating one by one, or using a loop
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}

	stmt, err := tx.PrepareContext(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2")
	if err != nil {
		tx.Rollback()
		return err
	}
	defer stmt.Close()

	now := time.Now()
	for _, id := range ids {
		if _, err := stmt.ExecContext(ctx, now, id); err != nil {
			tx.Rollback()
			telemetry.RecordRagSyncError(ctx)
			return err
		}
		telemetry.RecordRagRecordSynced(ctx)
	}

	return tx.Commit()
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	// Not implemented completely, but placeholder for the interface
	return nil
}
