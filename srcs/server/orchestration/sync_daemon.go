package orchestration

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var throttleSemaphore = make(chan struct{}, 10) // Allow up to 10 concurrent syncs

var meter = otel.Meter("onehumancorp/sync")
var escalationsCounter metric.Int64Counter
var syncDaemonErrorTotal metric.Int64Counter
var syncLatencyMs metric.Float64Histogram
var syncDaemonBatchSize metric.Int64Counter

func init() {
	var err error
	escalationsCounter, err = meter.Int64Counter(
		"ohc.sync.escalations.count",
		metric.WithDescription("Number of local missions escalated to the cloud"),
	)
	if err != nil {
		log.Printf("Failed to initialize escalationsCounter: %v", err)
	}

	syncDaemonErrorTotal, err = meter.Int64Counter(
		"sync_daemon_error_total",
		metric.WithDescription("Total number of sync errors"),
	)
	if err != nil {
		log.Printf("Failed to initialize syncDaemonErrorTotal: %v", err)
	}

	syncLatencyMs, err = meter.Float64Histogram(
		"sync_latency_ms",
		metric.WithDescription("Latency of sync operations in ms"),
	)
	if err != nil {
		log.Printf("Failed to initialize syncLatencyMs: %v", err)
	}

	syncDaemonBatchSize, err = meter.Int64Counter(
		"sync_daemon_batch_size",
		metric.WithDescription("Batch size of sync operations"),
	)
	if err != nil {
		log.Printf("Failed to initialize syncDaemonBatchSize: %v", err)
	}
}

func getSyncMode() string {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return "Cloud"
	}
	return "Standalone"
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
	if remoteURL == "" {
		remoteURL = os.Getenv("OHC_CORE_URL")
	}
	return &HybridMCPRAGDaemon{
		db:          db,
		remoteURL:   remoteURL,
	}
}

// SyncPendingMissions queries the database for agent_missions with status 'CLOUD_ESCALATION'
// and synced_to_cloud = false, then attempts to sync them to the remote API.
func (d *HybridMCPRAGDaemon) SyncPendingMissions(ctx context.Context) error {
	mode := getSyncMode()
	start := time.Now()

	rows, err := d.db.QueryContext(ctx, "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = false AND status = 'CLOUD_ESCALATION' LIMIT 100")
	if err != nil {
		if syncDaemonErrorTotal != nil {
			syncDaemonErrorTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("mode", mode), attribute.String("error", "DB_ERROR")))
		}
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
			continue
		}
		missions = append(missions, m)
	}

	if err := rows.Err(); err != nil {
		rows.Close()
		if syncDaemonErrorTotal != nil {
			syncDaemonErrorTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("mode", mode), attribute.String("error", "DB_ITERATION_ERROR")))
		}
		return fmt.Errorf("sync_daemon: rows iteration error: %w", err)
	}
	rows.Close()

	if syncDaemonBatchSize != nil {
		syncDaemonBatchSize.Add(ctx, int64(len(missions)), metric.WithAttributes(attribute.String("mode", mode)))
	}

	var syncedCount int

	for _, m := range missions {
		select {
		case throttleSemaphore <- struct{}{}:
			// Acquired semaphore
		case <-ctx.Done():
			return ctx.Err()
		}

		// Syncing to remote cloud
		err = d.syncToCloud(ctx, m.id, m.status, m.payload)

		if err != nil {
			// Release semaphore on error
			<-throttleSemaphore
			if syncDaemonErrorTotal != nil {
				syncDaemonErrorTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("mode", mode), attribute.String("error", "HTTP_ERROR")))
			}
			continue
		}

		// Mark as synced locally
		_, err = d.db.ExecContext(ctx, "UPDATE agent_missions SET synced_to_cloud = true WHERE id = $1", m.id)

		// Release semaphore after db transaction
		<-throttleSemaphore
		if err != nil {
			if syncDaemonErrorTotal != nil {
				syncDaemonErrorTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("mode", mode), attribute.String("error", "DB_ERROR")))
			}
			continue
		}

		syncedCount++
	}

	if syncLatencyMs != nil {
		syncLatencyMs.Record(ctx, float64(time.Since(start).Milliseconds()), metric.WithAttributes(attribute.String("mode", mode)))
	}

	return nil
}

