package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                   = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedCounter metric.Int64Counter
	ragSyncErrorsCounter    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedCounter, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced successfully"),
	)
	if err != nil {
		panic(err)
	}

	ragSyncErrorsCounter, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		panic(err)
	}
}

// RecordSyncSuccess increments the rag_records_synced_total counter
func RecordSyncSuccess(ctx context.Context, count int64) {
	ragRecordsSyncedCounter.Add(ctx, count)
}

// RecordSyncError increments the rag_sync_errors_total counter
func RecordSyncError(ctx context.Context, count int64) {
	ragSyncErrorsCounter.Add(ctx, count)
}

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type MemoryType string

const (
	MemoryTypeAutoDream MemoryType = "autodream"
	MemoryTypeAgent     MemoryType = "agent"
)

type RAGSyncRecord struct {
	ID         string
	Type       MemoryType
	Context    string
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, records []RAGSyncRecord) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
	db *sql.DB
}

func NewDefaultRAGSyncService(db *sql.DB) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: db}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var records []RAGSyncRecord

	// Fetch from autodream_memories
	autoDreamRecords, err := s.fetchPendingFromTable(ctx, "autodream_memories", MemoryTypeAutoDream, limit)
	if err != nil {
		return nil, err
	}
	records = append(records, autoDreamRecords...)

	// Fetch from agent_memories
	remainingLimit := limit - len(records)
	if remainingLimit > 0 {
		agentRecords, err := s.fetchPendingFromTable(ctx, "agent_memories", MemoryTypeAgent, remainingLimit)
		if err != nil {
			return nil, err
		}
		records = append(records, agentRecords...)
	}

	return records, nil
}

func (s *DefaultRAGSyncService) fetchPendingFromTable(ctx context.Context, table string, memType MemoryType, limit int) ([]RAGSyncRecord, error) {
	// Not vulnerable to SQL injection because table name is controlled by constants
	query := fmt.Sprintf(`
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM %s
		WHERE sync_status = 'pending'
		LIMIT $1
	`, table)

	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		RecordSyncError(ctx, 1)
		return nil, fmt.Errorf("failed to fetch pending syncs from %s: %w", table, err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		rec.Type = memType
		var embeddingStr sql.NullString
		var lastSync sql.NullTime

		if err := rows.Scan(&rec.ID, &rec.Context, &embeddingStr, &rec.SyncStatus, &lastSync); err != nil {
			RecordSyncError(ctx, 1)
			return nil, fmt.Errorf("failed to scan record from %s: %w", table, err)
		}

		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}

		if embeddingStr.Valid && embeddingStr.String != "" {
			var vec []float32
			if err := json.Unmarshal([]byte(embeddingStr.String), &vec); err == nil {
				rec.Vector = vec
			}
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		RecordSyncError(ctx, 1)
		return nil, fmt.Errorf("row error from %s: %w", table, err)
	}

	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		RecordSyncError(ctx, 1)
		return err
	}
	defer tx.Rollback()

	now := time.Now()
	for _, rec := range records {
		var table string
		if rec.Type == MemoryTypeAutoDream {
			table = "autodream_memories"
		} else if rec.Type == MemoryTypeAgent {
			table = "agent_memories"
		} else {
			continue // skip unknown
		}

		query := fmt.Sprintf("UPDATE %s SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2", table)
		if _, err := tx.ExecContext(ctx, query, now, rec.ID); err != nil {
			RecordSyncError(ctx, 1)
			return fmt.Errorf("failed to mark record %s as synced in %s: %w", rec.ID, table, err)
		}
	}

	if err := tx.Commit(); err != nil {
		RecordSyncError(ctx, 1)
		return err
	}

	RecordSyncSuccess(ctx, int64(len(records)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		RecordSyncError(ctx, 1)
		return err
	}
	defer tx.Rollback()

	now := time.Now()
	for _, rec := range records {
		var table string
		if rec.Type == MemoryTypeAutoDream {
			table = "autodream_memories"
		} else if rec.Type == MemoryTypeAgent {
			table = "agent_memories"
		} else {
			continue
		}

		var embeddingVal interface{}
		if len(rec.Vector) > 0 {
			b, _ := json.Marshal(rec.Vector)
			embeddingVal = string(b)
		} else {
			embeddingVal = nil
		}

		// Use the specific JSON/vector string format that pgvector and go-pg expect depending on DB.
		// For SQLite tests, passing a string is enough.
		// Let's use standard direct insert without CAST, because most standard Go drivers
		// handles driver.Value directly without needing explicit casting for inserting vector string literal if the column is vector.
		query := fmt.Sprintf(`
			INSERT INTO %s (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = 'synced',
				last_sync_at = EXCLUDED.last_sync_at
		`, table)

		if _, err := tx.ExecContext(ctx, query, rec.ID, rec.Context, embeddingVal, now); err != nil {
			// Fallback with CAST for Postgres in case of typed parameters issues
			fallbackQuery := fmt.Sprintf(`
				INSERT INTO %s (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, CAST($3 AS vector), 'synced', $4)
				ON CONFLICT (id) DO UPDATE SET
					content = EXCLUDED.content,
					embedding = EXCLUDED.embedding,
					sync_status = 'synced',
					last_sync_at = EXCLUDED.last_sync_at
			`, table)
			if _, err2 := tx.ExecContext(ctx, fallbackQuery, rec.ID, rec.Context, embeddingVal, now); err2 != nil {
				RecordSyncError(ctx, 1)
				return fmt.Errorf("failed to upsert record %s: primary err %v, fallback err %v", rec.ID, err, err2)
			}
		}
	}

	if err := tx.Commit(); err != nil {
		RecordSyncError(ctx, 1)
		return err
	}

	RecordSyncSuccess(ctx, int64(len(records)))
	return nil
}

// Ensure the implementation conforms to the interface
var _ RAGSyncService = (*DefaultRAGSyncService)(nil)
