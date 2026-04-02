package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// TaskOrchestrator defines the interface for task queue management and dependency tracking.
type TaskOrchestrator interface {
	EnqueueTask(ctx context.Context, task *SharedTask, dependsOn []string) (*SharedTask, error)
	AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*SharedTask, error)
	CompleteTask(ctx context.Context, taskID string, agentID string, result string) error
}

// QueueManager implements TaskOrchestrator.
type QueueManager struct {
	db          db.Provider
	redisClient rueidis.Client
	hub         *CentrifugeNode
	mu          sync.Mutex // For standalone mode coordination
}

// NewQueueManager creates a new QueueManager instance.
func NewQueueManager(provider db.Provider, rClient rueidis.Client, hub *CentrifugeNode) *QueueManager {
	return &QueueManager{
		db:          provider,
		redisClient: rClient,
		hub:         hub,
	}
}

// EnqueueTask adds a new task to the queue. If it has dependencies, its status is PENDING, else READY.
func (qm *QueueManager) EnqueueTask(ctx context.Context, task *SharedTask, dependsOn []string) (*SharedTask, error) {
	// Initialize ID and timestamp if not set
	if task.ID == "" {
		if qm.db.IsSQLite() {
			task.ID = fmt.Sprintf("%d", time.Now().UnixNano())
		} else {
			// In Postgres, let gen_random_uuid() handle it, or we could set a UUID ourselves.
			// The current schema uses DEFAULT gen_random_uuid()
			// Actually, if we don't pass it, we must scan it. Let's just generate it if empty, but usually the DB generates it.
		}
	}

	payloadMap := map[string]string{
		"description": task.Description,
		"priority":    task.Priority,
	}
	payloadBytes, _ := json.Marshal(payloadMap)
	task.Payload = string(payloadBytes)

	task.Status = "PENDING"
	if len(dependsOn) == 0 {
		task.Status = "READY"
	}

	tx, err := qm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	if qm.db.IsSQLite() {
		if task.ID == "" {
			task.ID = fmt.Sprintf("%d", time.Now().UnixNano())
		}
		query = `
			INSERT INTO shared_tasks (id, mission_id, title, description, priority, status, created_at, updated_at)
			VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
			RETURNING id, mission_id, title, description, priority, status, created_at, updated_at
		`
		err = tx.QueryRow(ctx, query, task.ID, task.MissionID, task.Title, task.Description, task.Priority, task.Status).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.CreatedAt, &task.UpdatedAt,
		)
	} else {
		query = `
			INSERT INTO shared_tasks (mission_id, title, description, priority, status)
			VALUES ($1, $2, $3, $4, $5)
			RETURNING id, mission_id, title, description, priority, status, created_at, updated_at
		`
		err = tx.QueryRow(ctx, query, task.MissionID, task.Title, task.Description, task.Priority, task.Status).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.CreatedAt, &task.UpdatedAt,
		)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to insert task: %w", err)
	}

	for _, depID := range dependsOn {
		_, err := tx.Exec(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)", task.ID, depID)
		if err != nil {
			return nil, fmt.Errorf("failed to insert task dependency: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	if qm.hub != nil {
		qm.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{
			"action":      "CREATE",
			"mission_id":  task.MissionID,
			"title":       task.Title,
			"description": task.Description,
			"priority":    task.Priority,
			"status":      task.Status,
		})
	}

	return task, nil
}

