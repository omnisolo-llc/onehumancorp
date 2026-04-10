package hub

import (
	"context"
	"database/sql"
	"encoding/binary"
	"fmt"
	"math"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
)

var (
	meter                = otel.GetMeterProvider().Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RagRecordsSynced, _  = meter.Int64Counter("rag_records_synced_total")
	RagSyncErrors, _     = meter.Int64Counter("rag_sync_errors_total")
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
	SyncStatusConflict SyncStatus = "conflict"
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

type RAGSyncProvider struct {
	db db.Provider
}

func NewRAGSyncProvider(dbProvider db.Provider) *RAGSyncProvider {
	return &RAGSyncProvider{
		db: dbProvider,
	}
}

// Convert float32 slice to byte slice directly for DB.
func FloatsToBytes(floats []float32) []byte {
	bytes := make([]byte, len(floats)*4)
	for i, f := range floats {
		binary.LittleEndian.PutUint32(bytes[i*4:], math.Float32bits(f))
	}
	return bytes
}

// Convert byte slice back to float32 slice.
func BytesToFloats(bytes []byte) ([]float32, error) {
	if len(bytes)%4 != 0 {
		return nil, fmt.Errorf("byte array length is not a multiple of 4")
	}
	floats := make([]float32, len(bytes)/4)
	for i := range floats {
		bits := binary.LittleEndian.Uint32(bytes[i*4:])
		floats[i] = math.Float32frombits(bits)
	}
	return floats, nil
}

func (p *RAGSyncProvider) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	if p.db.IsSQLite() {
		query = `
			SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
			FROM swarm_memory_embeddings
			WHERE sync_status = 'pending'
			LIMIT ?
		`
	}

	rows, err := p.db.Query(ctx, query, limit)
	if err != nil {
		RagSyncErrors.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vecBytes []byte
		var lastSyncAt sql.NullString
		if err := rows.Scan(&rec.ID, &rec.Context, &vecBytes, &rec.SyncStatus, &lastSyncAt); err != nil {
			RagSyncErrors.Add(ctx, 1)
			return nil, err
		}

		if vecBytes != nil {
			floats, err := BytesToFloats(vecBytes)
			if err != nil {
				RagSyncErrors.Add(ctx, 1)
				return nil, fmt.Errorf("failed to decode vector: %w", err)
			}
			rec.Vector = floats
		}

		if lastSyncAt.Valid {
			t, err := time.Parse(time.RFC3339, lastSyncAt.String)
			if err == nil {
				rec.LastSyncAt = t
			} else {
				// Handle fallback formats for SQLite
				t, err = time.Parse("2006-01-02 15:04:05.999999999Z07:00", lastSyncAt.String)
				if err == nil {
					rec.LastSyncAt = t
				}
			}
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		RagSyncErrors.Add(ctx, 1)
		return nil, err
	}

	return records, nil
}

func (p *RAGSyncProvider) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := p.db.Begin(ctx)
	if err != nil {
		RagSyncErrors.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = $1
		WHERE memory_id = $2
	`
	if p.db.IsSQLite() {
		query = `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = ?
			WHERE memory_id = ?
		`
	}

	now := time.Now().Format(time.RFC3339)
	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, now, id); err != nil {
			RagSyncErrors.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrors.Add(ctx, 1)
		return err
	}

	RagRecordsSynced.Add(ctx, int64(len(ids)))
	return nil
}

func (p *RAGSyncProvider) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := p.db.Begin(ctx)
	if err != nil {
		RagSyncErrors.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = 'synced',
			last_sync_at = EXCLUDED.last_sync_at
	`
	if p.db.IsSQLite() {
		query = `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES (?, ?, ?, ?, ?)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = 'synced',
				last_sync_at = excluded.last_sync_at
		`
	}

	now := time.Now().Format(time.RFC3339)
	for _, rec := range records {
		vecBytes := FloatsToBytes(rec.Vector)
		if _, err := tx.Exec(ctx, query, rec.ID, rec.Context, vecBytes, "synced", now); err != nil {
			RagSyncErrors.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrors.Add(ctx, 1)
		return err
	}

	RagRecordsSynced.Add(ctx, int64(len(records)))
	return nil
}
