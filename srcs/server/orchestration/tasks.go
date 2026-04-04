package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"os"
	"time"

	"encoding/json"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// SharedTask represents a shared task distributed across agents.
type SharedTask struct {
	ID              string
	OrganizationID  string
	Dependencies    []string
	Title           string
	Description     string
	AssignedAgentID string
	Status          string // PENDING, IN_PROGRESS, COMPLETED, FAILED, BLOCKED
	Priority        string
	Payload         string
	LockedUntil     sql.NullTime
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

// TaskManager manages the shared tasks list
type TaskManager struct {
	db          db.Provider
	redisClient rueidis.Client
	hub         *CentrifugeNode // For Teammate Mesh broadcast
	stopChan    chan struct{}
}

// NewTaskManager creates a new TaskManager.
func NewTaskManager(provider db.Provider, hub *CentrifugeNode) *TaskManager {
	tm := &TaskManager{
		db:  provider,
		hub: hub,
	}

	if os.Getenv("OHC_MULTITENANT") == "true" {
		redisURL := os.Getenv("REDIS_URL")
		if redisURL != "" {
			opts, err := rueidis.ParseURL(redisURL)
			if err == nil {
				c, err := rueidis.NewClient(opts)
				if err == nil {
					tm.redisClient = c
				}
			}
		}
	}
	tm.stopChan = make(chan struct{})
	return tm
}

// StartWorkerLoop periodically checks for satisfied PENDING tasks and ensures they are broadcasted
// to unblock awaiting agents, properly handling DAG dependency checks across swarm_tasks.
func (tm *TaskManager) StartWorkerLoop(ctx context.Context) {
	ticker := time.NewTicker(2 * time.Second)
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-tm.stopChan:
				return
			case <-ticker.C:
				tm.evaluatePendingDependencies(ctx)
			}
		}
	}()
}

// StopWorkerLoop stops the task manager background loop
func (tm *TaskManager) StopWorkerLoop() {
	close(tm.stopChan)
}

// evaluatePendingDependencies finds tasks whose dependencies have just been met and broadcasts them.
func (tm *TaskManager) evaluatePendingDependencies(ctx context.Context) {
	// A simple check to find PENDING tasks without active locks and met dependencies
	// and trigger a broadcast to awake idle agents.
	tasks, err := tm.PollTasks(ctx, "system-orchestrator", 0) // Polling with 0 limit acts as a peek if implemented, or we can just run a custom query.
	if err != nil {
		return
	}
	_ = tasks // Ignore if using PollTasks, but let's implement a real check

	var query string
	if tm.db.IsSQLite() {
		query = `
			SELECT id, dependencies, status FROM swarm_tasks WHERE status = 'PENDING'
		`
	} else {
		query = `
			SELECT id, dependencies, status FROM swarm_tasks WHERE status = 'PENDING'
		`
	}
	rows, err := tm.db.Query(ctx, query)
	if err != nil {
		return
	}
	defer rows.Close()

	for rows.Next() {
		var id, deps, status string
		if err := rows.Scan(&id, &deps, &status); err == nil {
			// For each task, check dependencies
			var depList []string
			if err := json.Unmarshal([]byte(deps), &depList); err == nil && len(depList) > 0 {
				allMet := true
				for _, depID := range depList {
					var depStatus string
					err := tm.db.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1", depID).Scan(&depStatus)
					if err != nil || depStatus != "COMPLETED" {
						allMet = false
						break
					}
				}
				if allMet && tm.hub != nil {
					// Broadcast that task is now ready
					go func(taskID string) {
						tm.hub.PublishTaskBroadcast(taskID, map[string]interface{}{
							"action":   "READY",
							"agent_id": "",
							"status":   "PENDING",
						})
					}(id)
				}
			}
		}
	}
}

// SetHub injects the CentrifugeNode dependency into the TaskManager.
func (tm *TaskManager) SetHub(hub *CentrifugeNode) {
	tm.hub = hub
}

// CreateTask creates a new shared task.
// CreateTask creates a new pending task.
func (tm *TaskManager) CreateTask(ctx context.Context, organizationID, title, description, priority string) (*SharedTask, error) {
	return tm.CreateTaskWithPlan(ctx, organizationID, nil, title, description, priority)
}

