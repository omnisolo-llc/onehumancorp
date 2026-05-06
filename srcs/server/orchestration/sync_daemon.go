package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"time"

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

type SQLiteProvider interface {
	TaskStore
	GetDB() *sql.DB
	Lock()
	Unlock()
}

type PostgresProvider interface {
	TaskStore
}

// Add these methods to SqliteTaskStore so it implements SQLiteProvider
func (s *SqliteTaskStore) GetDB() *sql.DB {
	return s.db
}

func (s *SqliteTaskStore) Lock() {
	s.mutex.Lock()
}

func (s *SqliteTaskStore) Unlock() {
	s.mutex.Unlock()
}

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

// SyncPendingMissions queries the database for agent_missions with status 'CLOUD_ESCALATION'
// and synced_to_cloud = false, then attempts to sync them to the remote API.
func (d *HybridMCPRAGDaemon) SyncPendingMissions(ctx context.Context) error {
	rows, err := d.db.QueryContext(ctx, "SELECT id, status, payload, mission_log FROM agent_missions WHERE synced_to_cloud = false AND status = 'CLOUD_ESCALATION' LIMIT 100")
	if err != nil {
		return fmt.Errorf("sync_daemon: failed to query agent_missions: %w", err)
	}

	type mission struct {
		id         string
		status     string
		payload    []byte
		missionLog sql.NullString
	}
	var missions []mission

	for rows.Next() {
		var m mission
		if err := rows.Scan(&m.id, &m.status, &m.payload, &m.missionLog); err != nil {
			log.Printf("sync_daemon: failed to scan row: %v", err)
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
		err = d.syncToCloud(ctx, m.id, m.payload, m.missionLog.String)

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

		syncedCount++
	}

	log.Printf("sync_daemon: successfully synced %d agent_missions", syncedCount)
	return nil
}

// syncToCloud simulates the actual RPC/HTTP call to the cloud endpoint
func (d *HybridMCPRAGDaemon) syncToCloud(ctx context.Context, id string, payload []byte, missionLog string) error {
	// In a real implementation, this would use d.remoteURL and make an HTTP/gRPC request.
	// For this test daemon, we just return nil assuming success.
	return nil
}

// StartSyncDaemon monitors local SQLite for missions marked for 'CLOUD_ESCALATION'.
// It sanitizes the payload, injects it into the cloud Postgres DB, and polls for completion.
func StartSyncDaemon(ctx context.Context, localDB SQLiteProvider, cloudDB PostgresProvider) {
	ticker := time.NewTicker(100 * time.Millisecond)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			syncPendingEscalations(ctx, localDB, cloudDB)
			syncCompletedEscalations(ctx, localDB, cloudDB)
		}
	}
}

func syncPendingEscalations(ctx context.Context, localDB SQLiteProvider, cloudDB PostgresProvider) error {
	localDB.Lock()
	defer localDB.Unlock()

	query := `
		SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies
		FROM shared_tasks
		WHERE status = 'CLOUD_ESCALATION'
	`
	rows, err := localDB.GetDB().QueryContext(ctx, query)
	if err != nil {
		return err
	}
	defer rows.Close()

	var tasksToUpdate []SharedTask

	for rows.Next() {
		var task SharedTask
		var payloadBytes, depsBytes []byte
		if err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
			&task.AgentID, &task.Priority, &payloadBytes, &task.ParentPlanID, &depsBytes,
		); err != nil {
			log.Printf("Error scanning local task: %v", err)
			continue
		}

		if len(payloadBytes) > 0 {
			sanitized, err := SanitizePayload(string(payloadBytes))
			if err != nil {
				log.Printf("Error sanitizing payload for task %s: %v", task.ID, err)
				continue
			}
			sanitizedRaw := json.RawMessage(sanitized)
			task.Payload = &sanitizedRaw
		}

		if len(depsBytes) > 0 {
			task.Dependencies = json.RawMessage(depsBytes)
		}

		// Change status to PENDING for cloud
		task.Status = "PENDING"
		// Insert into cloud DB
		err = cloudDB.CreateTask(ctx, &task)
		if err != nil {
			log.Printf("Error creating task in cloud DB: %v", err)
			continue
		}

		tasksToUpdate = append(tasksToUpdate, task)

		if escalationsCounter != nil {
			escalationsCounter.Add(ctx, 1)
		}
	}

	rows.Close()

	for _, task := range tasksToUpdate {
		// Update local status to avoid re-syncing
		updateQuery := `UPDATE shared_tasks SET status = 'CLOUD_PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?`
		localDB.GetDB().ExecContext(ctx, updateQuery, task.ID)
	}

	return nil
}

func syncCompletedEscalations(ctx context.Context, localDB SQLiteProvider, cloudDB PostgresProvider) error {
	localDB.Lock()
	defer localDB.Unlock()

	query := `
		SELECT id
		FROM shared_tasks
		WHERE status = 'CLOUD_PROCESSING'
	`
	rows, err := localDB.GetDB().QueryContext(ctx, query)
	if err != nil {
		return err
	}
	defer rows.Close()

	var taskIDs []string
	for rows.Next() {
		var id string
		if err := rows.Scan(&id); err != nil {
			log.Printf("Error scanning local task ID: %v", err)
			continue
		}
		taskIDs = append(taskIDs, id)
	}
	if err := rows.Err(); err != nil {
		return err
	}

	for _, id := range taskIDs {
		// Check cloud DB
		cloudTask, err := cloudDB.GetTask(ctx, id)
		if err != nil {
			if err != sql.ErrNoRows {
				log.Printf("Error getting task from cloud DB: %v", err)
			}
			continue
		}

		if cloudTask.Status == "DONE" {
			// Pull results back and update local DB
			updateQuery := `UPDATE shared_tasks SET status = 'DONE', payload = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?`
			var payloadBytes []byte
			if cloudTask.Payload != nil {
				payloadBytes = []byte(*cloudTask.Payload)
			}
			localDB.GetDB().ExecContext(ctx, updateQuery, payloadBytes, id)
		}
	}
	return nil
}
