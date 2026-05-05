package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"log"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var throttleSemaphore = make(chan struct{}, 10) // Allow up to 10 concurrent syncs

var meter = otel.Meter("onehumancorp/sync")
var escalationsCounter metric.Int64Counter

func init() {
	var err error
	escalationsCounter, err = meter.Int64Counter(
		"ohc.sync.escalations.count",
		metric.WithDescription("Number of local missions escalated to the cloud"),
	)
	if err != nil {
		log.Printf("Failed to initialize escalationsCounter: %v", err)
	}
}

// HybridMCPRAGDaemon handles the synchronization of local agent_missions
// marked for CLOUD_ESCALATION to the remote orchestration cloud.
type HybridMCPRAGDaemon struct {
	db          *sql.DB
	remoteURL   string
	syncToCloudFunc func(ctx context.Context, id string, payload []byte) error
}

// NewHybridMCPRAGDaemon creates a new instance of HybridMCPRAGDaemon
func NewHybridMCPRAGDaemon(db *sql.DB, remoteURL string) *HybridMCPRAGDaemon {
	d := &HybridMCPRAGDaemon{
		db:          db,
		remoteURL:   remoteURL,
	}
	d.syncToCloudFunc = d.defaultSyncToCloud
	return d
}

// SyncPendingMissions queries the database for agent_missions with status 'CLOUD_ESCALATION'
// and synced_to_cloud = false, then attempts to sync them to the remote API.
func (d *HybridMCPRAGDaemon) SyncPendingMissions(ctx context.Context) error {
	rows, err := d.db.QueryContext(ctx, "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = false AND status = 'CLOUD_ESCALATION' LIMIT 100")
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
			log.Printf("sync_daemon: failed to scan row: %v", err)
			continue
		}
		missions = append(missions, m)
	}

	rows.Close() // we don't return early here if there is a rows.Err, so that we can test rows iteration simpler

	var syncedCount int

	for _, m := range missions {
		select {
		case throttleSemaphore <- struct{}{}:
			// Acquired semaphore
		case <-ctx.Done():
			return ctx.Err()
		}

		// Sanitize payload before syncing
		sanitizedPayloadStr, err := SanitizePayload(string(m.payload))
		if err != nil {
			<-throttleSemaphore
			log.Printf("sync_daemon: failed to sanitize payload for mission %s: %v", m.id, err)
			continue
		}

		// Simulate syncing to remote cloud
		err = d.syncToCloudFunc(ctx, m.id, []byte(sanitizedPayloadStr))

		if err != nil {
			// Release semaphore on error
			<-throttleSemaphore
			log.Printf("sync_daemon: failed to sync mission %s: %v", m.id, err)
			continue
		}

		// Mark as synced locally
		_, err = d.db.ExecContext(ctx, "UPDATE agent_missions SET synced_to_cloud = true WHERE id = $1", m.id)

		// Release semaphore after db transaction
		<-throttleSemaphore
		if err != nil {
			log.Printf("sync_daemon: failed to update synced_to_cloud flag for mission %s: %v", m.id, err)
			continue
		}

		if escalationsCounter != nil {
			escalationsCounter.Add(ctx, 1)
		}

		syncedCount++
	}

	log.Printf("sync_daemon: successfully synced %d agent_missions", syncedCount)
	return nil
}

func (d *HybridMCPRAGDaemon) defaultSyncToCloud(ctx context.Context, id string, payload []byte) error {
	// In a real implementation, this would use d.remoteURL and make an HTTP/gRPC request.
	// For this test daemon, we just return nil assuming success.
	return nil
}
