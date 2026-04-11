package hub

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSynced, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	syncErrors, _ = meter.Int64Counter(
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
	ID         string
	Context    string
	Vector     *string // Pointers for nullability
	SyncStatus SyncStatus
	LastSyncAt *time.Time
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
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{
		provider: provider,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		ORDER BY created_at ASC
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vectorStr *string
		var statusStr *string

		err := rows.Scan(&r.ID, &r.Context, &vectorStr, &statusStr, &r.LastSyncAt)
		if err != nil {
			syncErrors.Add(ctx, 1)
			return nil, err
		}

		r.Vector = vectorStr
		if statusStr != nil {
			r.SyncStatus = SyncStatus(*statusStr)
		} else {
			r.SyncStatus = SyncStatusPending
		}

		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		syncErrors.Add(ctx, 1)
		return nil, err
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	if s.provider.IsSQLite() {
		// SQLite doesn't natively support = ANY($1) arrays well in database/sql driver natively without json extensions.
		// Since we want standard compat, we'll build a batch update query or use the loop. Given we want optimized,
		// we'll use a transaction and simple loop which SQLite executes very fast in a txn.
		for _, id := range ids {
			query := `
				UPDATE autodream_memories
				SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
				WHERE id = $1
			`
			_, err := tx.Exec(ctx, query, id)
			if err != nil {
				syncErrors.Add(ctx, 1)
				return err
			}
		}
	} else {
		// Postgres bulk update
		query := `
			UPDATE autodream_memories
			SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			WHERE id = ANY($1)
		`
		_, err := tx.Exec(ctx, query, ids)
		if err != nil {
			syncErrors.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrors.Add(ctx, 1)
		return err
	}

	recordsSynced.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var query string
		var err error

		if s.provider.IsSQLite() {
			// SQLite: ON CONFLICT DO UPDATE
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT (id) DO UPDATE SET
					content = EXCLUDED.content,
					embedding = EXCLUDED.embedding,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`
			_, err = tx.Exec(ctx, query, r.ID, r.Context, r.Vector)
		} else {
			// Postgres: require vector casting for insertion
			// Also checking for conflict on id
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3::vector, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT (id) DO UPDATE SET
					content = EXCLUDED.content,
					embedding = EXCLUDED.embedding,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`
			_, err = tx.Exec(ctx, query, r.ID, r.Context, r.Vector)
		}

		if err != nil {
			syncErrors.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrors.Add(ctx, 1)
		return err
	}

	recordsSynced.Add(ctx, int64(len(records)))
	return nil
}
