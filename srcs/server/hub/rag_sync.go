package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Content    string
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type sqliteRAGSyncService struct {
	db *db.DB
}

func NewSqliteRAGSyncService(dbWrapper *db.DB) RAGSyncService {
	return &sqliteRAGSyncService{
		db: dbWrapper,
	}
}

func (s *sqliteRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_timestamp
		FROM autodream_memories
		WHERE sync_status = 'pending' OR sync_status IS NULL
		LIMIT $1
	`
	rows, err := s.db.Provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vecStr string
		var lastSync *time.Time
		if err := rows.Scan(&r.ID, &r.Content, &vecStr, &r.SyncStatus, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if vecStr != "" {
			if err := json.Unmarshal([]byte(vecStr), &r.Vector); err != nil {
				return nil, fmt.Errorf("failed to unmarshal vector: %w", err)
			}
		}
		if lastSync != nil {
			r.LastSyncAt = *lastSync
		}

		if r.SyncStatus == "" {
			r.SyncStatus = SyncStatusPending
		}

		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *sqliteRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create query with correct number of placeholders
	placeholders := ""
	args := []interface{}{SyncStatusSynced, time.Now()}
	for i, id := range ids {
		if i > 0 {
			placeholders += ", "
		}
		placeholders += fmt.Sprintf("$%d", i+3)
		args = append(args, id)
	}

	query := fmt.Sprintf(`
		UPDATE autodream_memories
		SET sync_status = $1, last_sync_timestamp = $2
		WHERE id IN (%s)
	`, placeholders)

	_, err := s.db.Provider.Exec(ctx, query, args...)
	if err != nil {
		return fmt.Errorf("failed to update sync status: %w", err)
	}

	return nil
}

func (s *sqliteRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		vecBytes, err := json.Marshal(r.Vector)
		if err != nil {
			return fmt.Errorf("failed to marshal vector: %w", err)
		}

		query := `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_timestamp)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (id) DO UPDATE SET
				content = excluded.content,
				embedding = excluded.embedding,
				sync_status = excluded.sync_status,
				last_sync_timestamp = excluded.last_sync_timestamp
		`

		_, err = s.db.Provider.Exec(ctx, query, r.ID, r.Content, string(vecBytes), SyncStatusSynced, time.Now())
		if err != nil {
			return fmt.Errorf("failed to upsert record: %w", err)
		}
	}
	return nil
}
