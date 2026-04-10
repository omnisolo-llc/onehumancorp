package hub

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// RAGSyncManager implements the RAGSyncService interface.
type RAGSyncManager struct {
	dbProvider      db.Provider
	cloudGatewayURL string
	httpClient      *http.Client
}

// NewRAGSyncManager creates a new RAGSyncManager.
func NewRAGSyncManager(dbProvider db.Provider, cloudGatewayURL string) *RAGSyncManager {
	return &RAGSyncManager{
		dbProvider:      dbProvider,
		cloudGatewayURL: cloudGatewayURL,
		httpClient:      &http.Client{Timeout: 30 * time.Second},
	}
}

func (m *RAGSyncManager) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	rows, err := m.dbProvider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return records, nil
}

func (m *RAGSyncManager) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create placeholders for the IN clause
	placeholders := make([]string, len(ids))
	args := make([]any, len(ids)+1)

	now := time.Now()
	args[0] = now

	for i, id := range ids {
		// Start parameter index at 2 since $1 is 'now'
		placeholders[i] = fmt.Sprintf("$%d", i+2)
		args[i+1] = id
	}

	query := fmt.Sprintf(`UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id IN (%s)`, strings.Join(placeholders, ", "))

	_, err := m.dbProvider.Exec(ctx, query, args...)
	if err != nil {
		return fmt.Errorf("failed to update sync status: %w", err)
	}
	return nil
}

func (m *RAGSyncManager) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		query := `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := m.dbProvider.Exec(ctx, query, r.ID, r.Context, r.SyncStatus, r.LastSyncAt)
		if err != nil {
			return fmt.Errorf("failed to upsert incoming record %s: %w", r.ID, err)
		}
	}
	return nil
}

// SyncToCloud initiates the push sync process to the cloud endpoint.
func (m *RAGSyncManager) SyncToCloud(ctx context.Context, limit int) error {
	records, err := m.FetchPendingSyncs(ctx, limit)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx, "fetch_failed")
		return err
	}

	if len(records) == 0 {
		return nil
	}

	payload, err := json.Marshal(records)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx, "marshal_failed")
		return fmt.Errorf("failed to marshal sync records: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, m.cloudGatewayURL+"/v1/sync/rag", bytes.NewReader(payload))
	if err != nil {
		telemetry.RecordRAGSyncError(ctx, "request_creation_failed")
		return fmt.Errorf("failed to create sync request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := m.httpClient.Do(req)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx, "http_do_failed")
		return fmt.Errorf("failed to execute sync request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		telemetry.RecordRAGSyncError(ctx, "http_error_response")
		return fmt.Errorf("cloud sync failed with status %d", resp.StatusCode)
	}

	var syncedIDs []string
	for _, r := range records {
		syncedIDs = append(syncedIDs, r.ID)
	}

	if err := m.MarkSynced(ctx, syncedIDs); err != nil {
		telemetry.RecordRAGSyncError(ctx, "mark_synced_failed")
		return err
	}

	telemetry.RecordRAGSyncSuccess(ctx, len(syncedIDs))
	return nil
}
