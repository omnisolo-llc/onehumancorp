package hub

import (
	"context"
	"database/sql"
	"fmt"
	"strings"
	"time"

	"go.opentelemetry.io/otel"
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

var (
	meter                    = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
	ragSyncErrorsTotal, _    = meter.Int64Counter("rag_sync_errors_total")
)

type SQLRAGSyncService struct {
	db *sql.DB
}

func NewSQLRAGSyncService(db *sql.DB) *SQLRAGSyncService {
	return &SQLRAGSyncService{db: db}
}

func (s *SQLRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *SQLRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args[i] = id
	}

	query := fmt.Sprintf(`UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id IN (%s)`, strings.Join(placeholders, ","))
	_, err = tx.ExecContext(ctx, query, args...)
	if err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}

	if err := tx.Commit(); err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}

	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return nil
}

func (s *SQLRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	for _, r := range records {
		// simplified upsert logic for both sqlite/postgres
		query := `
        INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
        VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
        ON CONFLICT(memory_id) DO UPDATE SET context = excluded.context, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
        `
		_, err := tx.ExecContext(ctx, query, r.ID, r.Context)
		if err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(); err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}

	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return nil
}
