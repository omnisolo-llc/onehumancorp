package hub

import (
	"context"
	"fmt"
	"strings"
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
	ID             string
	OrganizationID string
	AgentID        string
	Context        string
	Embedding      []float32 // Convert to string internally for SQLite compat if needed
	SourceType     string
	SyncStatus     SyncStatus
	LastSyncAt     time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
	provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{provider: provider}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, organization_id, agent_id, content, embedding, source_type, sync_status, last_sync_at FROM consolidated_memory WHERE sync_status = 'pending'"
	var args []any

	if s.provider.IsSQLite() {
		if limit > 0 {
			query += " LIMIT ?"
			args = append(args, limit)
		}
	} else {
		if limit > 0 {
			query += " LIMIT $1 FOR UPDATE SKIP LOCKED"
			args = append(args, limit)
		} else {
			query += " FOR UPDATE SKIP LOCKED"
		}
	}

	rows, err := s.provider.Query(ctx, query, args...)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var status string
		var orgID, agentID, sourceType *string
		var embeddingStr *string // representation
		err := rows.Scan(&r.ID, &orgID, &agentID, &r.Context, &embeddingStr, &sourceType, &status, &lastSyncAt)
		if err != nil {
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}
		if orgID != nil {
			r.OrganizationID = *orgID
		}
		if agentID != nil {
			r.AgentID = *agentID
		}
		if sourceType != nil {
			r.SourceType = *sourceType
		}
		if embeddingStr != nil && *embeddingStr != "" {
			// Extremely naive parsing for "[1.0, 2.0]" string
			cleanStr := strings.Trim(*embeddingStr, "[] ")
			if cleanStr != "" {
				parts := strings.Split(cleanStr, ",")
				for _, p := range parts {
					var val float32
					fmt.Sscanf(strings.TrimSpace(p), "%f", &val)
					r.Embedding = append(r.Embedding, val)
				}
			}
		}
		r.SyncStatus = SyncStatus(status)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// Batch update using a parameterized IN clause or ANY
	var query string
	var args []any

	if s.provider.IsSQLite() {
		placeholders := make([]string, len(ids))
		for i, id := range ids {
			placeholders[i] = "?"
			args = append(args, id)
		}
		query = fmt.Sprintf("UPDATE consolidated_memory SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id IN (%s)", strings.Join(placeholders, ","))
	} else {
		// Use unnest for arrays or build dynamically if pq.Array isn't used
		placeholders := make([]string, len(ids))
		for i, id := range ids {
			placeholders[i] = fmt.Sprintf("$%d", i+1)
			args = append(args, id)
		}
		query = fmt.Sprintf("UPDATE consolidated_memory SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id IN (%s)", strings.Join(placeholders, ","))
	}

	_, err = tx.Exec(ctx, query, args...)
	if err != nil {
		if telemetry.RAGSyncErrorsTotal != nil {
			telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("failed to mark batch synced: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if telemetry.RAGRecordsSyncedTotal != nil {
		telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var query string
		if s.provider.IsSQLite() {
			var vecArg any
			if len(r.Embedding) > 0 {
				vecStr := "["
				for i, v := range r.Embedding {
					if i > 0 {
						vecStr += ","
					}
					vecStr += fmt.Sprintf("%f", v)
				}
				vecStr += "]"
				vecArg = vecStr
			} else {
				vecArg = nil
			}
			query = `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, sync_status, last_sync_at)
					 VALUES (?, ?, ?, ?, ?, ?, 'synced', CURRENT_TIMESTAMP)
					 ON CONFLICT(id) DO UPDATE SET content=excluded.content, embedding=excluded.embedding, sync_status=excluded.sync_status, last_sync_at=CURRENT_TIMESTAMP`
			_, err = tx.Exec(ctx, query, r.ID, r.OrganizationID, r.AgentID, r.Context, vecArg, r.SourceType)
		} else {
			// pgvector uses string representation like '[1.0, 2.0, 3.0]' for INSERT
			var vecArg any
			if len(r.Embedding) > 0 {
				vecStr := "["
				for i, v := range r.Embedding {
					if i > 0 {
						vecStr += ","
					}
					vecStr += fmt.Sprintf("%f", v)
				}
				vecStr += "]"
				vecArg = vecStr
			} else {
				vecArg = nil
			}
			query = `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, sync_status, last_sync_at)
					 VALUES ($1, $2, $3, $4, $5, $6, 'synced', CURRENT_TIMESTAMP)
					 ON CONFLICT (id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding, sync_status = EXCLUDED.sync_status, last_sync_at = CURRENT_TIMESTAMP`
			_, err = tx.Exec(ctx, query, r.ID, r.OrganizationID, r.AgentID, r.Context, vecArg, r.SourceType)
		}

		if err != nil {
			if telemetry.RAGSyncErrorsTotal != nil {
				telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to upsert record %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if telemetry.RAGRecordsSyncedTotal != nil {
		telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}

	return nil
}
