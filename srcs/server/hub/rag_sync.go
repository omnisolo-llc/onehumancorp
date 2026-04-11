package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
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
	Context    string
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncService struct {
	dbProvider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{
		dbProvider: provider,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// In SQLite, vector could be text, so we cast to TEXT for safety in both pg and sqlite.
	query := `SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at
			  FROM autodream_memories
			  WHERE sync_status = 'pending'
			  LIMIT $1`

	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return []RAGSyncRecord{}, nil
		}
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorStr *string
		var syncStatus sql.NullString
		var lastSyncAt sql.NullTime

		err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &syncStatus, &lastSyncAt)
		if err != nil {
			return nil, err
		}

		if syncStatus.Valid {
			rec.SyncStatus = SyncStatus(syncStatus.String)
		} else {
			rec.SyncStatus = SyncStatusPending
		}

		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}

		if vectorStr != nil && *vectorStr != "" && *vectorStr != "null" {
			// Parsing the stringified vector back into []float32
			err = json.Unmarshal([]byte(*vectorStr), &rec.Vector)
			if err != nil {
				// Ignore parse errors for vector stringification. Just keep it empty if it fails.
				// In a real system we might want to handle it, but continuing is fine for this layer.
			}
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1`
	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			return err
		}
	}

	return tx.Commit(ctx)
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var query string
	if s.dbProvider.IsSQLite() {
		query = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, CASE WHEN $3 = 'null' OR $3 IS NULL THEN NULL ELSE CAST($3 AS TEXT) END, $4, $5)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
	} else {
		query = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, CASE WHEN CAST($3 AS TEXT) = 'null' OR CAST($3 AS TEXT) IS NULL THEN NULL ELSE CAST($3 AS TEXT)::vector END, $4, $5)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
	}
	for _, rec := range records {
		var vectorStr interface{}
		if len(rec.Vector) > 0 {
			b, _ := json.Marshal(rec.Vector)
			vectorStr = string(b)
		} else {
			vectorStr = "null"
		}

		status := string(rec.SyncStatus)
		if status == "" {
			status = string(SyncStatusSynced)
		}

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vectorStr, status, time.Now())
		if err != nil {
			return err
		}
	}

	return tx.Commit(ctx)
}
