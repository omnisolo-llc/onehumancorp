package hub

import (
	"context"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSyncedTotal, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced to cloud"),
	)
	syncErrorsTotal, _ = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Content    string
	Vector     []float32 // Vector embedding
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

type ragSyncService struct {
	db db.Provider
}

// NewRAGSyncService creates a new RAGSyncService
func NewRAGSyncService(database db.Provider) RAGSyncService {
	return &ragSyncService{
		db: database,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// In order to fetch vectors cleanly across both Postgres and SQLite without binary format issues,
	// we select the embedding as TEXT, then parse it back to []float32.
	// But first, let's just see if we can read the raw string
	query := `SELECT id, content, CAST(embedding AS TEXT), sync_status FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		syncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var id, content, syncStatus string
		var embeddingText *string
		if err := rows.Scan(&id, &content, &embeddingText, &syncStatus); err != nil {
			syncErrorsTotal.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}

		var vector []float32
		if embeddingText != nil && *embeddingText != "" {
			// Parse text like "[0.1, 0.2, ...]" into []float32
			cleanStr := strings.Trim(*embeddingText, "[]")
			parts := strings.Split(cleanStr, ",")
			for _, part := range parts {
				part = strings.TrimSpace(part)
				if part == "" {
					continue
				}
				var v float32
				fmt.Sscanf(part, "%f", &v)
				vector = append(vector, v)
			}
		}

		records = append(records, RAGSyncRecord{
			ID:         id,
			Content:    content,
			Vector:     vector,
			SyncStatus: SyncStatus(syncStatus),
		})
	}
	if err := rows.Err(); err != nil {
		syncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Because `db.Provider` and typical SQL don't easily do WHERE id IN (?,?,?) with arbitrary lengths in a generic way,
	// and to ensure cross-db compat, we execute a loop or use a transaction.
	tx, err := s.db.Begin(ctx)
	if err != nil {
		syncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx) // Safe to call even if committed

	query := `UPDATE autodream_memories SET sync_status = $1, last_sync_at = CURRENT_TIMESTAMP WHERE id = $2`
	for _, id := range ids {
		_, err := tx.Exec(ctx, query, string(SyncStatusSynced), id)
		if err != nil {
			syncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	recordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		syncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	// ON CONFLICT DO UPDATE is compatible with SQLite (3.24.0+) and Postgres (9.5+)
	// Convert vector to string representation to insert correctly across both providers if needed,
	// or use standard representation.
	query := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = CURRENT_TIMESTAMP
	`

	for _, rec := range records {
		var vecStr *string
		if len(rec.Vector) > 0 {
			strs := make([]string, len(rec.Vector))
			for i, v := range rec.Vector {
				strs[i] = fmt.Sprintf("%f", v)
			}
			s := "[" + strings.Join(strs, ",") + "]"
			vecStr = &s
		}

		_, err := tx.Exec(ctx, query, rec.ID, rec.Content, vecStr, string(rec.SyncStatus))
		if err != nil {
			syncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	return nil
}
