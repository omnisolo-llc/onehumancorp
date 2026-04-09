package hub

import (
	"context"
	"database/sql"
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

type ProviderRAGSyncService struct {
	db db.Provider
}

func NewProviderRAGSyncService(provider db.Provider) *ProviderRAGSyncService {
	return &ProviderRAGSyncService{db: provider}
}

func (s *ProviderRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"

	// SQLite syntax might not fully match Postgres without care, but we assume basic compatibility
	// based on the interface expectations. Let's use standard placeholders if DB abstracts it,
	// or we may need conditional logic. Here we just assume standard query handling via provider.

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		RecordSyncError(ctx)
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorBytes []byte
		var lastSync sql.NullTime
		var status sql.NullString

		err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &status, &lastSync)
		if err != nil {
			RecordSyncError(ctx)
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}

		// In SQLite, vector might be stored as a string or blob of JSON array
		if len(vectorBytes) > 0 {
			if vectorBytes[0] == '[' {
				// It's a json array string
				_ = json.Unmarshal(vectorBytes, &rec.Vector)
			}
			// If it's a binary pgvector type, we'd need more complex handling, but instruction says:
			// "Convert to string internally for SQLite compat if needed"
		}

		if status.Valid {
			rec.SyncStatus = SyncStatus(status.String)
		} else {
			rec.SyncStatus = SyncStatusPending
		}

		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}

		records = append(records, rec)
	}

	return records, rows.Err()
}

func (s *ProviderRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create a transaction since we are updating multiple
	tx, err := s.db.Begin(ctx)
	if err != nil {
		RecordSyncError(ctx)
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1", id)
		if err != nil {
			RecordSyncError(ctx)
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		RecordSyncError(ctx)
		return err
	}

	RecordSyncSuccess(ctx, int64(len(ids)))
	return nil
}

func (s *ProviderRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		RecordSyncError(ctx)
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var vectorVal interface{}

		if s.db.IsSQLite() {
			// Convert to string for SQLite
			if rec.Vector != nil {
				vBytes, _ := json.Marshal(rec.Vector)
				vectorVal = string(vBytes)
			}
		} else {
			// for pgvector string "[1,2,3]" is also acceptable
			if rec.Vector != nil {
				vBytes, _ := json.Marshal(rec.Vector)
				vectorVal = string(vBytes)
			}
		}

		// Basic upsert query, syntax might differ between sqlite/postgres,
		// but standard ON CONFLICT(memory_id) works in both recent versions.
		upsertQuery := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT(memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`

		_, err := tx.Exec(ctx, upsertQuery, rec.ID, rec.Context, vectorVal)
		if err != nil {
			RecordSyncError(ctx)
			return fmt.Errorf("failed to upsert record %s: %w", rec.ID, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		RecordSyncError(ctx)
		return err
	}

	RecordSyncSuccess(ctx, int64(len(records)))
	return nil
}

var (
	meter                = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSyncedCounter metric.Int64Counter
	syncErrorsCounter    metric.Int64Counter
)

func init() {
	var err error
	recordsSyncedCounter, err = meter.Int64Counter("rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		panic(err)
	}

	syncErrorsCounter, err = meter.Int64Counter("rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}

func RecordSyncSuccess(ctx context.Context, count int64) {
	recordsSyncedCounter.Add(ctx, count)
}

func RecordSyncError(ctx context.Context) {
	syncErrorsCounter.Add(ctx, 1)
}