// CreateTaskWithPlan creates a new pending task with plan association.
func (tm *TaskManager) CreateTaskWithPlan(ctx context.Context, organizationID string, dependencies []string, title, description, priority string) (*SharedTask, error) {
	var task SharedTask

	id := generateID()

	payloadMap := map[string]interface{}{
		"description": description,
	}
	payloadBytes, _ := json.Marshal(payloadMap)
	payload := string(payloadBytes)

	if priority == "" {
		priority = "P2"
	}

	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	if tm.db.IsSQLite() {
		query = `
			INSERT INTO shared_tasks (id, organization_id, title, description, payload, status, priority)
			VALUES ($1, $2, $3, $4, $5, 'PENDING', $6)
			RETURNING id, organization_id, title, payload, status, priority, created_at, updated_at
		`
	} else {
		query = `
			INSERT INTO shared_tasks (id, organization_id, title, description, payload, status, priority)
			VALUES ($1, $2, $3, $4, $5, 'PENDING', $6)
			RETURNING id, organization_id, title, payload, status, priority, created_at, updated_at
		`
	}

	err = tx.QueryRow(ctx, query, id, organizationID, title, description, payload, priority).Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.CreatedAt, &task.UpdatedAt,
	)

	if err != nil {
		return nil, fmt.Errorf("failed to create task: %w", err)
	}

	task.Description = description
	task.Priority = priority

	for _, dep := range dependencies {
		_, err = tx.Exec(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)", task.ID, dep)
		if err != nil {
			return nil, fmt.Errorf("failed to insert dependency: %w", err)
		}
		task.Dependencies = append(task.Dependencies, dep)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	// Broadcast task creation
	if tm.hub != nil {
		go func() {
			payload := map[string]interface{}{
				"task_id":     task.ID,
				"action":      "CREATE",
				"agent_id":    task.AssignedAgentID,
				"status":      task.Status,
				"organization_id": task.OrganizationID,
				"title":       task.Title,
				"description": task.Description,
				"priority":    task.Priority,
			}
			tm.hub.PublishTaskBroadcast(task.ID, payload)
		}()
	}

	return &task, nil
}

