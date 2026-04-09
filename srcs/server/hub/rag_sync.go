package hub

import (
	"context"
	"time"
	"strings"
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
	meter                   = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal   metric.Int64Counter
	ragSyncErrorsTotal      metric.Int64Counter
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

// Concrete implementation
type DefaultRAGSyncService struct {
	database *db.DB
}

func NewDefaultRAGSyncService(database *db.DB) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{database: database}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = $1
		LIMIT $2
	`
	// sqlite uses ? for params in db.DB
	if s.database.Provider.IsSQLite() {
		query = strings.ReplaceAll(query, "$1", "?")
		query = strings.ReplaceAll(query, "$2", "?")
	}

	rows, err := s.database.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var vectorBytes []byte

		if err := rows.Scan(&r.ID, &r.Context, &vectorBytes, &r.SyncStatus, &lastSyncAt); err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return nil, err
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}

		if len(vectorBytes) > 0 {
			var vector []float32
			if err := json.Unmarshal(vectorBytes, &vector); err == nil {
				r.Vector = vector
			}
		}
		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}

	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.database.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = $1, last_sync_at = $2
		WHERE memory_id = $3
	`
	if s.database.Provider.IsSQLite() {
		query = strings.ReplaceAll(query, "$1", "?")
		query = strings.ReplaceAll(query, "$2", "?")
		query = strings.ReplaceAll(query, "$3", "?")
	}

	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, query, string(SyncStatusSynced), now, id)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.database.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`
	if s.database.Provider.IsSQLite() {
		query = strings.ReplaceAll(query, "$1", "?")
		query = strings.ReplaceAll(query, "$2", "?")
		query = strings.ReplaceAll(query, "$3", "?")
		query = strings.ReplaceAll(query, "$4", "?")
		query = strings.ReplaceAll(query, "$5", "?")
	}

	now := time.Now()
	for _, r := range records {
		vectorBytes, _ := json.Marshal(r.Vector)
		_, err := tx.Exec(ctx, query, r.ID, r.Context, vectorBytes, string(SyncStatusSynced), now)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
