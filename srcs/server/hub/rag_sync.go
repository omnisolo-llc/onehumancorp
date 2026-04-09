package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"
	"time"

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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type hybridRAGSyncService struct {
	db *sql.DB
}

func NewHybridRAGSyncService(db *sql.DB) RAGSyncService {
	return &hybridRAGSyncService{db: db}
}

func (s *hybridRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorBytes []byte
		var lastSyncAt sql.NullTime
		var syncStatus sql.NullString
		if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &syncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if vectorBytes != nil {
			_ = json.Unmarshal(vectorBytes, &rec.Vector)
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		if syncStatus.Valid {
			rec.SyncStatus = SyncStatus(syncStatus.String)
		} else {
			rec.SyncStatus = SyncStatusPending
		}
		records = append(records, rec)
	}
	return records, nil
}

func (s *hybridRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Prepare IN clause safely
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args[i] = id
	}

	query := fmt.Sprintf(`
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id IN (%s)
	`, strings.Join(placeholders, ","))

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	res, err := tx.ExecContext(ctx, query, args...)
	if err != nil {
		return err
	}

	if err := tx.Commit(); err != nil {
		return err
	}

	rowsAffected, _ := res.RowsAffected()
	ragRecordsSyncedTotal.Add(ctx, rowsAffected)
	return nil
}

func (s *hybridRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
		ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = 'synced',
			last_sync_at = CURRENT_TIMESTAMP
	`
	stmt, err := tx.PrepareContext(ctx, query)
	if err != nil {
		return err
	}
	defer stmt.Close()

	var synced int64
	for _, rec := range records {
		var vectorBytes []byte
		if rec.Vector != nil {
			vectorBytes, _ = json.Marshal(rec.Vector)
		}

		// In OHC, to ensure hybrid compatibility and avoid driver issues with base64,
		// we must explicitly cast byte slice to string when writing JSON to the DB.
		var vectorParam interface{}
		if vectorBytes != nil {
			vectorParam = string(vectorBytes)
		} else {
			vectorParam = nil
		}

		if _, err := stmt.ExecContext(ctx, rec.ID, rec.Context, vectorParam); err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return err
		}
		synced++
	}

	if err := tx.Commit(); err != nil {
		return err
	}

	ragRecordsSyncedTotal.Add(ctx, synced)
	return nil
}

var (
	meter                    = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	ragSyncErrorsTotal, _    = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
)
