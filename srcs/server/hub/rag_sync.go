package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

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
	Vector     []float32 // Convert to string internally for SQLite compat if needed
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
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	var err error

	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced to the cloud"),
	)
	if err != nil {
		panic(fmt.Sprintf("Failed to initialize rag_records_synced_total counter: %v", err))
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(fmt.Sprintf("Failed to initialize rag_sync_errors_total counter: %v", err))
	}
}

// DefaultRAGSyncService is a concrete implementation of RAGSyncService.
type DefaultRAGSyncService struct {
	dbProvider db.Provider
}

// NewDefaultRAGSyncService creates a new DefaultRAGSyncService.
func NewDefaultRAGSyncService(dbProvider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{
		dbProvider: dbProvider,
	}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// In SQLite, vector might be stored as text, but for Go compatibility we can try to scan it appropriately or just ignore it if it's not strictly necessary to return it for the sync logic (the requirements say convert to string internally for SQLite compat).
	// Let's use CAST(embedding AS TEXT) to support both SQLite and Postgres when fetching.
	query := `
		SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := tx.Query(ctx, query, limit)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("query autodream_memories: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorStr *string
		var lastSyncAt *time.Time
		var syncStatus *string
		if err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &syncStatus, &lastSyncAt); err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return nil, fmt.Errorf("scan autodream_memories row: %w", err)
		}
		if vectorStr != nil {
			var vec []float32
			if err := json.Unmarshal([]byte(*vectorStr), &vec); err == nil {
				rec.Vector = vec
			}
		}
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		if syncStatus != nil {
			rec.SyncStatus = SyncStatus(*syncStatus)
		} else {
			rec.SyncStatus = SyncStatusPending
		}
		records = append(records, rec)
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("commit transaction: %w", err)
	}

	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	syncedCount := 0

	// Loop over individual queries to support both Postgres and SQLite without JSON extensions
	for _, id := range ids {
		query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2`
		rowsAffected, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("update autodream_memories for id %s: %w", id, err)
		}
		syncedCount += int(rowsAffected)
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("commit transaction: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(syncedCount))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, rec := range records {
		var vectorParam interface{}
		if len(rec.Vector) > 0 {
			vectorBytes, _ := json.Marshal(rec.Vector)
			if s.dbProvider.IsSQLite() {
				vectorParam = string(vectorBytes)
			} else {
				vectorParam = string(vectorBytes) // For Postgres we can pass string representations of vectors as long as we cast or trust the pgx driver
			}
		}

		if s.dbProvider.IsSQLite() {
			query := `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, 'synced', $4)
				ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = 'synced',
					last_sync_at = excluded.last_sync_at
			`
			_, err = tx.Exec(ctx, query, rec.ID, rec.Context, vectorParam, now)
		} else {
			// In Postgres, vector fields can be parsed from strings (e.g. '[1,2,3]') if cast to ::vector
			query := `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3::vector, 'synced', $4)
				ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = 'synced',
					last_sync_at = excluded.last_sync_at
			`
			_, err = tx.Exec(ctx, query, rec.ID, rec.Context, vectorParam, now)
		}

		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("upsert autodream_memories for id %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("commit transaction: %w", err)
	}

	return nil
}