// ClaimTask attempts to claim a specific PENDING task for the given agentID.
// It uses row-level locking (FOR UPDATE) in Postgres, and relies on SQLite's lock mechanism
// to prevent race conditions.
// In Multi-tenant cloud mode, it attempts to acquire a distributed Redis lock.
func (tm *TaskManager) ClaimTask(ctx context.Context, taskID, agentID string) (*SharedTask, error) {
	if tm.redisClient != nil {
		// Acquire Redis-backed distributed lock with 30s TTL
		lockKey := "lock:task:" + taskID
		cmd := tm.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Ex(30 * time.Second).Build()
		err := tm.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return nil, nil // Lock could not be acquired (task is locked)
			}
			return nil, fmt.Errorf("failed to acquire distributed lock: %w", err)
		}
	}

	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var task SharedTask
	var errQuery error

	if tm.db.IsSQLite() {
		// SQLite doesn't support FOR UPDATE, but `Begin` handles concurrent writes lock.
		query := `
			SELECT id, organization_id, title, payload, status, priority, locked_until, created_at, updated_at
			FROM shared_tasks
			WHERE id = $1 AND status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
		`
		errQuery = tx.QueryRow(ctx, query, taskID).Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
	} else {
		// PostgreSQL with SKIP LOCKED
		query := `
			SELECT id, organization_id, title, payload, status, priority, locked_until, created_at, updated_at
			FROM shared_tasks
			WHERE id = $1 AND status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
		errQuery = tx.QueryRow(ctx, query, taskID).Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
	}

	if errQuery != nil {
		if errors.Is(errQuery, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, fmt.Errorf("failed to find pending task: %w", errQuery)
	}

	// Fetch dependencies from task_dependencies table
	depQuery := `SELECT depends_on_task_id FROM task_dependencies WHERE task_id = $1`
	depRows, err := tx.Query(ctx, depQuery, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to get dependencies: %w", err)
	}
	defer depRows.Close()

	for depRows.Next() {
		var depID string
		if err := depRows.Scan(&depID); err == nil {
			task.Dependencies = append(task.Dependencies, depID)
		}
	}

	// Check DAG dependencies
	if len(task.Dependencies) > 0 {
		var pendingDeps int
		checkQuery := `
			SELECT COUNT(*)
			FROM shared_tasks
			WHERE id IN (SELECT depends_on_task_id FROM task_dependencies WHERE task_id = $1) AND status != 'COMPLETED'
		`
		err = tx.QueryRow(ctx, checkQuery, task.ID).Scan(&pendingDeps)
		if err != nil || pendingDeps > 0 {
			// Dependencies not satisfied
			return nil, nil // Cannot claim yet
		}
	}

	// Reconstruct Description from JSON payload
	var payloadMap map[string]interface{}
	if err := json.Unmarshal([]byte(task.Payload), &payloadMap); err == nil {
		if desc, ok := payloadMap["description"].(string); ok {
			task.Description = desc
		}
	}

	// Update task status to IN_PROGRESS
	updateQuery := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND status = 'PENDING'
	`
	rowsAffected, err := tx.Exec(ctx, updateQuery, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if rowsAffected == 0 {
		// Task was likely claimed by another worker concurrently.
		return nil, nil
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "IN_PROGRESS"
	telemetry.RecordSwarmTaskTransition(ctx, task.OrganizationID, "PENDING", "IN_PROGRESS")
	task.AssignedAgentID = agentID

	// Broadcast task claim
	if tm.hub != nil {
		go func() {
			payload := map[string]interface{}{
				"task_id":  task.ID,
				"action":   "CLAIM",
				"agent_id": agentID,
				"status":   task.Status,
			}
			tm.hub.PublishTaskBroadcast(task.ID, payload)
		}()
	}

	return &task, nil
}

// ReviewTask marks a task for review.
func (tm *TaskManager) ReviewTask(ctx context.Context, taskID, agentID string) error {
	query := `
		UPDATE shared_tasks
		SET status = 'REVIEW', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND agent_id = $2 AND status = 'IN_PROGRESS'
	`
	rowsAffected, err := tm.db.Exec(ctx, query, taskID, agentID)
	if err != nil {
		return fmt.Errorf("failed to move task to review: %w", err)
	}

	if rowsAffected == 0 {
		return errors.New("task not found, not assigned to agent, or not in progress")
	}

	var orgID string
	err = tm.db.QueryRow(ctx, "SELECT organization_id FROM shared_tasks WHERE id = $1", taskID).Scan(&orgID)
	if err == nil {
		telemetry.RecordSwarmTaskTransition(ctx, orgID, "IN_PROGRESS", "REVIEW")
	}

	// Broadcast task review
	if tm.hub != nil {
		go func() {
			payload := map[string]interface{}{
				"task_id":  taskID,
				"action":   "REVIEW",
				"agent_id": agentID,
				"status":   "REVIEW",
			}
			tm.hub.PublishTaskBroadcast(taskID, payload)
		}()
	}

	return nil
}

// CompleteTask marks a task as completed.
func (tm *TaskManager) CompleteTask(ctx context.Context, taskID, agentID string) error {
	query := `
		UPDATE shared_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND agent_id = $2 AND status IN ('IN_PROGRESS', 'REVIEW')
	`
	rowsAffected, err := tm.db.Exec(ctx, query, taskID, agentID)
	if err != nil {
		return fmt.Errorf("failed to complete task: %w", err)
	}

	if rowsAffected == 0 {
		return errors.New("task not found or not assigned to agent")
	}

	// Broadcast task completion
	if tm.hub != nil {
		go func() {
			payload := map[string]interface{}{
				"task_id":  taskID,
				"action":   "COMPLETE",
				"agent_id": agentID,
				"status":   "COMPLETED",
			}
			tm.hub.PublishTaskBroadcast(taskID, payload)
		}()
	}

	return nil
}

// PollTasks attempts to claim up to `limit` PENDING tasks for the given agentID.
// It uses row-level locking (FOR UPDATE SKIP LOCKED) in Postgres, or relies on
// SQLite's concurrent writes lock for safe queue picking.
func (tm *TaskManager) PollTasks(ctx context.Context, agentID string, limit int) ([]*SharedTask, error) {
	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	// Fetch slightly more tasks initially in case some are filtered out by dependency checks
	fetchLimit := limit * 3
	if tm.db.IsSQLite() {
		query = `
			SELECT id, organization_id, title, payload, status, priority, locked_until, created_at, updated_at
			FROM shared_tasks
			WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY priority ASC, created_at ASC
			LIMIT $1
		`
	} else {
		// PostgreSQL with SKIP LOCKED
		query = `
			SELECT id, organization_id, title, payload, status, priority, locked_until, created_at, updated_at
			FROM shared_tasks
			WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY priority ASC, created_at ASC
			LIMIT $1
			FOR UPDATE SKIP LOCKED
		`
	}

	rows, err := tx.Query(ctx, query, fetchLimit)
	if err != nil {
		return nil, fmt.Errorf("failed to query tasks: %w", err)
	}
	defer rows.Close()

	var candidateTasks []*SharedTask

	for rows.Next() {
		task := &SharedTask{}
		if err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		); err != nil {
			return nil, fmt.Errorf("failed to scan task: %w", err)
		}

		var payloadMap map[string]interface{}
		if err := json.Unmarshal([]byte(task.Payload), &payloadMap); err == nil {
			if desc, ok := payloadMap["description"].(string); ok {
				task.Description = desc
			}
		}

		candidateTasks = append(candidateTasks, task)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("row iteration error: %w", err)
	}

	var tasks []*SharedTask
	var taskIDs []string

	for _, task := range candidateTasks {
		// Fetch dependencies from task_dependencies table
		depQuery := `SELECT depends_on_task_id FROM task_dependencies WHERE task_id = $1`
		depRows, err := tx.Query(ctx, depQuery, task.ID)
		if err != nil {
			return nil, fmt.Errorf("failed to get dependencies: %w", err)
		}

		for depRows.Next() {
			var depID string
			if err := depRows.Scan(&depID); err == nil {
				task.Dependencies = append(task.Dependencies, depID)
			}
		}
		depRows.Close()

		// Check DAG dependencies: Task is only available if all dependencies are COMPLETED
		depsSatisfied := true

		if len(task.Dependencies) > 0 {
			var pendingDeps int
			checkQuery := `
				SELECT COUNT(*)
				FROM shared_tasks
				WHERE id IN (SELECT depends_on_task_id FROM task_dependencies WHERE task_id = $1) AND status != 'COMPLETED'
			`
			err = tx.QueryRow(ctx, checkQuery, task.ID).Scan(&pendingDeps)
			if err != nil || pendingDeps > 0 {
				depsSatisfied = false
			}
		}

		if depsSatisfied {
			tasks = append(tasks, task)
			taskIDs = append(taskIDs, task.ID)
			if len(tasks) >= limit {
				break
			}
		}
	}

	if len(tasks) == 0 {
		return nil, nil // No tasks to claim
	}

	// Update status for all claimed tasks
	var claimedTasks []*SharedTask

	for _, task := range tasks {
		rowsAffected, err := tx.Exec(ctx, `
			UPDATE shared_tasks
			SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = $2 AND status = 'PENDING'
		`, agentID, task.ID)

		if err != nil {
			return nil, fmt.Errorf("failed to update task %s: %w", task.ID, err)
		}

		if rowsAffected > 0 {
			task.Status = "IN_PROGRESS"
			task.AssignedAgentID = agentID
			claimedTasks = append(claimedTasks, task)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	for _, task := range claimedTasks {
		// Broadcast task claim
		if tm.hub != nil {
			go func(t *SharedTask) {
				payload := map[string]interface{}{
					"task_id":  t.ID,
					"action":   "CLAIM",
					"agent_id": agentID,
					"status":   t.Status,
				}
				tm.hub.PublishTaskBroadcast(t.ID, payload)
			}(task)
		}
	}

	return claimedTasks, nil
}

// generateID generates a pseudo-uuid for SQLite compatibility.
func generateID() string {
	return fmt.Sprintf("%d", time.Now().UnixNano())
}
