package hub

import (
	"context"
	"database/sql"
	"encoding/binary"
	"fmt"
	"math"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
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

var (
	RAGRecordsSyncedTotal metric.Int64Counter
	RAGSyncErrorsTotal    metric.Int64Counter
)

func InitRAGSyncMetrics(meter metric.Meter) error {
	var err error
	RAGRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced successfully"),
	)
	if err != nil {
		return fmt.Errorf("failed to initialize RAGRecordsSyncedTotal: %w", err)
	}

	RAGSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		return fmt.Errorf("failed to initialize RAGSyncErrorsTotal: %w", err)
	}
	return nil
}

type DatabaseRAGSyncService struct {
	provider db.Provider
}

func NewDatabaseRAGSyncService(provider db.Provider) *DatabaseRAGSyncService {
	return &DatabaseRAGSyncService{
		provider: provider,
	}
}

func (s *DatabaseRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_timestamp
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending' LIMIT `
    if s.provider.IsSQLite() {
        query += "?"
    } else {
        query += "$1"
    }

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		if RAGSyncErrorsTotal != nil {
			RAGSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var (
			id             string
			contextData    string
			vectorBytes    []byte
			syncStatus     sql.NullString
			lastSyncString sql.NullString
		)
		err := rows.Scan(&id, &contextData, &vectorBytes, &syncStatus, &lastSyncString)
		if err != nil {
			if RAGSyncErrorsTotal != nil {
				RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return nil, fmt.Errorf("failed to scan pending sync row: %w", err)
		}

		var vector []float32
		if len(vectorBytes) > 0 {
			vector = make([]float32, len(vectorBytes)/4)
			for i := 0; i < len(vector); i++ {
				bits := binary.LittleEndian.Uint32(vectorBytes[i*4 : (i+1)*4])
				vector[i] = math.Float32frombits(bits)
			}
		}

		status := SyncStatusPending
		if syncStatus.Valid && syncStatus.String != "" {
			status = SyncStatus(syncStatus.String)
		}

		var lastSyncAt time.Time
		if lastSyncString.Valid && lastSyncString.String != "" {
			// Try to parse both possible sqlite formats (RFC3339 and standard SQL)
			parsed, err := time.Parse(time.RFC3339, lastSyncString.String)
			if err != nil {
				parsed, err = time.Parse("2006-01-02 15:04:05.999999-07:00", lastSyncString.String)
				if err != nil {
					parsed, _ = time.Parse("2006-01-02 15:04:05", lastSyncString.String)
				}
			}
			lastSyncAt = parsed
		}

		records = append(records, RAGSyncRecord{
			ID:         id,
			Context:    contextData,
			Vector:     vector,
			SyncStatus: status,
			LastSyncAt: lastSyncAt,
		})
	}
	if err := rows.Err(); err != nil {
		if RAGSyncErrorsTotal != nil {
			RAGSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, fmt.Errorf("rows error during fetch: %w", err)
	}

	return records, nil
}

func (s *DatabaseRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		if s.provider.IsSQLite() {
			placeholders[i] = "?"
		} else {
			placeholders[i] = fmt.Sprintf("$%d", i+1)
		}
		args[i] = id
	}

	query := fmt.Sprintf(
		"UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP WHERE memory_id IN (%s)",
		strings.Join(placeholders, ","),
	)

	_, err := s.provider.Exec(ctx, query, args...)
	if err != nil {
		if RAGSyncErrorsTotal != nil {
			RAGSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("failed to mark records as synced: %w", err)
	}

	if RAGRecordsSyncedTotal != nil {
		RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *DatabaseRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, rec := range records {
		var vectorBytes []byte
		if len(rec.Vector) > 0 {
			vectorBytes = make([]byte, len(rec.Vector)*4)
			for i, v := range rec.Vector {
				bits := math.Float32bits(v)
				binary.LittleEndian.PutUint32(vectorBytes[i*4:(i+1)*4], bits)
			}
		}

		query := ""
		args := []interface{}{}

		if s.provider.IsSQLite() {
			query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_timestamp)
				VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT(memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = 'synced',
				last_sync_timestamp = CURRENT_TIMESTAMP`
			args = []interface{}{rec.ID, rec.Context, vectorBytes}
		} else {
			query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_timestamp)
				VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT(memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_timestamp = CURRENT_TIMESTAMP`
			args = []interface{}{rec.ID, rec.Context, vectorBytes}
		}

		_, err := s.provider.Exec(ctx, query, args...)
		if err != nil {
			if RAGSyncErrorsTotal != nil {
				RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to upsert incoming record %s: %w", rec.ID, err)
		}
		if RAGRecordsSyncedTotal != nil {
			RAGRecordsSyncedTotal.Add(ctx, 1)
		}
	}

	return nil
}
