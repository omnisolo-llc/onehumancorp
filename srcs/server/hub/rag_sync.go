package hub

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/binary"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

	ragRecordsSyncedTotal, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Number of RAG records successfully synced"),
	)
	ragSyncErrorsTotal, _ = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Number of errors encountered during RAG sync"),
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

// ensure ragSyncProvider implements RAGSyncService
var _ RAGSyncService = (*ragSyncProvider)(nil)

type ragSyncProvider struct {
	provider db.Provider
}

// NewRAGSyncProvider creates a new concrete implementation of RAGSyncService.
func NewRAGSyncProvider(provider db.Provider) RAGSyncService {
	return &ragSyncProvider{
		provider: provider,
	}
}

func encodeVectorToBytes(vector []float32) ([]byte, error) {
	buf := new(bytes.Buffer)
	err := binary.Write(buf, binary.LittleEndian, vector)
	if err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}

func encodeVectorToString(vector []float32) (string, error) {
	b, err := json.Marshal(vector)
	if err != nil {
		return "", err
	}
	return string(b), nil
}

func decodeVectorFromBytes(b []byte) ([]float32, error) {
	var v []float32
	buf := bytes.NewReader(b)
	// Compute number of floats
	numFloats := len(b) / 4
	v = make([]float32, numFloats)
	err := binary.Read(buf, binary.LittleEndian, &v)
	if err != nil {
		return nil, err
	}
	return v, nil
}

func decodeVectorFromString(s string) ([]float32, error) {
	var v []float32
	err := json.Unmarshal([]byte(s), &v)
	return v, err
}

func (p *ragSyncProvider) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = $1
		LIMIT $2
	`
	rows, err := p.provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch")))
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vecRaw interface{}
		var lastSync sql.NullTime

		err := rows.Scan(&rec.ID, &rec.Context, &vecRaw, &rec.SyncStatus, &lastSync)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch_scan")))
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}

		if vecRaw != nil {
			switch v := vecRaw.(type) {
			case []byte:
				rec.Vector, err = decodeVectorFromBytes(v)
				if err != nil {
					// Fallback to string decode if it's stored as JSON string in []byte
					rec.Vector, _ = decodeVectorFromString(string(v))
				}
			case string:
				rec.Vector, _ = decodeVectorFromString(v)
			}
		}

		records = append(records, rec)
	}

	if err = rows.Err(); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch_rows")))
		return nil, err
	}

	return records, nil
}

func (p *ragSyncProvider) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Prepare IN clause safely
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids)+2)
	args[0] = string(SyncStatusSynced)
	args[1] = time.Now().UTC()
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+3)
		args[i+2] = id
	}

	query := fmt.Sprintf(`
		UPDATE swarm_memory_embeddings
		SET sync_status = $1, last_sync_at = $2
		WHERE memory_id IN (%s)
	`, strings.Join(placeholders, ","))

	_, err := p.provider.Exec(ctx, query, args...)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "mark_synced")))
		return fmt.Errorf("failed to mark records as synced: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (p *ragSyncProvider) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, rec := range records {
		var vecArg interface{}
		if !p.provider.IsSQLite() {
			// For postgres, we could use pgvector string format or standard binary.
			// pgvector uses string representation "[1,2,3]" or []float32 for jackc/pgx depending on setup.
			// Using JSON string to represent it for generic storage if pgvector is not fully typed, or standard encode.
			// Let's use string encoding which is safer cross-db without pgvector types registered
			vecArg, _ = encodeVectorToString(rec.Vector)
		} else {
			// SQLite
			vecArg, _ = encodeVectorToString(rec.Vector)
		}

		// Use standard ON CONFLICT DO UPDATE
		// memory_id is PRIMARY KEY
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at, organization_id)
			VALUES ($1, $2, $3, $4, $5, 'system')
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`

		lastSync := sql.NullTime{Time: rec.LastSyncAt, Valid: !rec.LastSyncAt.IsZero()}

		_, err := p.provider.Exec(ctx, query, rec.ID, rec.Context, vecArg, string(SyncStatusSynced), lastSync)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming")))
			return fmt.Errorf("failed to upsert incoming sync record %s: %w", rec.ID, err)
		}
	}

	return nil
}
