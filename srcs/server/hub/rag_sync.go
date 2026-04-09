package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
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
	meter                 = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RAGRecordsSyncedTotal metric.Int64Counter
	RAGSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	RAGRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		panic(err)
	}

	RAGSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}

type defaultRAGSyncService struct {
	dbWrapper   *db.DB
	cloudAPIURL string
}

func NewRAGSyncService(dbWrapper *db.DB, cloudAPIURL string) RAGSyncService {
	if cloudAPIURL == "" {
		cloudAPIURL = os.Getenv("OHC_CORE_URL")
	}
	if cloudAPIURL == "" {
		cloudAPIURL = "http://localhost:8080"
	}
	return &defaultRAGSyncService{
		dbWrapper:   dbWrapper,
		cloudAPIURL: cloudAPIURL,
	}
}

func (s *defaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if !s.dbWrapper.IsSQLite() {
		return nil, nil // Only standalone SQLite should fetch pending local syncs
	}

	query := "SELECT memory_id, context, sync_status FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.dbWrapper.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("query swarm_memory_embeddings: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus); err != nil {
			return nil, fmt.Errorf("scan swarm_memory_embeddings: %w", err)
		}
		records = append(records, r)
	}
	return records, nil
}

func (s *defaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2"
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			RAGSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("update sync_status: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("commit tx: %w", err)
	}

	RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *defaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		// Convert Vector to byte array for Postgres BYTEA or JSON array
		var vectorBytes []byte
		if len(r.Vector) > 0 {
			vectorBytes, _ = json.Marshal(r.Vector)
		}

		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := tx.Exec(ctx, query, r.ID, r.Context, vectorBytes, time.Now())
		if err != nil {
			RAGSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("upsert swarm_memory_embeddings: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("commit tx: %w", err)
	}

	RAGRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