// syncToCloud makes an HTTP request to the cloud endpoint to sync a mission
func (d *HybridMCPRAGDaemon) syncToCloud(ctx context.Context, id string, status string, payload []byte) error {
	sanitizedPayloadStr, err := SanitizePayload(string(payload))
	if err != nil {
		return fmt.Errorf("failed to sanitize payload for mission %s: %w", id, err)
	}

	sanitizedRaw := json.RawMessage(sanitizedPayloadStr)

	task := SharedTask{
		ID:      id,
		Status:  status,
		Payload: &sanitizedRaw,
	}

	requestBody, err := json.Marshal(task)
	if err != nil {
		return fmt.Errorf("failed to marshal request body for mission %s: %w", id, err)
	}

	endpoint := fmt.Sprintf("%s/api/sync/missions", d.remoteURL)
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, endpoint, bytes.NewReader(requestBody))
	if err != nil {
		return fmt.Errorf("failed to create sync request for mission %s: %w", id, err)
	}

	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer " + os.Getenv("OHC_SYNC_AUTH_TOKEN"))

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("sync request failed for mission %s: %w", id, err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusCreated {
		bodyBytes, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("sync request returned unexpected status %d for mission %s: %s", resp.StatusCode, id, string(bodyBytes))
	}

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
	mode := getSyncMode()
	start := time.Now()

	localDB.Lock()
	defer localDB.Unlock()

	query := `
		SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies
		FROM shared_tasks
		WHERE status = 'CLOUD_ESCALATION'
		LIMIT 100
	`
	rows, err := localDB.GetDB().QueryContext(ctx, query)
	if err != nil {
		if syncDaemonErrorTotal != nil {
			syncDaemonErrorTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("mode", mode), attribute.String("error", "DB_ERROR")))
		}
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
			if syncDaemonErrorTotal != nil {
				syncDaemonErrorTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("mode", mode), attribute.String("error", "DB_ITERATION_ERROR")))
			}
			log.Printf("Error scanning local task: %v", err)
			continue
		}

		if len(payloadBytes) > 0 {
			sanitized, err := SanitizePayload(string(payloadBytes))
			if err != nil {
				if syncDaemonErrorTotal != nil {
					syncDaemonErrorTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("mode", mode), attribute.String("error", "SANITIZE_ERROR")))
				}
				log.Printf("Error sanitizing task %s", task.ID)
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
			if syncDaemonErrorTotal != nil {
				syncDaemonErrorTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("mode", mode), attribute.String("error", "CLOUD_SYNC_ERROR")))
			}
			log.Printf("Error creating task in cloud DB: %v", err)
			continue
		}

		tasksToUpdate = append(tasksToUpdate, task)

		if escalationsCounter != nil {
			escalationsCounter.Add(ctx, 1)
		}
	}

	rows.Close()

	if syncDaemonBatchSize != nil {
		syncDaemonBatchSize.Add(ctx, int64(len(tasksToUpdate)), metric.WithAttributes(attribute.String("mode", mode)))
	}

	for _, task := range tasksToUpdate {
		// Update local status to avoid re-syncing
		updateQuery := `UPDATE shared_tasks SET status = 'CLOUD_PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?`
		localDB.GetDB().ExecContext(ctx, updateQuery, task.ID)
	}

	if syncLatencyMs != nil {
		syncLatencyMs.Record(ctx, float64(time.Since(start).Milliseconds()), metric.WithAttributes(attribute.String("mode", mode)))
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
		cloudTask, err := cloudDB.GetTask(ctx, id, localTask.OrganizationID)
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

// DUMMY VALIDATION COMMENT
