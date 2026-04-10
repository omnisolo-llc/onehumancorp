package hub

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
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

type defaultRAGSyncService struct {
	db db.Provider
}

func NewRAGSyncService(database db.Provider) RAGSyncService {
	return &defaultRAGSyncService{db: database}
}

func (s *defaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	query := `SELECT id, content, embedding, sync_status, last_sync_at FROM agent_memories WHERE sync_status = 'pending' LIMIT $1`
	if s.db.IsSQLite() {
		query = `SELECT id, content, embedding, sync_status, last_sync_at FROM agent_memories WHERE sync_status = 'pending' LIMIT ?`
	}

	rows, err := tx.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var vector []byte
		err := rows.Scan(&r.ID, &r.Context, &vector, &r.SyncStatus, &lastSyncAt)
		if err != nil {
			return nil, err
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		r.Vector = vector
		records = append(records, r)
	}

	return records, nil
}

func (s *defaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		query := `UPDATE agent_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2`
		if s.db.IsSQLite() {
			query = `UPDATE agent_memories SET sync_status = 'synced', last_sync_at = ? WHERE id = ?`
		}
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			if RAGSyncErrorsTotal != nil {
				RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
		if RAGRecordsSyncedTotal != nil {
			RAGRecordsSyncedTotal.Add(ctx, 1)
		}
	}

	return tx.Commit(ctx)
}

func (s *defaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	claims := auth.ClaimsFromContext(ctx)
	orgID := "default"
	if claims != nil && claims.OrganizationID != "" {
		orgID = claims.OrganizationID
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		query := `INSERT INTO agent_memories (id, organization_id, content, embedding, sync_status, last_sync_at)
		          VALUES ($1, $2, $3, $4, 'synced', $5)
		          ON CONFLICT (id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding, sync_status = 'synced', last_sync_at = EXCLUDED.last_sync_at`
		if s.db.IsSQLite() {
			query = `INSERT INTO agent_memories (id, organization_id, content, embedding, sync_status, last_sync_at)
		             VALUES (?, ?, ?, ?, 'synced', ?)
		             ON CONFLICT (id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding, sync_status = 'synced', last_sync_at = EXCLUDED.last_sync_at`
		}
		_, err := tx.Exec(ctx, query, r.ID, orgID, r.Context, r.Vector, time.Now())
		if err != nil {
			if RAGSyncErrorsTotal != nil {
				RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	return tx.Commit(ctx)
}

var (
	meter                   = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RAGRecordsSyncedTotal   metric.Int64Counter
	RAGSyncErrorsTotal      metric.Int64Counter
)

func init() {
	var err error
	RAGRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced to the cloud"),
	)
	if err != nil {
		_ = err
	}

	RAGSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		_ = err
	}
}
