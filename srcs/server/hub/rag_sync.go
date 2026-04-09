package hub

import (
	"context"
	"database/sql"
	"time"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID           string
	Context      string
	Vector       []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus   SyncStatus
	LastSyncAt   time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ConcreteRAGSyncService struct {
	db *sql.DB
}

func NewConcreteRAGSyncService(db *sql.DB) *ConcreteRAGSyncService {
	return &ConcreteRAGSyncService{db: db}
}

func (s *ConcreteRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, "SELECT id, content, sync_status, last_sync_at FROM consolidated_memory WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		err := rows.Scan(&rec.ID, &rec.Context, &rec.SyncStatus, &lastSyncAt)
		if err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, rec)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return records, nil
}

func (s *ConcreteRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	stmt, err := tx.PrepareContext(ctx, "UPDATE consolidated_memory SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1")
	if err != nil {
		return err
	}
	defer stmt.Close()

	for _, id := range ids {
		_, err := stmt.ExecContext(ctx, id)
		if err != nil {
			return err
		}
	}
	return tx.Commit()
}

func (s *ConcreteRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	// Update existing or Insert
	for _, rec := range records {
		res, err := tx.ExecContext(ctx, "UPDATE consolidated_memory SET content = $1, sync_status = $2, last_sync_at = CURRENT_TIMESTAMP WHERE id = $3", rec.Context, rec.SyncStatus, rec.ID)
		if err != nil {
			return err
		}
		rowsAffected, err := res.RowsAffected()
		if err != nil {
			return err
		}
		if rowsAffected == 0 {
			// Insert
			_, err = tx.ExecContext(ctx, "INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status, last_sync_at) VALUES ($1, 'default', $2, 'sync', $3, CURRENT_TIMESTAMP)", rec.ID, rec.Context, rec.SyncStatus)
			if err != nil {
				return err
			}
		}
	}

	return tx.Commit()
}
