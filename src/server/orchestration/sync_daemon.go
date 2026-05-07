package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"log"
)

var throttleSemaphore = make(chan struct{}, 10) // Allow up to 10 concurrent syncs

// HybridMCPRAGDaemon handles the synchronization of local agent_missions
// marked for CLOUD_ESCALATION to the remote orchestration cloud.
type HybridMCPRAGDaemon struct {
	db          *sql.DB
	remoteURL   string
}

// NewHybridMCPRAGDaemon creates a new instance of HybridMCPRAGDaemon
func NewHybridMCPRAGDaemon(db *sql.DB, remoteURL string) *HybridMCPRAGDaemon {
	return &HybridMCPRAGDaemon{
		db:          db,
		remoteURL:   remoteURL,
	}
}

// SyncPendingMissions queries the database for agent_missions with status 'CLOUD_ESCALATION' or 'BURSTING'
// and synced_to_cloud = false, then attempts to sync them to the remote API.
func (d *HybridMCPRAGDaemon) SyncPendingMissions(ctx context.Context) error {
	rows, err := d.db.QueryContext(ctx, "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = false AND (status = 'CLOUD_ESCALATION' OR status = 'BURSTING') AND (sync_error IS NULL OR last_synced_at < datetime('now', '-5 minutes')) LIMIT 100")
	if err != nil {
		return fmt.Errorf("sync_daemon: failed to query agent_missions: %w", err)
	}

	type mission struct {
		id      string
		status  string
		payload []byte
	}
	var missions []mission

	for rows.Next() {
		var m mission
		if err := rows.Scan(&m.id, &m.status, &m.payload); err != nil {
			log.Printf("sync_daemon: [DEBUG] failed to scan row: %v", err)
			continue
		}
		missions = append(missions, m)
	}

	if err := rows.Err(); err != nil {
		rows.Close()
		return fmt.Errorf("sync_daemon: rows iteration error: %w", err)
	}
	rows.Close()

	var syncedCount int

	for _, m := range missions {
		select {
		case throttleSemaphore <- struct{}{}:
			// Acquired semaphore
		case <-ctx.Done():
			return ctx.Err()
		}

		// Simulate syncing to remote cloud
		err = d.syncToCloud(ctx, m.id, m.payload)

		if err != nil {
			// Release semaphore on error
			<-throttleSemaphore
			_, _ = d.db.ExecContext(ctx, "UPDATE agent_missions SET sync_error = $1, last_synced_at = datetime('now') WHERE id = $2", err.Error(), m.id)
			log.Printf("sync_daemon: [DEBUG] failed to sync mission %s: %v", m.id, err)
			continue
		}

		// Mark as synced locally
		_, err = d.db.ExecContext(ctx, "UPDATE agent_missions SET synced_to_cloud = true WHERE id = $1", m.id)

		// Release semaphore after db transaction
		<-throttleSemaphore
		if err != nil {
			log.Printf("sync_daemon: [DEBUG] failed to update synced_to_cloud flag for mission %s: %v", m.id, err)
			continue
		}

		syncedCount++
	}

	// log.Printf("sync_daemon: successfully synced %d agent_missions", syncedCount)
	return nil
}

// syncToCloud simulates the actual RPC/HTTP call to the cloud endpoint
func (d *HybridMCPRAGDaemon) syncToCloud(ctx context.Context, id string, payload []byte) error {
	// In a real implementation, this would use d.remoteURL and make an HTTP/gRPC request.
	// For this test daemon, we just return nil assuming success.
	return nil
}
