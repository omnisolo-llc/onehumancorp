package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
	"github.com/redis/rueidis"
)

// TaskOrchestrator abstracts the state machine and dependency tracking for the Teammate Mesh
type TaskOrchestrator interface {
	EnqueueTask(ctx context.Context, task *models.Task, dependsOn []string) (*models.Task, error)
	AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*models.Task, error)
	CompleteTask(ctx context.Context, taskID string, agentID string, result string) error
}

type DefaultTaskOrchestrator struct {
	db          db.Provider
	redisClient rueidis.Client
	hub         *CentrifugeNode
	mesh        TeammateMesh
	mu          sync.Mutex // For standalone mode coordination
}

func NewTaskOrchestrator(provider db.Provider, redisClient rueidis.Client, hub *CentrifugeNode, mesh TeammateMesh) TaskOrchestrator {
	return &DefaultTaskOrchestrator{
		db:          provider,
		redisClient: redisClient,
		hub:         hub,
		mesh:        mesh,
	}
}

func (to *DefaultTaskOrchestrator) EnqueueTask(ctx context.Context, task *models.Task, dependsOn []string) (*models.Task, error) {
	// If in standalone, we use mutex, else distributed lock logic implicitly via transactions
	if to.redisClient == nil {
		to.mu.Lock()
		defer to.mu.Unlock()
	}

	id := task.ID
	if id == "" {
		id = generateID()
		task.ID = id
	}

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	// Determine initial status based on dependencies
	status := "PENDING"
	if len(dependsOn) == 0 {
		status = "READY"
	}
	task.Status = status

	payloadBytes, err := json.Marshal(map[string]interface{}{
		"description": task.Description,
		"priority":    task.Priority,
	})
	if err != nil {
		return nil, err
	}
	payload := string(payloadBytes)

	// Insert task
	query := `
		INSERT INTO swarm_tasks (id, mission_id, title, payload, status, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`
	if to.db.IsSQLite() {
		_, err = tx.Exec(ctx, query, task.ID, task.MissionID, task.Title, payload, task.Status)
	} else {
		_, err = tx.Exec(ctx, query, task.ID, task.MissionID, task.Title, payload, task.Status)
	}
	if err != nil {
		return nil, fmt.Errorf("failed to insert task: %w", err)
	}

	// Insert dependencies
	if len(dependsOn) > 0 {
		for _, depID := range dependsOn {
			_, err = tx.Exec(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)", task.ID, depID)
			if err != nil {
				return nil, fmt.Errorf("failed to insert dependency: %w", err)
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit: %w", err)
	}

	// Broadcast
	if to.mesh != nil {
		_ = to.mesh.BroadcastTask(ctx, Task{
			AgentID: "",
			Action:  "CREATE",
			Status:  task.Status,
			TaskID:  task.ID,
		})
	} else if to.hub != nil {
		to.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{
			"action":     "CREATE",
			"mission_id": task.MissionID,
			"title":      task.Title,
			"status":     task.Status,
		})
	}

	return task, nil
}

func (to *DefaultTaskOrchestrator) AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*models.Task, error) {
	if to.redisClient == nil {
		to.mu.Lock()
		defer to.mu.Unlock()
	}

	// We look for a READY task, update to IN_PROGRESS.
	// In Postgres we use SKIP LOCKED.
	// In SQLite we just rely on standard single connection or the mutex lock we added.

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var task models.Task
	var query string
	if to.db.IsSQLite() {
		query = `
			SELECT id, mission_id, title, status, payload, created_at, updated_at
			FROM swarm_tasks
			WHERE status = 'READY'
			ORDER BY json_extract(payload, '$.priority') ASC, created_at ASC
			LIMIT 1
		`
	} else {
		query = `
			SELECT id, mission_id, title, status, payload, created_at, updated_at
			FROM swarm_tasks
			WHERE status = 'READY'
			ORDER BY payload->>'priority' ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
	}

	err = tx.QueryRow(ctx, query).Scan(
		&task.ID, &task.MissionID, &task.Title, &task.Status, &task.Payload, &task.CreatedAt, &task.UpdatedAt,
	)

	if err != nil {
		// sql.ErrNoRows is fine
		return nil, nil // No task available
	}

	// Unmarshal payload
	var payloadMap map[string]interface{}
	if err := json.Unmarshal([]byte(task.Payload), &payloadMap); err == nil {
		if desc, ok := payloadMap["description"].(string); ok {
			task.Description = desc
		}
		if prio, ok := payloadMap["priority"].(string); ok {
			task.Priority = prio
		}
	}

	// Check if task exists and was successfully queried
	if task.ID == "" {
		return nil, nil
	}

	// Update to IN_PROGRESS
	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = agentID

	_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit: %w", err)
	}

	// Broadcast
	if to.mesh != nil {
		_ = to.mesh.BroadcastTask(ctx, Task{
			AgentID: agentID,
			Action:  "CLAIM",
			Status:  task.Status,
			TaskID:  task.ID,
		})
	} else if to.hub != nil {
		to.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{
			"action":   "CLAIM",
			"agent_id": agentID,
			"status":   task.Status,
		})
	}

	return &task, nil
}

func (to *DefaultTaskOrchestrator) CompleteTask(ctx context.Context, taskID string, agentID string, result string) error {
	if to.redisClient == nil {
		to.mu.Lock()
		defer to.mu.Unlock()
	}

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	// Update status to COMPLETED
	affected, err := tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND assigned_agent_id = $2 AND status = 'IN_PROGRESS'", taskID, agentID)
	if err != nil {
		return fmt.Errorf("failed to complete task: %w", err)
	}

	if affected == 0 {
		return fmt.Errorf("task not found or not assigned to agent")
	}

	// Find dependent tasks that might now be READY
	// A task is READY if all its dependencies are COMPLETED.
	// We do this by checking all tasks that depend on the completed one,
	// and for each, checking if any unresolved dependencies remain.
	query := `
		SELECT t.id
		FROM swarm_tasks t
		JOIN task_dependencies td ON t.id = td.task_id
		WHERE td.depends_on_task_id = $1 AND t.status = 'PENDING'
	`
	rows, err := tx.Query(ctx, query, taskID)
	if err != nil {
		return fmt.Errorf("failed to query dependent tasks: %w", err)
	}

	var pendingTasks []string
	for rows.Next() {
		var id string
		if err := rows.Scan(&id); err == nil {
			pendingTasks = append(pendingTasks, id)
		}
	}
	rows.Close()

	var newReadyTasks []string
	for _, pid := range pendingTasks {
		// Check if all dependencies are completed
		var pendingCount int
		checkQuery := `
			SELECT COUNT(*)
			FROM task_dependencies td
			JOIN swarm_tasks t ON td.depends_on_task_id = t.id
			WHERE td.task_id = $1 AND t.status != 'COMPLETED'
		`
		err = tx.QueryRow(ctx, checkQuery, pid).Scan(&pendingCount)
		if err == nil && pendingCount == 0 {
			// All dependencies completed, mark as READY
			_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'READY', updated_at = CURRENT_TIMESTAMP WHERE id = $1", pid)
			if err == nil {
				newReadyTasks = append(newReadyTasks, pid)
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit: %w", err)
	}

	// Fetch task payload for AutoDream
	var taskPayload string
	_ = to.db.QueryRow(ctx, "SELECT payload FROM swarm_tasks WHERE id = $1", taskID).Scan(&taskPayload)

	// Broadcast
	if to.mesh != nil {
		_ = to.mesh.BroadcastTask(ctx, Task{
			AgentID: agentID,
			Action:  "COMPLETE",
			Status:  "COMPLETED",
			TaskID:  taskID,
		})
		for _, rid := range newReadyTasks {
			_ = to.mesh.BroadcastTask(ctx, Task{
				AgentID: "",
				Action:  "READY",
				Status:  "READY",
				TaskID:  rid,
			})
		}
	} else if to.hub != nil {
		to.hub.PublishTaskBroadcast(taskID, map[string]interface{}{
			"action":   "COMPLETE",
			"agent_id": agentID,
			"status":   "COMPLETED",
		})
		for _, rid := range newReadyTasks {
			to.hub.PublishTaskBroadcast(rid, map[string]interface{}{
				"action": "READY",
				"status": "READY",
			})
		}
	}

	// Trigger AutoDream embedding in background
	go func() {
		bgCtx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
		defer cancel()

		var worker *AutoDreamWorker = NewAutoDreamWorker(to.db)

		// Let AutoDreamWorker handle it via direct LLM call
		contextStr := fmt.Sprintf("Task ID: %s, Result: %s, Initial Payload: %s", taskID, result, taskPayload)

		// In a real environment MINIMAX_API_KEY is set. For tests we mock/skip.
		apiKey := os.Getenv("MINIMAX_API_KEY")
		if apiKey == "" {
			slog.Warn("AutoDream: MINIMAX_API_KEY not set, skipping embedding hook")
			return
		}

		client := NewCachedMinimaxClient(NewMinimaxClient(apiKey), to.db, to.redisClient)
		emb, err := client.GenerateEmbedding(bgCtx, contextStr)
		if err != nil {
			slog.Warn("AutoDream: Failed to generate embedding for completed task", "error", err)
			return
		}

		b, _ := json.Marshal(emb)
		memID := generateID()

		// Instead of directly inserting, use the worker to handle standard DB inserts for AutoDream.
		_ = worker.InjectTruth(bgCtx, memID, contextStr, string(b))

		slog.Info("AutoDream: Consolidated task completion memory", "taskID", taskID)
	}()

	return nil
}
