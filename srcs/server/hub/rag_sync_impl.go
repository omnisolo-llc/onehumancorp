package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type RAGSyncServiceImpl struct {
	database *db.DB
}

func NewRAGSyncService(database *db.DB) RAGSyncService {
	return &RAGSyncServiceImpl{database: database}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.database.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var embeddingStr sql.NullString
		var lastSync sql.NullTime
		var syncStatus string

		if err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &syncStatus, &lastSync); err != nil {
			slog.Error("failed to scan autodream_memories row", "error", err)
			continue
		}

		r.SyncStatus = SyncStatus(syncStatus)
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}

		if embeddingStr.Valid && embeddingStr.String != "" {
			var vec []float32
			if err := json.Unmarshal([]byte(embeddingStr.String), &vec); err != nil {
				// Just ignore vector unmarshal failures to not block sync
				slog.Warn("failed to unmarshal embedding", "error", err, "id", r.ID)
			} else {
				r.Vector = vec
			}
		}

		records = append(records, r)
	}

	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		_, err := s.database.Exec(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2", time.Now(), id)
		if err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			slog.Error("failed to mark synced", "id", id, "error", err)
			continue
		}
		if ragRecordsSyncedTotal != nil {
			ragRecordsSyncedTotal.Add(ctx, 1)
		}
	}
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		vecBytes, err := json.Marshal(r.Vector)
		var vecStr *string
		if err == nil && len(r.Vector) > 0 {
			str := string(vecBytes)
			vecStr = &str
		}

		var count int
		err = s.database.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE id = $1", r.ID).Scan(&count)
		if err != nil {
			slog.Error("failed to check existence", "id", r.ID, "error", err)
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			continue
		}

		if count > 0 {
			_, err = s.database.Exec(ctx, "UPDATE autodream_memories SET content = $1, embedding = $2, sync_status = 'synced', last_sync_at = $3 WHERE id = $4",
				r.Context, vecStr, time.Now(), r.ID)
		} else {
			_, err = s.database.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', $4)",
				r.ID, r.Context, vecStr, time.Now())
		}

		if err != nil {
			slog.Error("failed to process incoming sync", "id", r.ID, "error", err)
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
		} else {
			if ragRecordsSyncedTotal != nil {
				ragRecordsSyncedTotal.Add(ctx, 1)
			}
		}
	}
	return nil
}
