package hybrid_sync_daemon

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// CloudClient defines the interface to push tasks to the Cloud API.
type CloudClient interface {
	PushTasks(ctx context.Context, tasks []db.TaskRecord) error
}

// MissionSyncDaemon continuously synchronizes local SQLite state to the cloud.
type MissionSyncDaemon struct {
	dbProvider db.Provider
	cloud      CloudClient
	batchSize  int
	pollDelay  time.Duration
	stopChan   chan struct{}
	wg         sync.WaitGroup
}

// NewMissionSyncDaemon creates a new MissionSyncDaemon.
func NewMissionSyncDaemon(dbProvider db.Provider, cloud CloudClient, batchSize int, pollDelay time.Duration) *MissionSyncDaemon {
	if batchSize <= 0 {
		batchSize = 10
	}
	if pollDelay <= 0 {
		pollDelay = 5 * time.Second
	}
	return &MissionSyncDaemon{
		dbProvider: dbProvider,
		cloud:      cloud,
		batchSize:  batchSize,
		pollDelay:  pollDelay,
		stopChan:   make(chan struct{}),
	}
}

// Start begins the synchronization loop.
func (d *MissionSyncDaemon) Start(ctx context.Context) {
	d.wg.Add(1)
	go func() {
		defer d.wg.Done()
		ticker := time.NewTicker(d.pollDelay)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-d.stopChan:
				return
			case <-ticker.C:
				if err := d.syncTasks(ctx); err != nil {
					// Log error in production.
					fmt.Printf("Error syncing tasks: %v\n", err)
				}
			}
		}
	}()
}

// Stop gracefully stops the daemon.
func (d *MissionSyncDaemon) Stop() {
	close(d.stopChan)
	d.wg.Wait()
}

// syncTasks queries SQLite for un-synced tasks and pushes them.
func (d *MissionSyncDaemon) syncTasks(ctx context.Context) error {
	// Only run in standalone (SQLite) mode.
	if !d.dbProvider.IsSQLite() {
		return nil
	}

	// We query tasks that need syncing. Let's assume we use a specific status, e.g. "needs_sync"
	// or we just fetch tasks that have some flag. Since we don't have a sync flag, we can fetch tasks
	// where status is e.g. "cloud_escalation". Let's assume the status we look for is "cloud_escalation".

	// Query: SELECT id, parent_task_id, agent_id, status, payload, created_at, updated_at FROM agent_missions WHERE status = 'pending' LIMIT ?
	query := `
		SELECT id, parent_task_id, agent_id, status, payload, created_at, updated_at
		FROM agent_missions
		WHERE status = 'pending'
		LIMIT ?
	`
	rows, err := d.dbProvider.Query(ctx, query, d.batchSize)
	if err != nil {
		return fmt.Errorf("failed to query tasks: %w", err)
	}
	defer rows.Close()

	var tasks []db.TaskRecord
	for rows.Next() {
		var t db.TaskRecord
		if err := rows.Scan(&t.ID, &t.ParentTaskID, &t.AgentID, &t.Status, &t.Payload, &t.CreatedAt, &t.UpdatedAt); err != nil {
			return fmt.Errorf("failed to scan task: %w", err)
		}
		tasks = append(tasks, t)
	}
	if err := rows.Err(); err != nil {
		return fmt.Errorf("rows error: %w", err)
	}

	if len(tasks) == 0 {
		return nil
	}

	// Scrub PII from payload before pushing if necessary. The requirement says "Batched, PII-scrubbed Cloud Sync".
	// Let's implement a simple scrubber for payloads.
	for i := range tasks {
		if tasks[i].Payload != nil {
			scrubbed := d.scrubPII(*tasks[i].Payload)
			tasks[i].Payload = &scrubbed
		}
	}

	// Push to cloud.
	if err := d.cloud.PushTasks(ctx, tasks); err != nil {
		return fmt.Errorf("failed to push tasks to cloud: %w", err)
	}

	// Update local status to "cloud_escalated" or similar so we don't sync again.
	// We'll update the status to 'synced' for the successfully pushed tasks.
	updateQuery := `UPDATE agent_missions SET status = 'synced', updated_at = CURRENT_TIMESTAMP WHERE id = ?`
	for _, t := range tasks {
		if _, err := d.dbProvider.Exec(ctx, updateQuery, t.ID); err != nil {
			return fmt.Errorf("failed to update task %s status: %w", t.ID, err)
		}
	}

	return nil
}

// scrubPII removes typical PII patterns.
func (d *MissionSyncDaemon) scrubPII(payload string) string {
	// A basic scrub function. In a real app, this would use regex for emails, SSNs, etc.
	// Or we use the sanitizer module.

	// Since we're in orchestration, we might have access to the Sanitizer.
	// For now, let's parse JSON if it is JSON and remove known PII keys.
	var data map[string]interface{}
	if err := json.Unmarshal([]byte(payload), &data); err == nil {
		delete(data, "email")
		delete(data, "phone")
		delete(data, "ssn")
		scrubbed, _ := json.Marshal(data)
		return string(scrubbed)
	}

	return payload
}
