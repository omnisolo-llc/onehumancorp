package hub

import (
	"context"
	"time"
	"encoding/json"
	"fmt"
	"log/slog"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		provider: provider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, embedding, sync_status, last_sync_at FROM consolidated_memory WHERE sync_status = 'pending' LIMIT $1"
	if s.provider.IsSQLite() {
		query = "SELECT id, content, embedding, sync_status, last_sync_at FROM consolidated_memory WHERE sync_status = 'pending' LIMIT ?"
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		if RAGSyncErrorsTotal != nil {
			RAGSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, fmt.Errorf("query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var embeddingData *string
		var lastSyncAt *time.Time // pointer because it can be null

		if err := rows.Scan(&r.ID, &r.Context, &embeddingData, &r.SyncStatus, &lastSyncAt); err != nil {
			if RAGSyncErrorsTotal != nil {
				RAGSyncErrorsTotal.Add(ctx, 1)
			}
			slog.Error("failed to scan sync record", "error", err)
			continue
		}

		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}

		if embeddingData != nil && *embeddingData != "" {
			vector, err := parseVector(*embeddingData)
			if err == nil {
				r.Vector = vector
			} else {
				slog.Error("failed to parse vector embedding", "error", err)
			}
		}

		records = append(records, r)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	query := "UPDATE consolidated_memory SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1"
	if s.provider.IsSQLite() {
		query = "UPDATE consolidated_memory SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = ?"
	}

	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, id); err != nil {
			if RAGSyncErrorsTotal != nil {
				RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("update sync status for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if RAGSyncErrorsTotal != nil {
			RAGSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("commit tx: %w", err)
	}

	if RAGRecordsSyncedTotal != nil {
		RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	// LWW conflict resolution via ON CONFLICT
	insertQuery := `
		INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type, sync_status, last_sync_at)
		VALUES ($1, 'default', $2, $3, 'sync', 'synced', CURRENT_TIMESTAMP)
		ON CONFLICT(id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			sync_status = 'synced',
			last_sync_at = CURRENT_TIMESTAMP
	`

	if s.provider.IsSQLite() {
		insertQuery = `
			INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type, sync_status, last_sync_at)
			VALUES (?, 'default', ?, ?, 'sync', 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT(id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
	}

	for _, r := range records {
		var embeddingStr *string
		if len(r.Vector) > 0 {
			str := serializeVector(r.Vector)
			embeddingStr = &str
		}

		if _, err := tx.Exec(ctx, insertQuery, r.ID, r.Context, embeddingStr); err != nil {
			if RAGSyncErrorsTotal != nil {
				RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("upsert record id %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if RAGSyncErrorsTotal != nil {
			RAGSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("commit tx: %w", err)
	}

	return nil
}

// parseVector converts a JSON array string "[1.1, 2.2]" to []float32
func parseVector(data string) ([]float32, error) {
	data = strings.TrimSpace(data)
	if !strings.HasPrefix(data, "[") || !strings.HasSuffix(data, "]") {
		return nil, fmt.Errorf("invalid vector format")
	}

	var floats []float32
	if err := json.Unmarshal([]byte(data), &floats); err != nil {
		return nil, err
	}
	return floats, nil
}

// serializeVector converts []float32 to a JSON array string "[1.1, 2.2]"
func serializeVector(vector []float32) string {
	b, _ := json.Marshal(vector)
	return string(b)
}
