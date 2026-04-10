package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced successfully"),
	)
	if err != nil {
		panic(err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}

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

type hybridRAGSyncService struct {
	db db.Provider
}

func NewHybridRAGSyncService(db db.Provider) RAGSyncService {
	return &hybridRAGSyncService{
		db: db,
	}
}

func (s *hybridRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_timestamp
		FROM autodream_memories
		WHERE sync_status = $1
		ORDER BY created_at ASC
		LIMIT $2
	`

	rows, err := s.db.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorStr *string
		var statusStr string
		var lastSync *time.Time

		if err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &statusStr, &lastSync); err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}

		rec.SyncStatus = SyncStatus(statusStr)
		if lastSync != nil {
			rec.LastSyncAt = *lastSync
		}

		if vectorStr != nil && *vectorStr != "" {
			// Extract floats from standard vector representation [1.0, 2.0, ...]
			vStr := strings.Trim(*vectorStr, "[] ")
			if vStr != "" {
				var floats []float32
				strVals := strings.Split(vStr, ",")
				for _, sv := range strVals {
					var f float32
					fmt.Sscanf(strings.TrimSpace(sv), "%f", &f)
					floats = append(floats, f)
				}
				rec.Vector = floats
			}
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil && !errors.Is(err, sql.ErrNoRows) {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *hybridRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Update in a loop or batch. Since db.Provider doesn't have an IN clause array builder, we loop for simplicity or use transactions.
	tx, err := s.db.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE autodream_memories
		SET sync_status = $1, last_sync_timestamp = $2
		WHERE id = $3
	`

	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, query, string(SyncStatusSynced), now, id)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
		ragRecordsSyncedTotal.Add(ctx, 1)
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

func (s *hybridRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// In SQLite we can't use INSERT ... ON CONFLICT with the same syntax as Postgres for complex types sometimes,
	// but the application uses standard upsert patterns.
	// Since we are UPSERTing, we'll try to just UPDATE first, if 0 rows affected, INSERT.
	// We'll construct a simple approach compatible with db.Provider

	updateQuery := `
		UPDATE autodream_memories
		SET content = $1, embedding = CASE WHEN $2::text IS NULL THEN NULL ELSE $2::text::vector END, sync_status = $3, last_sync_timestamp = $4
		WHERE id = $5
	`
	if s.db.IsSQLite() {
		updateQuery = `
			UPDATE autodream_memories
			SET content = $1, embedding = $2, sync_status = $3, last_sync_timestamp = $4
			WHERE id = $5
		`
	}

	insertQuery := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_timestamp)
		VALUES ($1, $2, CASE WHEN $3::text IS NULL THEN NULL ELSE $3::text::vector END, $4, $5)
	`
	if s.db.IsSQLite() {
		insertQuery = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_timestamp)
			VALUES ($1, $2, $3, $4, $5)
		`
	}

	for _, rec := range records {
		var vectorParam interface{}
		if len(rec.Vector) > 0 {
			b, err := json.Marshal(rec.Vector)
			if err != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
				return fmt.Errorf("failed to marshal vector: %w", err)
			}
			vectorParam = string(b)
		}

		rowsAffectedInt, err := tx.Exec(ctx, updateQuery, rec.Context, vectorParam, string(rec.SyncStatus), rec.LastSyncAt, rec.ID)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to update record %s: %w", rec.ID, err)
		}

		if rowsAffectedInt == 0 {
			// Insert
			_, err = tx.Exec(ctx, insertQuery, rec.ID, rec.Context, vectorParam, string(rec.SyncStatus), rec.LastSyncAt)
			if err != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
				return fmt.Errorf("failed to insert record %s: %w", rec.ID, err)
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
