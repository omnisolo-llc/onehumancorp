package hub

import (
    "context"
    "time"
    "fmt"
    "strings"
    "strconv"

    "github.com/jackc/pgx/v5"
    "github.com/jackc/pgx/v5/pgxpool"
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
    ID           string
    Context      string
    Vector       []float32 // pgvector vector type
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
    meter = otel.GetMeterProvider().Meter("github.com/onehumancorp/mono/srcs/server/hub")

    RagRecordsSyncedTotal metric.Int64Counter
    RagSyncErrorsTotal    metric.Int64Counter
)

func init() {
    var err error
    RagRecordsSyncedTotal, err = meter.Int64Counter(
        "rag_records_synced_total",
        metric.WithDescription("Total number of RAG records successfully synced"),
    )
    if err != nil {
        panic(err)
    }

    RagSyncErrorsTotal, err = meter.Int64Counter(
        "rag_sync_errors_total",
        metric.WithDescription("Total number of RAG sync errors"),
    )
    if err != nil {
        panic(err)
    }
}

// PostgresRAGSyncService is a concrete implementation of RAGSyncService
type PostgresRAGSyncService struct {
    db *pgxpool.Pool
}

func NewPostgresRAGSyncService(db *pgxpool.Pool) *PostgresRAGSyncService {
    return &PostgresRAGSyncService{db: db}
}

func (s *PostgresRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `
        SELECT id, content, embedding::text, sync_status, last_sync_at
        FROM autodream_memories
        WHERE sync_status = $1
        LIMIT $2
    `
    rows, err := s.db.Query(ctx, query, SyncStatusPending, limit)
    if err != nil {
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var lastSyncAt *time.Time
        var embeddingStr *string
        if err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &r.SyncStatus, &lastSyncAt); err != nil {
            return nil, fmt.Errorf("failed to scan row: %w", err)
        }
        if lastSyncAt != nil {
            r.LastSyncAt = *lastSyncAt
        }
        if embeddingStr != nil {
             r.Vector, err = parsePgVector(*embeddingStr)
             if err != nil {
                 return nil, fmt.Errorf("failed to parse pgvector: %w", err)
             }
        }

        records = append(records, r)
    }
    return records, nil
}

func (s *PostgresRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    query := `
        UPDATE autodream_memories
        SET sync_status = $1, last_sync_at = $2
        WHERE id = ANY($3)
    `
    now := time.Now()
    _, err := s.db.Exec(ctx, query, SyncStatusSynced, now, ids)
    if err != nil {
        RagSyncErrorsTotal.Add(ctx, 1)
        return fmt.Errorf("failed to mark synced: %w", err)
    }
    RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *PostgresRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    batch := &pgx.Batch{}

    for _, r := range records {
        query := `
            INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
            VALUES ($1, $2, $3::vector, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                content = EXCLUDED.content,
                embedding = EXCLUDED.embedding,
                sync_status = EXCLUDED.sync_status,
                last_sync_at = EXCLUDED.last_sync_at
        `
        var embeddingStr *string
        if r.Vector != nil {
            str := formatPgVector(r.Vector)
            embeddingStr = &str
        }

        batch.Queue(query, r.ID, r.Context, embeddingStr, r.SyncStatus, r.LastSyncAt)
    }

    br := s.db.SendBatch(ctx, batch)
    defer br.Close()

    for i := 0; i < len(records); i++ {
       _, err := br.Exec()
        if err != nil {
            RagSyncErrorsTotal.Add(ctx, 1)
            return fmt.Errorf("failed to process incoming sync batch: %w", err)
        }
    }

    RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
    return nil
}

func parsePgVector(s string) ([]float32, error) {
    if s == "" || s == "[]" {
        return nil, nil
    }
    s = strings.TrimPrefix(s, "[")
    s = strings.TrimSuffix(s, "]")
    parts := strings.Split(s, ",")
    vector := make([]float32, len(parts))
    for i, p := range parts {
        v, err := strconv.ParseFloat(strings.TrimSpace(p), 32)
        if err != nil {
            return nil, err
        }
        vector[i] = float32(v)
    }
    return vector, nil
}

func formatPgVector(v []float32) string {
    if v == nil {
        return "[]"
    }
    parts := make([]string, len(v))
    for i, val := range v {
        parts[i] = strconv.FormatFloat(float64(val), 'f', -1, 32)
    }
    return "[" + strings.Join(parts, ",") + "]"
}
