package hub

import (
	"context"
	"database/sql"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/metric"
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
	Vector     []byte
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

var (
	RagRecordsSyncedTotal metric.Int64Counter
	RagSyncErrorsTotal    metric.Int64Counter
)

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
	dbWrapper *db.DB
}

func NewRAGSyncService(dbWrapper *db.DB) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{dbWrapper: dbWrapper}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	provider := s.dbWrapper.Provider
	query := `SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	if provider.IsSQLite() {
		query = `SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?`
	}

	rows, err := provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		var embeddingBytes []byte
		var syncStatus sql.NullString
		if err := rows.Scan(&rec.ID, &rec.Content, &embeddingBytes, &syncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		rec.Vector = embeddingBytes
		if syncStatus.Valid {
			rec.SyncStatus = SyncStatus(syncStatus.String)
		} else {
			rec.SyncStatus = SyncStatusPending
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, rec)
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	provider := s.dbWrapper.Provider
	tx, err := provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = NOW() WHERE id = $1`
		if provider.IsSQLite() {
			query = `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = datetime('now') WHERE id = ?`
		}
		if _, err := tx.Exec(ctx, query, id); err != nil {
			if RagSyncErrorsTotal != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
		if RagRecordsSyncedTotal != nil {
			RagRecordsSyncedTotal.Add(ctx, 1)
		}
	}
	return tx.Commit(ctx)
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	provider := s.dbWrapper.Provider
	tx, err := provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var count int
		checkQuery := `SELECT COUNT(*) FROM autodream_memories WHERE id = $1`
		if provider.IsSQLite() {
			checkQuery = `SELECT COUNT(*) FROM autodream_memories WHERE id = ?`
		}
		if err := tx.QueryRow(ctx, checkQuery, rec.ID).Scan(&count); err != nil {
			if RagSyncErrorsTotal != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}

		if count == 0 {
			insertQuery := `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', NOW())`
			if provider.IsSQLite() {
				insertQuery = `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES (?, ?, ?, 'synced', datetime('now'))`
			}
			if _, err := tx.Exec(ctx, insertQuery, rec.ID, rec.Content, rec.Vector); err != nil {
				if RagSyncErrorsTotal != nil {
					RagSyncErrorsTotal.Add(ctx, 1)
				}
				return err
			}
		} else {
			updateQuery := `UPDATE autodream_memories SET content = $1, embedding = $2, sync_status = 'synced', last_sync_at = NOW() WHERE id = $3`
			if provider.IsSQLite() {
				updateQuery = `UPDATE autodream_memories SET content = ?, embedding = ?, sync_status = 'synced', last_sync_at = datetime('now') WHERE id = ?`
			}
			if _, err := tx.Exec(ctx, updateQuery, rec.Content, rec.Vector, rec.ID); err != nil {
				if RagSyncErrorsTotal != nil {
					RagSyncErrorsTotal.Add(ctx, 1)
				}
				return err
			}
		}
	}
	return tx.Commit(ctx)
}
