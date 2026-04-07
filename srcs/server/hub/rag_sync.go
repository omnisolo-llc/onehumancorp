package hub

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// SyncStatus represents the sync state of a RAG record.
type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

// RAGSyncRecord holds the data and sync metadata for a context memory.
type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

// RAGSyncService defines the interface for synchronizing local RAG memories to the Cloud.
type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

// DefaultRAGSyncService is the default implementation.
type DefaultRAGSyncService struct {
	db db.Provider
}

// NewDefaultRAGSyncService creates a new sync service.
func NewDefaultRAGSyncService(db db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: db}
}

// FetchPendingSyncs retrieves records from the local DB that need syncing.
func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		ORDER BY created_at ASC
		LIMIT $1
	`
	if s.db.IsSQLite() {
		query = `
			SELECT id, content, embedding, sync_status, last_sync_at
			FROM autodream_memories
			WHERE sync_status = 'pending'
			ORDER BY created_at ASC
			LIMIT ?
		`
	}

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		var vectorBytes []byte // Pgvector returns string or byte array depending on driver, SQLite returns whatever we stored
		if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &rec.SyncStatus, &lastSyncAt); err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return nil, err
		}
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}

		if len(vectorBytes) > 0 {
			// For pgx/vector, it could be a string representation like "[0.1,0.2]".
			// Or a JSON array. We attempt to unmarshal JSON.
			// SQLite driver gives us []byte.
			strVal := string(vectorBytes)
			if !strings.HasPrefix(strVal, "[") && !strings.HasSuffix(strVal, "]") {
				// Pgvector sometimes formats as [1,2,3], which is JSON compatible.
				// If not JSON compatible, we need special parsing, but standard vector cast in Postgres returns text like "[1,2,3]".
			}
			var vec []float32
			if err := json.Unmarshal(vectorBytes, &vec); err == nil {
				rec.Vector = vec
			}
		}

		records = append(records, rec)
	}
	return records, rows.Err()
}

// MarkSynced updates the local DB after a successful sync to the cloud.
func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create placeholders for IN clause
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		if s.db.IsSQLite() {
			placeholders[i] = "?"
		} else {
			placeholders[i] = fmt.Sprintf("$%d", i+1)
		}
		args[i] = id
	}

	query := fmt.Sprintf(`UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id IN (%s)`, strings.Join(placeholders, ","))

	_, err := s.db.Exec(ctx, query, args...)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}

	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB.
func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var vectorBytes []byte
		if len(rec.Vector) > 0 {
			vectorBytes, _ = json.Marshal(rec.Vector)
		}

		query := `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
		if s.db.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`
		}

		// Use nil for empty vectorBytes if pgvector requires NULL instead of empty
		var vecArg interface{} = vectorBytes
		if len(vectorBytes) == 0 {
			vecArg = nil
		}

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vecArg)
		if err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}

	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}

	return nil
}

// StartSyncDaemon starts a background worker that polls the local SQLite database
// and pushes pending sync records to the cloud gateway.
func StartSyncDaemon(ctx context.Context, service RAGSyncService, cloudGatewayURL string, token string, pollInterval time.Duration) {
	slog.Info("Starting RAG Sync Daemon", "poll_interval", pollInterval)

	go func() {
		ticker := time.NewTicker(pollInterval)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				slog.Info("RAG Sync Daemon shutting down")
				return
			case <-ticker.C:
				records, err := service.FetchPendingSyncs(ctx, 50)
				if err != nil {
					slog.Error("SyncDaemon: Failed to fetch pending syncs", "error", err)
					continue
				}

				if len(records) == 0 {
					continue
				}

				// Push to Cloud API
				payload, err := json.Marshal(records)
				if err != nil {
					slog.Error("SyncDaemon: Failed to marshal payload", "error", err)
					continue
				}

				req, err := http.NewRequestWithContext(ctx, "POST", cloudGatewayURL+"/api/v1/sync/rag", bytes.NewBuffer(payload))
				if err != nil {
					slog.Error("SyncDaemon: Failed to create request", "error", err)
					continue
				}

				req.Header.Set("Content-Type", "application/json")
				req.Header.Set("Authorization", "Bearer "+token)

				client := &http.Client{Timeout: 30 * time.Second}
				resp, err := client.Do(req)
				if err != nil {
					slog.Error("SyncDaemon: Failed to sync with cloud", "error", err)
					if telemetry.RagSyncErrorsTotal != nil {
						telemetry.RagSyncErrorsTotal.Add(ctx, 1)
					}
					continue
				}

				if resp.StatusCode != http.StatusOK {
					slog.Error("SyncDaemon: Cloud API returned non-200 status", "status", resp.StatusCode)
					if telemetry.RagSyncErrorsTotal != nil {
						telemetry.RagSyncErrorsTotal.Add(ctx, 1)
					}
					resp.Body.Close()
					continue
				}
				resp.Body.Close()

				// Mark as synced locally
				ids := make([]string, len(records))
				for i, r := range records {
					ids[i] = r.ID
				}

				if err := service.MarkSynced(ctx, ids); err != nil {
					slog.Error("SyncDaemon: Failed to mark records as synced", "error", err)
				} else {
					slog.Info("SyncDaemon: Successfully synced records", "count", len(ids))
				}
			}
		}
	}()
}
