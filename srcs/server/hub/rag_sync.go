package hub

import (
	"context"
	"fmt"
	"time"
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
	Vector     []byte
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

type DBProvider interface {
	ExecContext(ctx context.Context, query string, args ...interface{}) error
	QueryContext(ctx context.Context, query string, args ...interface{}) (Rows, error)
	IsSQLite() bool
}

type Rows interface {
	Next() bool
	Scan(dest ...interface{}) error
	Close() error
}

type RAGSyncServiceImpl struct {
	db DBProvider
}

func NewRAGSyncService(db DBProvider) RAGSyncService {
	return &RAGSyncServiceImpl{db: db}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if s.db.IsSQLite() {
		// SQLite: fetch pending without FOR UPDATE SKIP LOCKED
		query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT ?"
		rows, err := s.db.QueryContext(ctx, query, limit)
		if err != nil {
			return nil, err
		}
		defer rows.Close()

		var records []RAGSyncRecord
		for rows.Next() {
			var rec RAGSyncRecord
			err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &rec.SyncStatus, &rec.LastSyncAt)
			if err != nil {
				return nil, err
			}
			records = append(records, rec)
		}
		return records, nil
	} else {
        return nil, fmt.Errorf("FetchPendingSyncs is only supported on standalone mode (SQLite)")
    }
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	if !s.db.IsSQLite() {
	    return fmt.Errorf("MarkSynced is only supported on standalone mode (SQLite)")
	}

	// Create query with variable number of parameters
	query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = ? WHERE memory_id IN ("
	args := []interface{}{time.Now()}

	for i, id := range ids {
		if i > 0 {
			query += ", "
		}
		query += "?"
		args = append(args, id)
	}
	query += ")"

	return s.db.ExecContext(ctx, query, args...)
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, rec := range records {
		if s.db.IsSQLite() {
			// UPSERT for SQLite
			query := "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES (?, ?, ?, ?, ?) ON CONFLICT(memory_id) DO UPDATE SET context = excluded.context, vector_embedding = excluded.vector_embedding, sync_status = excluded.sync_status, last_sync_at = excluded.last_sync_at"
			err := s.db.ExecContext(ctx, query, rec.ID, rec.Context, rec.Vector, rec.SyncStatus, rec.LastSyncAt)
			if err != nil {
				return err
			}
		} else {
			// UPSERT for Postgres
			query := "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT(memory_id) DO UPDATE SET context = excluded.context, vector_embedding = excluded.vector_embedding, sync_status = excluded.sync_status, last_sync_at = excluded.last_sync_at"
			err := s.db.ExecContext(ctx, query, rec.ID, rec.Context, rec.Vector, rec.SyncStatus, rec.LastSyncAt)
			if err != nil {
				return err
			}
		}
	}
	return nil
}
