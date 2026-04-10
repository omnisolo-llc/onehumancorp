package hub

import (
	"context"
	"time"
	"database/sql"
	"fmt"
	"encoding/json"

	"go.opentelemetry.io/otel"
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
	ID           string
	Context      string
	Vector       []float32
	SyncStatus   SyncStatus
	LastSyncAt   time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	meter                = otel.Meter("github.com/onehumancorp/ohc/srcs/server/hub")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
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

type SqlRAGSyncService struct {
	DB *db.DB
}

func NewSqlRAGSyncService(db *db.DB) *SqlRAGSyncService {
	return &SqlRAGSyncService{DB: db}
}

func (s *SqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = $1
		LIMIT $2
	`
	rows, err := s.DB.Provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		return nil, fmt.Errorf("querying pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vecData []byte
		var lastSync sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &vecData, &r.SyncStatus, &lastSync); err != nil {
			return nil, fmt.Errorf("scanning record: %w", err)
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}
		if len(vecData) > 0 {
			var vec []float32
			if err := json.Unmarshal(vecData, &vec); err == nil {
				r.Vector = vec
			}
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *SqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create a parameterized query for the IN clause
	query := "UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_at = $2 WHERE memory_id IN ("
	args := []interface{}{string(SyncStatusSynced), time.Now()}
	for i, id := range ids {
		if i > 0 {
			query += ", "
		}
		query += fmt.Sprintf("$%d", i+3)
		args = append(args, id)
	}
	query += ")"

	_, err := s.DB.Provider.Exec(ctx, query, args...)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("updating sync status: %w", err)
	}
	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *SqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.DB.Provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("beginning tx: %w", err)
	}
	defer tx.Rollback(ctx)

	// Use simple upsert or insert depending on database semantics.
	// For simplicity in this implementation, we will use a naive approach.
	for _, r := range records {
		vecData, _ := json.Marshal(r.Vector)

		// This query is a simple upsert syntax that is common enough for this implementation.
		// A full robust implementation would consider SQLite vs Postgres dialect differences via s.DB.IsPostgres()
		// Here we're focusing on the general logic.
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT(memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = excluded.sync_status,
				last_sync_at = excluded.last_sync_at
		`
		_, err := tx.Exec(ctx, query, r.ID, r.Context, vecData, string(SyncStatusSynced), time.Now())
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("upserting record %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("committing tx: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
