package sync

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
	Context        string
	Vector         []float32
	SyncStatus     SyncStatus
	LastSyncAt     time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncService struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{
		provider: provider,
	}
}

func parseVectorString(s string) []float32 {
	if s == "" || s == "[]" {
		return nil
	}
	s = strings.TrimPrefix(s, "[")
	s = strings.TrimSuffix(s, "]")
	parts := strings.Split(s, ",")
	var result []float32
	for _, p := range parts {
		var f float32
		fmt.Sscanf(p, "%f", &f)
		result = append(result, f)
	}
	return result
}

func formatVectorString(v []float32) string {
	if len(v) == 0 {
		return "[]"
	}
	var sb strings.Builder
	sb.WriteString("[")
	for i, f := range v {
		if i > 0 {
			sb.WriteString(",")
		}
		sb.WriteString(fmt.Sprintf("%f", f))
	}
	sb.WriteString("]")
	return sb.String()
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, organization_id, content, embedding, sync_status, last_sync_at FROM agent_memories WHERE sync_status = $1 LIMIT $2"
	if s.provider.IsSQLite() {
		query = "SELECT id, organization_id, content, embedding, sync_status, last_sync_at FROM agent_memories WHERE sync_status = ? LIMIT ?"
	}

	rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus string
		var embeddingStr *string
		var orgID string

		if s.provider.IsSQLite() {
			if err := rows.Scan(&r.ID, &orgID, &r.Context, &embeddingStr, &syncStatus, &lastSyncAt); err != nil {
				telemetry.RecordRAGSyncError(ctx)
				return nil, fmt.Errorf("failed to scan sync record: %w", err)
			}
			if embeddingStr != nil {
				r.Vector = parseVectorString(*embeddingStr)
			}
		} else {
			if err := rows.Scan(&r.ID, &orgID, &r.Context, &embeddingStr, &syncStatus, &lastSyncAt); err != nil {
				telemetry.RecordRAGSyncError(ctx)
				return nil, fmt.Errorf("failed to scan sync record: %w", err)
			}
			if embeddingStr != nil {
				r.Vector = parseVectorString(*embeddingStr)
			}
		}

		r.OrganizationID = orgID
		r.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	query := "UPDATE agent_memories SET sync_status = $1, last_sync_at = CURRENT_TIMESTAMP WHERE id = $2"
	if s.provider.IsSQLite() {
		query = "UPDATE agent_memories SET sync_status = ?, last_sync_at = CURRENT_TIMESTAMP WHERE id = ?"
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err = tx.Exec(ctx, query, string(SyncStatusSynced), id)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return fmt.Errorf("failed to mark synced for %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit mark synced: %w", err)
	}

	telemetry.RecordRAGRecordsSynced(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	query := `INSERT INTO agent_memories (id, organization_id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)
		ON CONFLICT (id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding, sync_status = EXCLUDED.sync_status, last_sync_at = CURRENT_TIMESTAMP`

	if s.provider.IsSQLite() {
		query = `INSERT INTO agent_memories (id, organization_id, content, embedding, sync_status, last_sync_at)
			VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding, sync_status = EXCLUDED.sync_status, last_sync_at = CURRENT_TIMESTAMP`
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		orgID := r.OrganizationID
		if orgID == "" {
			orgID = "default" // Fallback if none provided
		}

		var emb interface{}
		if len(r.Vector) > 0 {
			emb = formatVectorString(r.Vector)
		}

		_, err := tx.Exec(ctx, query, r.ID, orgID, r.Context, emb, string(SyncStatusSynced))
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return fmt.Errorf("failed to process incoming sync for %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit process incoming sync: %w", err)
	}

	telemetry.RecordRAGRecordsSynced(ctx, int64(len(records)))
	return nil
}
