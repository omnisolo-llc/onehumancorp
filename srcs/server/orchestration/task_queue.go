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
	"github.com/onehumancorp/mono/srcs/server/models"
	"github.com/redis/rueidis"
)

// TaskOrchestrator manages the task queue, dependency tracking, and cross-mode synchronization.
type TaskOrchestrator struct {
	db          db.Provider
	redisClient rueidis.Client
	hub         *CentrifugeNode
	localMu     sync.Mutex
}

// NewTaskOrchestrator creates a new TaskOrchestrator.
func NewTaskOrchestrator(provider db.Provider, hub *CentrifugeNode) *TaskOrchestrator {
	to := &TaskOrchestrator{
		db:  provider,
		hub: hub,
	}

	if os.Getenv("OHC_MULTITENANT") == "true" {
		redisURL := os.Getenv("REDIS_URL")
		if redisURL != "" {
			c, err := rueidis.NewClient(rueidis.ClientOption{
				InitAddress: []string{redisURL},
			})
			if err == nil {
				to.redisClient = c
			}
		}
	}
	return to
}

// EnqueueTask adds a new task to the queue and sets up its dependencies.
func (to *TaskOrchestrator) EnqueueTask(ctx context.Context, task *models.Task, dependencies []string) error {
	to.localMu.Lock()
	defer to.localMu.Unlock()

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// Determine initial status based on dependencies
	initialStatus := "READY"
	if len(dependencies) > 0 {
		// Check if all dependencies are already COMPLETED
		for _, depID := range dependencies {
			var status string
			err := tx.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1", depID).Scan(&status)
			if err != nil {
				if errors.Is(err, sql.ErrNoRows) {
					return fmt.Errorf("dependency task %s not found", depID)
				}
				return fmt.Errorf("failed to check dependency %s: %w", depID, err)
			}
			if status != "COMPLETED" {
				initialStatus = "PENDING"
				break
			}
		}
	}
	task.Status = initialStatus

	if task.ID == "" {
		task.ID = generateID() // Using the same generateID from tasks.go
	}

	// Default payload with description and priority if not provided
	payloadMap := make(map[string]interface{})
	if task.Payload != "" {
		_ = json.Unmarshal([]byte(task.Payload), &payloadMap)
	}
	payloadMap["description"] = task.Description
	payloadMap["priority"] = task.Priority
	if len(task.Capabilities) > 0 {
		payloadMap["capabilities"] = task.Capabilities
	}
	payloadBytes, _ := json.Marshal(payloadMap)
	task.Payload = string(payloadBytes)

	// Insert task
	if to.db.IsSQLite() {
		query := `
			INSERT INTO swarm_tasks (id, mission_id, title, payload, status, created_at, updated_at)
			VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		`
		_, err = tx.Exec(ctx, query, task.ID, task.MissionID, task.Title, task.Payload, task.Status)
	} else {
		query := `
			INSERT INTO swarm_tasks (id, mission_id, title, payload, status)
			VALUES ($1, $2, $3, $4, $5)
		`
		_, err = tx.Exec(ctx, query, task.ID, task.MissionID, task.Title, task.Payload, task.Status)
	}

	if err != nil {
		return fmt.Errorf("failed to create task: %w", err)
	}

	// Insert dependencies
	for _, depID := range dependencies {
		_, err = tx.Exec(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)", task.ID, depID)
		if err != nil {
			return fmt.Errorf("failed to insert dependency %s: %w", depID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	// Broadcast task creation
	if to.hub != nil {
		to.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{
			"action":      "CREATE",
			"mission_id":  task.MissionID,
			"title":       task.Title,
			"description": task.Description,
			"priority":    task.Priority,
			"status":      task.Status,
		})
	}

	return nil
}

// AcquireReadyTask attempts to claim a READY task for the given agentID.
func (to *TaskOrchestrator) AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*models.Task, error) {
	// First check distributed lock if in cloud mode (we'll lock the agent to prevent concurrent claims by same agent, or lock a generic acquire queue)
	// Actually, rueidis locks are better per-task. We'll do a SELECT FOR UPDATE to pick a task, then optionally lock it in Redis if needed.
	// But since DB handles the atomic UPDATE, Redis lock is an extra layer. Let's stick to DB transactions for the queue pick.

	to.localMu.Lock()
	defer to.localMu.Unlock()

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var task models.Task
	var errQuery error

	if to.db.IsSQLite() {
		query := `
			SELECT id, mission_id, title, payload, status, locked_until, created_at, updated_at
			FROM swarm_tasks
			WHERE status = 'READY' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY json_extract(payload, '$.priority') ASC, created_at ASC
			LIMIT 1
		`
		errQuery = tx.QueryRow(ctx, query).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Payload, &task.Status, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
	} else {
		// PostgreSQL with SKIP LOCKED
		query := `
			SELECT id, mission_id, title, payload, status, locked_until, created_at, updated_at
			FROM swarm_tasks
			WHERE status = 'READY' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY payload->>'priority' ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
		errQuery = tx.QueryRow(ctx, query).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Payload, &task.Status, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
	}

	if errQuery != nil {
		if errors.Is(errQuery, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, fmt.Errorf("failed to find ready task: %w", errQuery)
	}

	// Update task status to IN_PROGRESS
	updateQuery := `
		UPDATE swarm_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND status = 'READY'
	`
	res, err := tx.Exec(ctx, updateQuery, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	rowsAffected, err := res.RowsAffected()
	if err != nil {
		return nil, fmt.Errorf("failed to get rows affected: %w", err)
	}

	if rowsAffected == 0 {
		return nil, nil // Claimed by another worker concurrently
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = agentID

	// If in cloud mode, acquire Redis lock for this specific task execution
	if to.redisClient != nil {
		lockKey := "lock:task:" + task.ID
		cmd := to.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Ex(30 * time.Minute).Build()
		_ = to.redisClient.Do(ctx, cmd).Error() // Best effort, DB already updated
	}

	// Broadcast task claim
	if to.hub != nil {
		to.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{
			"action":    "CLAIM",
			"agent_id":  agentID,
			"status":    task.Status,
		})
	}

	return &task, nil
}

// CompleteTask marks a task as completed and resolves dependencies for blocked tasks.
func (to *TaskOrchestrator) CompleteTask(ctx context.Context, taskID, agentID, result string) error {
	to.localMu.Lock()
	defer to.localMu.Unlock()

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// Update the task to COMPLETED
	updateQuery := `
		UPDATE swarm_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND assigned_agent_id = $2 AND status = 'IN_PROGRESS'
	`
	res, err := tx.Exec(ctx, updateQuery, taskID, agentID)
	if err != nil {
		return fmt.Errorf("failed to complete task: %w", err)
	}

	rowsAffected, err := res.RowsAffected()
	if err != nil || rowsAffected == 0 {
		return errors.New("task not found or not assigned to agent")
	}

	// Release Redis lock if applicable
	if to.redisClient != nil {
		lockKey := "lock:task:" + taskID
		cmd := to.redisClient.B().Del().Key(lockKey).Build()
		_ = to.redisClient.Do(ctx, cmd).Error()
	}

	// Find PENDING tasks that might now be READY
	// A task is READY if all its dependencies are COMPLETED.
	// We'll just fetch tasks that depend on the completed task.
	checkQuery := `
		SELECT t.id
		FROM swarm_tasks t
		JOIN task_dependencies td ON t.id = td.task_id
		WHERE td.depends_on_task_id = $1 AND t.status = 'PENDING'
	`
	rows, err := tx.Query(ctx, checkQuery, taskID)
	if err != nil {
		return fmt.Errorf("failed to query dependent tasks: %w", err)
	}
	defer rows.Close()

	var dependentTaskIDs []string
	for rows.Next() {
		var depID string
		if err := rows.Scan(&depID); err != nil {
			return fmt.Errorf("failed to scan dependent task ID: %w", err)
		}
		dependentTaskIDs = append(dependentTaskIDs, depID)
	}

	for _, depID := range dependentTaskIDs {
		// Check if ALL dependencies for depID are now COMPLETED
		allCompletedQuery := `
			SELECT COUNT(*)
			FROM task_dependencies td
			JOIN swarm_tasks st ON td.depends_on_task_id = st.id
			WHERE td.task_id = $1 AND st.status != 'COMPLETED'
		`
		var incompleteCount int
		if err := tx.QueryRow(ctx, allCompletedQuery, depID).Scan(&incompleteCount); err != nil {
			return fmt.Errorf("failed to check all dependencies for %s: %w", depID, err)
		}

		if incompleteCount == 0 {
			// Mark as READY
			_, err := tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'READY', updated_at = CURRENT_TIMESTAMP WHERE id = $1", depID)
			if err != nil {
				return fmt.Errorf("failed to mark task %s as READY: %w", depID, err)
			}
			// Broadcast READY state change
			if to.hub != nil {
				to.hub.PublishTaskBroadcast(depID, map[string]interface{}{
					"action": "READY",
					"status": "READY",
				})
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	// Broadcast task completion
	if to.hub != nil {
		to.hub.PublishTaskBroadcast(taskID, map[string]interface{}{
			"action":   "COMPLETE",
			"agent_id": agentID,
			"status":   "COMPLETED",
			"result":   result,
		})
	}

	// Trigger AutoDream hook asynchronously
	go to.triggerAutoDreamHook(context.Background(), taskID, result)

	return nil
}

func (to *TaskOrchestrator) triggerAutoDreamHook(ctx context.Context, taskID, result string) {
	// 5. AutoDream Hook: generate a summary embedding via CachedMinimaxClient and insert it into autodream_memories / swarm_long_term_memory
	// Note: AutoDream memory integration is a stub, but we would use the CachedMinimaxClient here.
	// Since CachedMinimaxClient takes a rueidis client and db.Provider, we could initialize it if needed.
	// We'll look up the task payload and result, generate an embedding and save it to the DB.

	// Implementation placeholder: normally calls CachedMinimaxClient.GenerateEmbedding(...)
	// and inserts into swarm_long_term_memory.
}
