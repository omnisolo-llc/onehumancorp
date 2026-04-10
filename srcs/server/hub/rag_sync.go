package hub

import (
	"context"
	"database/sql"
	"encoding/binary"
	"fmt"
	"math"

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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt sql.NullString
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{provider: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT "
	if s.provider.IsSQLite() {
		query += "?"
	} else {
		query += "$1"
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord

		if s.provider.IsSQLite() {
			var vecBytes []byte
			if err := rows.Scan(&rec.ID, &rec.Context, &vecBytes); err != nil {
				return nil, fmt.Errorf("failed to scan record: %w", err)
			}
			rec.Vector, err = bytesToFloat32Array(vecBytes)
			if err != nil {
				return nil, fmt.Errorf("failed to parse vector: %w", err)
			}
		} else {
			// In Postgres, vector type comes back as string e.g. "[0.1, 0.2]"
			// We need a proper way to handle pgvector format here.
			// For this implementation, we will use strings and custom parsing.
			var vecString string
			if err := rows.Scan(&rec.ID, &rec.Context, &vecString); err != nil {
				return nil, fmt.Errorf("failed to scan record: %w", err)
			}
			// Parse pgvector string representation "[0.1, 0.2]" back to []float32
			rec.Vector, err = parsePgVectorString(vecString)
			if err != nil {
			    return nil, fmt.Errorf("failed to parse pgvector string: %w", err)
			}
		}

		rec.SyncStatus = SyncStatusPending
		records = append(records, rec)
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = "
	if s.provider.IsSQLite() {
		query += "?"
	} else {
		query += "$1"
	}
	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, id); err != nil {
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}
	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}
	RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	if s.provider.IsSQLite() {
		query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                 VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
                 ON CONFLICT (memory_id) DO UPDATE SET context = excluded.context, vector_embedding = excluded.vector_embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`
	} else {
		query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                 VALUES ($1, $2, $3::vector, 'synced', CURRENT_TIMESTAMP)
                 ON CONFLICT (memory_id) DO UPDATE SET context = excluded.context, vector_embedding = excluded.vector_embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`
	}

	for _, rec := range records {
		if s.provider.IsSQLite() {
			vecBytes := float32ArrayToBytes(rec.Vector)
			if _, err := tx.Exec(ctx, query, rec.ID, rec.Context, vecBytes); err != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
				return fmt.Errorf("failed to upsert record %s: %w", rec.ID, err)
			}
		} else {
			vecString := formatPgVectorString(rec.Vector)
			if _, err := tx.Exec(ctx, query, rec.ID, rec.Context, vecString); err != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
				return fmt.Errorf("failed to upsert record %s: %w", rec.ID, err)
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}
	return nil
}

func bytesToFloat32Array(b []byte) ([]float32, error) {
	if len(b) == 0 {
		return nil, nil
	}
	if len(b)%4 != 0 {
		return nil, fmt.Errorf("byte slice length must be a multiple of 4, got %d", len(b))
	}
	arr := make([]float32, len(b)/4)
	for i := 0; i < len(arr); i++ {
		bits := binary.LittleEndian.Uint32(b[i*4 : (i+1)*4])
		arr[i] = math.Float32frombits(bits)
	}
	return arr, nil
}

func float32ArrayToBytes(arr []float32) []byte {
	if len(arr) == 0 {
		return nil
	}
	b := make([]byte, len(arr)*4)
	for i, f := range arr {
		bits := math.Float32bits(f)
		binary.LittleEndian.PutUint32(b[i*4:(i+1)*4], bits)
	}
	return b
}

func parsePgVectorString(s string) ([]float32, error) {
	if len(s) < 2 || s[0] != '[' || s[len(s)-1] != ']' {
		return nil, fmt.Errorf("invalid pgvector string format: %s", s)
	}
	s = s[1 : len(s)-1]
	if len(s) == 0 {
		return nil, nil
	}

	var arr []float32
	var currentVal float32
	var start int
	for i, c := range s {
		if c == ',' {
			if _, err := fmt.Sscanf(s[start:i], "%f", &currentVal); err != nil {
				return nil, err
			}
			arr = append(arr, currentVal)
			start = i + 1
		}
	}
	if start < len(s) {
		if _, err := fmt.Sscanf(s[start:], "%f", &currentVal); err != nil {
			return nil, err
		}
		arr = append(arr, currentVal)
	}

	return arr, nil
}

func formatPgVectorString(arr []float32) string {
	if len(arr) == 0 {
		return "[]"
	}
	s := "["
	for i, v := range arr {
		if i > 0 {
			s += ","
		}
		s += fmt.Sprintf("%f", v)
	}
	s += "]"
	return s
}

var (
	meter                 = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RagRecordsSyncedTotal, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced"),
	)
	RagSyncErrorsTotal, _ = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
)
