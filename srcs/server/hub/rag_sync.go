package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"

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

var (
	meter              = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSyncedTotal metric.Int64Counter
	syncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	recordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
	if err != nil {
		panic(fmt.Errorf("failed to create recordsSyncedTotal metric: %w", err))
	}
	syncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
	if err != nil {
		panic(fmt.Errorf("failed to create syncErrorsTotal metric: %w", err))
	}
}

type sqlRAGSyncService struct {
	dbWrapper *db.DB
}

func NewSQLRAGSyncService(dbWrapper *db.DB) RAGSyncService {
	return &sqlRAGSyncService{dbWrapper: dbWrapper}
}

func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, embedding, sync_status, last_sync_at FROM agent_memories WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.dbWrapper.Query(ctx, query, limit)
	if err != nil {
		syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch")))
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		var embeddingJSON sql.NullString

		if err := rows.Scan(&rec.ID, &rec.Context, &embeddingJSON, &rec.SyncStatus, &lastSyncAt); err != nil {
			syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch_scan")))
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		if embeddingJSON.Valid && embeddingJSON.String != "" {
			var vec []float32
			if err := json.Unmarshal([]byte(embeddingJSON.String), &vec); err != nil {
				syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch_unmarshal")))
				return nil, fmt.Errorf("failed to unmarshal vector: %w", err)
			}
			rec.Vector = vec
		}
		records = append(records, rec)
	}
	if err := rows.Err(); err != nil {
		syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch_rows_err")))
		return nil, fmt.Errorf("row error: %w", err)
	}
	return records, nil
}

func (s *sqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "mark_synced_begin")))
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer func() {
		_ = tx.Rollback(ctx)
	}()

	query := `UPDATE agent_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2`
	now := time.Now().UTC()
	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, now, id); err != nil {
			syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "mark_synced_exec")))
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "mark_synced_commit")))
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	recordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming_begin")))
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer func() {
		_ = tx.Rollback(ctx)
	}()

	query := `INSERT INTO agent_memories (id, organization_id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5, $6)
		ON CONFLICT (id) DO UPDATE SET
		content = excluded.content,
		embedding = excluded.embedding,
		sync_status = excluded.sync_status,
		last_sync_at = excluded.last_sync_at`


	for _, rec := range records {
		var embeddingJSON *string
		if rec.Vector != nil {
			b, err := json.Marshal(rec.Vector)
			if err != nil {
				syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming_marshal")))
				return fmt.Errorf("failed to marshal vector: %w", err)
			}
			jsonStr := string(b)
			embeddingJSON = &jsonStr
		}

		var lastSyncAt sql.NullTime
		if !rec.LastSyncAt.IsZero() {
			lastSyncAt.Time = rec.LastSyncAt
			lastSyncAt.Valid = true
		}

		// Assume incoming records have 'synced' status
		status := SyncStatusSynced
		if rec.SyncStatus != "" {
			status = rec.SyncStatus
		}
		orgID := "default"

		if _, err := tx.Exec(ctx, query, rec.ID, orgID, rec.Context, embeddingJSON, string(status), lastSyncAt); err != nil {
			syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming_exec")))
			return fmt.Errorf("failed to upsert record id %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming_commit")))
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
