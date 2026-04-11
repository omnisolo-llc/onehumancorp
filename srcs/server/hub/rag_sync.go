package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
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
	ID         string
	Content    string
	Vector     []float32 // Vector representation for pgvector compatibility, JSON stored in SQLite
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
	dbWrapper *db.DB
}

func NewRAGSyncService(dbWrapper *db.DB) RAGSyncService {
	return &ragSyncServiceImpl{
		dbWrapper: dbWrapper,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.dbWrapper.Query(ctx, query, limit)
	if err != nil {
		slog.ErrorContext(ctx, "failed to query pending syncs", "error", err)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var embeddingStr *string
		var lastSyncAt *time.Time
		if err := rows.Scan(&rec.ID, &rec.Content, &embeddingStr, &rec.SyncStatus, &lastSyncAt); err != nil {
			slog.ErrorContext(ctx, "failed to scan pending sync record", "error", err)
			continue
		}

		if embeddingStr != nil {
			// Try to parse the vector from string/JSON format
			var vec []float32
			if err := json.Unmarshal([]byte(*embeddingStr), &vec); err != nil {
				// Pgvector text format uses brackets like [1.2, 3.4]
				s := strings.Trim(*embeddingStr, "[] ")
				parts := strings.Split(s, ",")
				for _, p := range parts {
					var val float32
					fmt.Sscanf(strings.TrimSpace(p), "%f", &val)
					vec = append(vec, val)
				}
			}
			rec.Vector = vec
		}
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbWrapper.Provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1`
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to mark record as synced (id=%s): %w", id, err)
		}
		telemetry.RecordRagRecordSynced(ctx)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbWrapper.Provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var embeddingStr string
		if len(rec.Vector) > 0 {
			b, _ := json.Marshal(rec.Vector)
			embeddingStr = string(b)
		} else {
			embeddingStr = "null"
		}

		// Hybrid Upsert: Compatible with both SQLite and Postgres
		query := `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, CASE WHEN $3::text = 'null' OR $3::text IS NULL THEN NULL ELSE $3::text::vector END, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				content = excluded.content,
				embedding = excluded.embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`

		if s.dbWrapper.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`
			if embeddingStr == "null" {
				_, err = tx.Exec(ctx, query, rec.ID, rec.Content, nil)
			} else {
				_, err = tx.Exec(ctx, query, rec.ID, rec.Content, embeddingStr)
			}
		} else {
			_, err = tx.Exec(ctx, query, rec.ID, rec.Content, embeddingStr)
		}

		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to upsert incoming record (id=%s): %w", rec.ID, err)
		}
		telemetry.RecordRagRecordSynced(ctx)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}
	return nil
}
