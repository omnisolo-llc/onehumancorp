package hub

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type DefaultRAGSyncService struct {
	dbProvider db.Provider
	httpClient *http.Client
	cloudURL   string
}

func NewDefaultRAGSyncService(dbProvider db.Provider, httpClient *http.Client, cloudURL string) *DefaultRAGSyncService {
	if httpClient == nil {
		httpClient = http.DefaultClient
	}
	return &DefaultRAGSyncService{
		dbProvider: dbProvider,
		httpClient: httpClient,
		cloudURL:   cloudURL,
	}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, sync_status FROM autodream_memories WHERE sync_status = $1 LIMIT $2`
	rows, err := s.dbProvider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var statusStr string
		if err := rows.Scan(&r.ID, &r.Context, &statusStr); err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return nil, err
		}
		r.SyncStatus = SyncStatus(statusStr)
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	// For simplicity in SQLite/Postgres compatibility, we update one by one or use a transaction.
	// We'll use a simple loop here for demonstration, though a batch query is better in production.
	for _, id := range ids {
		query := `UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id = $3`
		_, err := s.dbProvider.Exec(ctx, query, string(SyncStatusSynced), time.Now(), id)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
	}
	telemetry.RecordRAGRecordsSynced(ctx, len(ids))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	payload, err := json.Marshal(records)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}

	req, err := http.NewRequestWithContext(ctx, "POST", s.cloudURL, bytes.NewBuffer(payload))
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := s.httpClient.Do(req)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	var syncedIDs []string
	for _, r := range records {
		syncedIDs = append(syncedIDs, r.ID)
	}
	return s.MarkSynced(ctx, syncedIDs)
}