// AcquireReadyTask finds a READY task, locks it atomically to prevent TOCTOU, and sets it to IN_PROGRESS.
func (qm *QueueManager) AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*SharedTask, error) {
	// Only use single Mutex lock in standalone to prevent TOCTOU
	if qm.db.IsSQLite() {
		qm.mu.Lock()
		defer qm.mu.Unlock()
	}

	tx, err := qm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var task SharedTask
	var errQuery error

	if qm.db.IsSQLite() {
		query := `
			SELECT id, mission_id, title, description, priority, status, created_at, updated_at
			FROM shared_tasks
			WHERE status = 'READY'
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
		`
		errQuery = tx.QueryRow(ctx, query).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.CreatedAt, &task.UpdatedAt,
		)
	} else {
		// Postgres mode: rely on SKIP LOCKED
		query := `
			SELECT id, mission_id, title, description, priority, status, created_at, updated_at
			FROM shared_tasks
			WHERE status = 'READY'
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
		errQuery = tx.QueryRow(ctx, query).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.CreatedAt, &task.UpdatedAt,
		)
	}

	if errQuery != nil {
		if errors.Is(errQuery, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, fmt.Errorf("failed to query ready tasks: %w", errQuery)
	}

	// Cloud Mode distributed locking
	if qm.redisClient != nil {
		lockKey := "lock:task:" + task.ID
		cmd := qm.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Ex(30 * time.Second).Build()
		err := qm.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return nil, nil // Locked by another process
			}
			return nil, fmt.Errorf("failed to acquire redis lock: %w", err)
		}
	}

	// Update to IN_PROGRESS
	_, err = tx.Exec(ctx, `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND status = 'READY'
	`, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task to IN_PROGRESS: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = agentID

	if qm.hub != nil {
		qm.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{
			"action":   "CLAIM",
			"agent_id": agentID,
			"status":   task.Status,
		})
	}

	return &task, nil
}

// CompleteTask marks task as COMPLETED, unlocks dependent tasks.
func (qm *QueueManager) CompleteTask(ctx context.Context, taskID string, agentID string, result string) error {
	// Only use single Mutex lock in standalone to prevent TOCTOU
	if qm.db.IsSQLite() {
		qm.mu.Lock()
		defer qm.mu.Unlock()
	}

	tx, err := qm.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// Fetch the task first to check if it belongs to the agent
	var currentStatus, assignedAgentID string
	err = tx.QueryRow(ctx, "SELECT status, assigned_agent_id FROM shared_tasks WHERE id = $1", taskID).Scan(&currentStatus, &assignedAgentID)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return errors.New("task not found")
		}
		return fmt.Errorf("failed to query task: %w", err)
	}

	if currentStatus != "IN_PROGRESS" || assignedAgentID != agentID {
		return errors.New("task not in progress or not assigned to agent")
	}

	// Mark as completed
	_, err = tx.Exec(ctx, `
		UPDATE shared_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`, taskID)
	if err != nil {
		return fmt.Errorf("failed to mark task complete: %w", err)
	}

	// Resolve dependencies for other tasks
	// We need to find all PENDING tasks that depend on this one, and check if ALL their dependencies are now COMPLETED.
	// We can do this efficiently via SQL.

	// First, find which tasks depend on this one
	rows, err := tx.Query(ctx, "SELECT task_id FROM task_dependencies WHERE depends_on_task_id = $1", taskID)
	if err != nil {
		return fmt.Errorf("failed to find dependent tasks: %w", err)
	}
	var dependentTasks []string
	for rows.Next() {
		var dID string
		if err := rows.Scan(&dID); err != nil {
			rows.Close()
			return fmt.Errorf("failed to scan dependent task ID: %w", err)
		}
		dependentTasks = append(dependentTasks, dID)
	}
	rows.Close()

	for _, depTaskID := range dependentTasks {
		// Check if all dependencies for depTaskID are COMPLETED
		var uncompletedCount int
		err := tx.QueryRow(ctx, `
			SELECT COUNT(*)
			FROM task_dependencies td
			JOIN shared_tasks st ON td.depends_on_task_id = st.id
			WHERE td.task_id = $1 AND st.status != 'COMPLETED'
		`, depTaskID).Scan(&uncompletedCount)
		if err != nil {
			return fmt.Errorf("failed to count uncompleted dependencies for %s: %w", depTaskID, err)
		}

		if uncompletedCount == 0 {
			// All dependencies completed! Mark as READY
			_, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = 'READY', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND status = 'PENDING'", depTaskID)
			if err != nil {
				return fmt.Errorf("failed to update dependent task to READY: %w", err)
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	// Broadcast
	if qm.hub != nil {
		qm.hub.PublishTaskBroadcast(taskID, map[string]interface{}{
			"action":   "COMPLETE",
			"agent_id": agentID,
			"status":   "COMPLETED",
		})
	}

	// AutoDream Hook for background embedding insertion
	go func() {
		// Create a background context so it doesn't die when the original context is cancelled
		bgCtx := context.Background()
		apiKey := os.Getenv("MINIMAX_API_KEY")
		if apiKey == "" {
			return
		}
		baseClient := NewMinimaxClient(apiKey)
		client := NewCachedMinimaxClient(baseClient, qm.db, qm.redisClient)

		summary := fmt.Sprintf("Task ID: %s. Result: %s", taskID, result)
		vec, err := client.GenerateEmbedding(bgCtx, summary)
		if err != nil {
			return
		}

		// Insert into autodream_memories
		var insertQuery string
		if qm.db.IsSQLite() {
			vecBytes, _ := json.Marshal(vec)
			insertQuery = `
				INSERT INTO autodream_memories (content, embedding, source_mission_id, consolidated_at)
				VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
			`
			_, _ = qm.db.Exec(bgCtx, insertQuery, summary, string(vecBytes), taskID)
		} else {
			// postgres vector format string
			vecStr := fmt.Sprintf("%v", vec)
			insertQuery = `
				INSERT INTO autodream_memories (content, embedding, source_mission_id, consolidated_at)
				VALUES ($1, $2, $3, NOW())
			`
			_, _ = qm.db.Exec(bgCtx, insertQuery, summary, vecStr, taskID)
		}
	}()

	return nil
}
