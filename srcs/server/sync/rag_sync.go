package sync

import (
	"context"
	"encoding/json"
	"strings"
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
	RagRecordsSyncedTotal metric.Int64Counter
	RagSyncErrorsTotal    metric.Int64Counter
)

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/sync")
	var err error

	RagRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced to the cloud"),
	)
	if err != nil {
		panic(err)
	}

	RagSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors encountered"),
	)
	if err != nil {
		panic(err)
	}
}



type ragSyncServiceImpl struct {
	dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		dbProvider: dbProvider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, embedding FROM agent_memories WHERE sync_status = $1 LIMIT $2`
	rows, err := s.dbProvider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var embeddingStr *string
		if err := rows.Scan(&rec.ID, &rec.Context, &embeddingStr); err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return nil, err
		}
		if embeddingStr != nil && *embeddingStr != "" {
			if err := json.Unmarshal([]byte(*embeddingStr), &rec.Vector); err != nil {
				// Handle specific unmarshal errors or convert format
				// In some DBs like pgvector it might be a format "[1.1,2.2]" that unmarshals cleanly
				// We do our best here
				var v []float32
				json.Unmarshal([]byte(strings.ReplaceAll(strings.ReplaceAll(*embeddingStr, "{", "["), "}", "]")), &v)
				rec.Vector = v
			}
		}
		rec.SyncStatus = SyncStatusPending
		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Hybrid architecture note: SQLite via modernc doesn't support $1 = ANY natively with arrays.
	// Therefore, we execute them in a transaction using individual updates for broad compatibility.
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `UPDATE agent_memories SET sync_status = $1, last_sync_at = CURRENT_TIMESTAMP WHERE id = $2`
		if _, err := tx.Exec(ctx, query, string(SyncStatusSynced), id); err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var embedding interface{}
		if len(rec.Vector) > 0 {
			b, err := json.Marshal(rec.Vector)
			if err == nil {
				embedding = string(b)
			}
		}

		if s.dbProvider.IsSQLite() {
			query := `
				INSERT INTO agent_memories (id, organization_id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, 'default', $2, CAST($3 AS TEXT), $4, CURRENT_TIMESTAMP)
				ON CONFLICT(id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
			if _, err := tx.Exec(ctx, query, rec.ID, rec.Context, embedding, string(SyncStatusSynced)); err != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
				return err
			}
		} else {
			query := `
				INSERT INTO agent_memories (id, organization_id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, 'default', $2, $3::vector, $4, CURRENT_TIMESTAMP)
				ON CONFLICT(id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
			if _, err := tx.Exec(ctx, query, rec.ID, rec.Context, embedding, string(SyncStatusSynced)); err != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
				return err
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
