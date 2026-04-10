package hub

import (
	"context"
	"database/sql"
	"fmt"
	"strings"
	"time"

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

var (
	RecordsSyncedTotal metric.Int64Counter
	SyncErrorsTotal    metric.Int64Counter
)

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/ohc")
	var err error
	RecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total")
	if err != nil {
		fmt.Printf("failed to initialize metrics: %v\n", err)
	}
	SyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total")
	if err != nil {
		fmt.Printf("failed to initialize metrics: %v\n", err)
	}
}

type dbRAGSyncService struct {
	db *sql.DB
}

func NewDBRAGSyncService(db *sql.DB) RAGSyncService {
	return &dbRAGSyncService{db: db}
}

func (s *dbRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit <= 0 {
		return nil, fmt.Errorf("limit must be greater than 0")
	}

	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		LIMIT $2
	`
	rows, err := s.db.QueryContext(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return records, nil
}

func (s *dbRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create placeholders for the IN clause
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids)+2)
	args[0] = string(SyncStatusSynced)
	args[1] = time.Now()

	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+3)
		args[i+2] = id
	}

	query := fmt.Sprintf(`
		UPDATE autodream_memories
		SET sync_status = $1, last_sync_at = $2
		WHERE id IN (%s)
	`, strings.Join(placeholders, ","))

	_, err := s.db.ExecContext(ctx, query, args...)
	if err == nil && RecordsSyncedTotal != nil {
		RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	} else if err != nil && SyncErrorsTotal != nil {
		SyncErrorsTotal.Add(ctx, 1)
	}
	return err
}

func (s *dbRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	// For a real implementation, we would use a transaction and insert/update.
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	// Assuming we upsert into consolidated_memory in the cloud DB
	query := `
		INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`

	stmt, err := tx.PrepareContext(ctx, query)
	if err != nil {
		return err
	}
	defer stmt.Close()

	for _, r := range records {
		_, err := stmt.ExecContext(ctx, r.ID, "default-org", r.Context, r.Vector, "sync", string(SyncStatusSynced), time.Now())
		if err != nil {
			if SyncErrorsTotal != nil {
				SyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	err = tx.Commit()
	if err == nil && RecordsSyncedTotal != nil {
		RecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return err
}
