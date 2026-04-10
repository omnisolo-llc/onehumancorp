package hub

import (
	"context"
	"time"
	"fmt"
	"strings"
	"strconv"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{provider: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var vectorStr *string
		err := rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSyncAt)
		if err != nil {
			return nil, err
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		if vectorStr != nil {
			r.Vector = parseVector(*vectorStr)
		}
		records = append(records, r)
	}

	return records, rows.Err()
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}

	defer func() {
		_ = tx.Rollback(ctx)
	}()

	query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1"

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			syncErrorsCounter.Add(ctx, 1)
			return err
		}
	}

	err = tx.Commit(ctx)
	if err == nil {
		recordsSyncedCounter.Add(ctx, int64(len(ids)))
	}
	return err
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}

	defer func() {
		_ = tx.Rollback(ctx)
	}()

	query := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			sync_status = 'synced',
			last_sync_at = CURRENT_TIMESTAMP
	`

	for _, r := range records {
		vectorStr := formatVector(r.Vector)
		_, err := tx.Exec(ctx, query, r.ID, r.Context, vectorStr)
		if err != nil {
			syncErrorsCounter.Add(ctx, 1)
			return err
		}
	}

	err = tx.Commit(ctx)
	if err == nil {
		recordsSyncedCounter.Add(ctx, int64(len(records)))
	}
	return err
}

// Helper to format float slice to pgvector format e.g. "[1.0,2.0]"
func formatVector(v []float32) string {
	if len(v) == 0 {
		return "[]"
	}
	res := "["
	for i, f := range v {
		if i > 0 {
			res += ","
		}
		res += fmt.Sprintf("%f", f)
	}
	res += "]"
	return res
}

func parseVector(s string) []float32 {
	s = strings.TrimPrefix(s, "[")
	s = strings.TrimSuffix(s, "]")
	if len(s) == 0 {
		return nil
	}
	parts := strings.Split(s, ",")
	var res []float32
	for _, p := range parts {
		f, err := strconv.ParseFloat(strings.TrimSpace(p), 32)
		if err == nil {
			res = append(res, float32(f))
		}
	}
	return res
}
