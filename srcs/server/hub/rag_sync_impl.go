package hub

import (
	"database/sql"
	"context"
	"fmt"
	"strings"
	"strconv"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/metric"
)

// RAGSyncServiceImpl provides a concrete implementation of RAGSyncService
type RAGSyncServiceImpl struct {
	provider db.Provider
}

// NewRAGSyncService creates a new RAGSyncService
func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &RAGSyncServiceImpl{provider: provider}
}

// float32SliceToString converts a float32 slice to a string format like "[0.1, 0.2, ...]"
func float32SliceToString(v []float32) string {
	strs := make([]string, len(v))
	for i, f := range v {
		strs[i] = fmt.Sprintf("%f", f)
	}
	return "[" + strings.Join(strs, ",") + "]"
}


// parseVectorString parses a string like "[0.1, 0.2]" into a []float32
func parseVectorString(s string) ([]float32, error) {
	if s == "" || s == "[]" {
		return nil, nil
	}
	s = strings.TrimPrefix(s, "[")
	s = strings.TrimSuffix(s, "]")
	parts := strings.Split(s, ",")
	vec := make([]float32, 0, len(parts))
	for _, p := range parts {
		p = strings.TrimSpace(p)
		if p == "" {
			continue
		}
		f, err := strconv.ParseFloat(p, 32)
		if err != nil {
			return nil, err
		}
		vec = append(vec, float32(f))
	}
	return vec, nil
}

// FetchPendingSyncs retrieves records from the local DB that need syncing
func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var embeddingStr sql.NullString
		var lastSyncAt sql.NullTime
		err := rows.Scan(&rec.ID, &rec.Context, &embeddingStr, &rec.SyncStatus, &lastSyncAt)
		if err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		if embeddingStr.Valid {
			vec, err := parseVectorString(embeddingStr.String)
			if err != nil {
				return nil, err
			}
			rec.Vector = vec
		}
		records = append(records, rec)
	}

	if err = rows.Err(); err != nil {
		return nil, err
	}

	return records, nil
}

// MarkSynced updates the local DB after a successful sync to the cloud
func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Construct parameterized IN clause for security and compatibility
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args[i] = id
	}

	query := fmt.Sprintf("UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id IN (%s)", strings.Join(placeholders, ","))

	_, err := s.provider.Exec(ctx, query, args...)
	if err != nil {
		telemetry.RagSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes())
		return err
	}

	telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)), metric.WithAttributes())
	return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, rec := range records {
		var embeddingVal interface{}
		if len(rec.Vector) > 0 {
			embeddingVal = float32SliceToString(rec.Vector)
		}

		query := `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3::vector, $4, CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		if s.provider.IsSQLite() {
			query = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		}

		_, err := s.provider.Exec(ctx, query, rec.ID, rec.Context, embeddingVal, SyncStatusSynced)
		if err != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes())
			return err
		}

		telemetry.RagRecordsSyncedTotal.Add(ctx, 1, metric.WithAttributes())
	}
	return nil
}
