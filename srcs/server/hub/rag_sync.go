package hub

import (
	"context"
	"time"
	"fmt"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSyncedCounter, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced to the cloud"),
	)
	syncErrorsCounter, _ = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
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
	Vector       *string // Changed from []float32 to string for cross-db compatibility in syncing
	SyncStatus   SyncStatus
	LastSyncAt   time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DBService struct {
	provider db.Provider
}

func NewDBService(provider db.Provider) *DBService {
	return &DBService{provider: provider}
}

func (s *DBService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// Cast embedding to text for cross-compatibility between PG and SQLite
	rows, err := s.provider.Query(ctx, "SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at FROM autodream_memories WHERE sync_status = $1 LIMIT $2", SyncStatusPending, limit)
	if err != nil {
		syncErrorsCounter.Add(ctx, 1)
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt); err != nil {
			syncErrorsCounter.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan pending syncs: %w", err)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		syncErrorsCounter.Add(ctx, 1)
		return nil, fmt.Errorf("rows error during pending syncs fetch: %w", err)
	}
	return records, nil
}

func (s *DBService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create placeholder array for IN clause
	placeholders := make([]string, len(ids))
	args := make([]any, len(ids)+2)
	args[0] = SyncStatusSynced
	args[1] = time.Now()

	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+3)
		args[i+2] = id
	}

	query := fmt.Sprintf("UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id IN (%s)", strings.Join(placeholders, ","))

	_, err := s.provider.Exec(ctx, query, args...)
	if err != nil {
		syncErrorsCounter.Add(ctx, 1)
		return fmt.Errorf("failed to mark records as synced: %w", err)
	}
	recordsSyncedCounter.Add(ctx, int64(len(ids)))

	return nil
}

func (s *DBService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	// Build dialect-specific INSERT query
	var insertQuery string
	if s.provider.IsSQLite() {
		insertQuery = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, CAST($3 AS TEXT), $4, $5)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
	} else {
		insertQuery = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, CAST($3 AS VECTOR), $4, $5)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
	}

	for _, record := range records {
		_, err := tx.Exec(ctx, insertQuery, record.ID, record.Context, record.Vector, SyncStatusSynced, time.Now())
		if err != nil {
			syncErrorsCounter.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming sync (id=%s): %w", record.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	return nil
}
