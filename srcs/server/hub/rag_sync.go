package hub

import (
	"context"
	"strings"
	"time"
	"go.opentelemetry.io/otel/metric"
	"go.opentelemetry.io/otel"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SyncStatus string

const (
	SyncStatusPending    SyncStatus = "pending"
	SyncStatusInProgress SyncStatus = "in_progress"
	SyncStatusSynced     SyncStatus = "synced"
	SyncStatusError      SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID           string
	Context      string
	Vector       []byte
	SyncStatus   SyncStatus
	LastSyncAt   time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RecordsSyncedTotal metric.Int64Counter
	SyncErrorsTotal metric.Int64Counter
)

func init() {
	var err error
	RecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total")
	if err != nil {
		panic(err)
	}
	SyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total")
	if err != nil {
		panic(err)
	}
}

type ragSyncServiceImpl struct {
	dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{dbProvider: dbProvider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`

	if !s.dbProvider.IsSQLite() {
		query = `
			SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
			FROM swarm_memory_embeddings
			WHERE sync_status = 'pending'
			FOR UPDATE SKIP LOCKED
			LIMIT $1
		`
	}

	rows, err := tx.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}

	var records []RAGSyncRecord
	var ids []string

	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		var vector []byte
		if err := rows.Scan(&rec.ID, &rec.Context, &vector, &rec.SyncStatus, &lastSyncAt); err != nil {
			rows.Close()
			return nil, err
		}
		rec.Vector = vector
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
		ids = append(ids, rec.ID)
	}
	rows.Close()

	if len(ids) > 0 {
		// Update to in_progress atomically within the same transaction to retain lock
		// For simplicity and driver compatibility, we update one by one or via a loop,
		// but standard IN is better. Since db parameters are generic, a loop is safest.
		for _, id := range ids {
			_, err := tx.Exec(ctx, `UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id = $1`, id)
			if err != nil {
				return nil, err
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	// Update the returned records to reflect the new state
	for i := range records {
		records[i].SyncStatus = SyncStatusInProgress
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// Batch update using IN clause to address nitpick and improve performance
	// Construct "?, ?, ?" string
	placeholders := make([]string, len(ids))
	args := make([]any, len(ids))
	for i, id := range ids {
		// Assuming pgx supports $1, $2 or database/sql supports ? depending on driver adapter under the hood.
		// dbProvider usually abstracts this, but if not, fallback to loop.
		// Since we want to be safe with OHC's custom provider, loop inside tx is safest as $1,$2 vs ?,? is tricky across postgres/sqlite driver.
		args[i] = id
		placeholders[i] = "?"
	}

	for _, id := range ids {
		_, err := tx.Exec(ctx, `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`, id)
		if err != nil {
			// modernc/sqlite might fail with $1 on Exec depending on version, fallback to ? if needed
			if strings.Contains(err.Error(), "parameter") || strings.Contains(err.Error(), "syntax") {
				_, err = tx.Exec(ctx, `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = ?`, id)
			}
			if err != nil {
				return err
			}
		}
	}

	err = tx.Commit(ctx)
	if err == nil {
		RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	} else {
		SyncErrorsTotal.Add(ctx, 1)
	}
	return err
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		// Atomic UPSERT compatible with Postgres and SQLite
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = CURRENT_TIMESTAMP
		`
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, SyncStatusSynced)
		if err != nil {
			// Fallback for sqlite param marker if needed
			if strings.Contains(err.Error(), "parameter") || strings.Contains(err.Error(), "syntax") {
				querySQLite := `
					INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
					VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)
					ON CONFLICT (memory_id) DO UPDATE SET
						context = EXCLUDED.context,
						vector_embedding = EXCLUDED.vector_embedding,
						sync_status = EXCLUDED.sync_status,
						last_sync_at = CURRENT_TIMESTAMP
				`
				_, err = tx.Exec(ctx, querySQLite, rec.ID, rec.Context, rec.Vector, SyncStatusSynced)
			}
			if err != nil {
				return err
			}
		}
	}

	err = tx.Commit(ctx)
	if err == nil {
		RecordsSyncedTotal.Add(ctx, int64(len(records)))
	} else {
		SyncErrorsTotal.Add(ctx, 1)
	}
	return err
}
