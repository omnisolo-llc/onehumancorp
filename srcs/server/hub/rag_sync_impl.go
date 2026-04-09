package hub

import (
	"context"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncServiceImpl struct {
	db db.Provider
}

// NewRAGSyncService creates a new RAGSyncService with the given DB provider.
func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{db: db}
}

func formatVector(v []float32) string {
	if len(v) == 0 {
		return ""
	}
	strs := make([]string, len(v))
	for i, val := range v {
		strs[i] = fmt.Sprintf("%f", val)
	}
	return "[" + strings.Join(strs, ",") + "]"
}

// Very basic float parser, ignoring errors for brevity but assuming format [1.0, 2.0]
func parseVector(s string) []float32 {
	if s == "" || s == "[]" {
		return nil
	}
	s = strings.TrimPrefix(s, "[")
	s = strings.TrimSuffix(s, "]")
	parts := strings.Split(s, ",")
	res := make([]float32, 0, len(parts))
	for _, p := range parts {
		var f float32
		fmt.Sscanf(strings.TrimSpace(p), "%f", &f)
		res = append(res, f)
	}
	return res
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, "SELECT id, organization_id, agent_id, content, embedding, source_type, sync_status, last_sync_at FROM consolidated_memory WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		RecordsSyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var vectorStr *string // Use pointer for NULL embeddings
		err := rows.Scan(&r.ID, &r.OrganizationID, &r.AgentID, &r.Context, &vectorStr, &r.SourceType, &r.SyncStatus, &lastSyncAt)
		if err != nil {
			RecordsSyncErrorsTotal.Add(ctx, 1)
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
	if err = rows.Err(); err != nil {
		RecordsSyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		RecordsSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err := tx.Exec(ctx, "UPDATE consolidated_memory SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1", id)
		if err != nil {
			RecordsSyncErrorsTotal.Add(ctx, int64(len(ids)))
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RecordsSyncErrorsTotal.Add(ctx, int64(len(ids)))
		return err
	}

	RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		RecordsSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var vectorStr *string
		if len(r.Vector) > 0 {
			v := formatVector(r.Vector)
			vectorStr = &v
		}

		_, err := tx.Exec(ctx, `
			INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at,
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding
		`, r.ID, r.OrganizationID, r.AgentID, r.Context, vectorStr, r.SourceType, r.SyncStatus)

		if err != nil {
			RecordsSyncErrorsTotal.Add(ctx, int64(len(records)))
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RecordsSyncErrorsTotal.Add(ctx, int64(len(records)))
		return err
	}

	RecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
