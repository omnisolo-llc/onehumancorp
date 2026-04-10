package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"database/sql"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type ragSyncService struct {
	prov db.Provider
}

func NewRAGSyncService(prov db.Provider) RAGSyncService {
	return &ragSyncService{
		prov: prov,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending' OR sync_status = 'error'
		LIMIT ?
	`
	rows, err := s.prov.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime // Use custom NullTime or sql.NullTime wrapper for DB specific nullability
		var vectorStr sql.NullString

		err := rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSyncAt)
		if err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}

		// Parse JSON array of floats for sqlite compatibility
		if vectorStr.Valid && len(vectorStr.String) > 0 {
			var vec []float32
			if err := json.Unmarshal([]byte(vectorStr.String), &vec); err == nil {
				r.Vector = vec
			}
		}

		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		placeholders[i] = "?"
		args[i] = id
	}

	query := fmt.Sprintf(`
		UPDATE autodream_memories
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE id IN (%s)
	`, strings.Join(placeholders, ", "))

	_, err := s.prov.Exec(ctx, query, args...)
	if err != nil {
		telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to mark records synced: %w", err)
	}

	telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		query := `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE
			SET content = EXCLUDED.content,
			    embedding = EXCLUDED.embedding,
			    sync_status = 'synced',
			    last_sync_at = CURRENT_TIMESTAMP
		`

		// parse vector back to JSON string for DB
		var vectorStr string
		if len(r.Vector) > 0 {
			b, err := json.Marshal(r.Vector)
			if err == nil {
				vectorStr = string(b)
			} else {
				vectorStr = "[]"
			}
		} else {
			vectorStr = "[]"
		}

		_, err := s.prov.Exec(ctx, query, r.ID, r.Context, vectorStr)
		if err != nil {
			telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming record %s: %w", r.ID, err)
		}
		telemetry.RAGRecordsSyncedTotal.Add(ctx, 1)
	}

	return nil
}
