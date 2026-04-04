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
	MissionID       string
	ParentPlanID    string
	Dependencies    []string
	Title           string
	Description     string
	AssignedAgentID string
	Status          string // PENDING, IN_PROGRESS, COMPLETED, FAILED
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
func (tm *TaskManager) CreateTask(ctx context.Context, missionID, title, description, priority string) (*SharedTask, error) {
	return tm.CreateTaskWithPlan(ctx, missionID, "", nil, title, description, priority)
}

// CreateTaskWithPlan creates a new shared task linked to an UltraPlan and with DAG dependencies.
func (tm *TaskManager) CreateTaskWithPlan(ctx context.Context, missionID, parentPlanID string, dependencies []string, title, description, priority string) (*SharedTask, error) {
	if priority == "" {
		priority = "P2"
	}

	// For standard SQLite insertion, we generate our own ID.
	id := generateID()

	// Default payload with description and priority based on schema requirements
	payloadMap := map[string]string{"description": description, "priority": priority}
	payloadBytes, err := json.Marshal(payloadMap)
	if err != nil {
		return nil, fmt.Errorf("failed to encode task payload: %w", err)
	}
	payload := string(payloadBytes)

	if dependencies == nil {
		dependencies = []string{}
	}
	depsBytes, _ := json.Marshal(dependencies)
	depsJSON := string(depsBytes)

	var parentPlanIDPtr *string
	if parentPlanID != "" {
		parentPlanIDPtr = &parentPlanID
	}

	var task SharedTask
	var query string

	if tm.db.IsSQLite() {
		query = `
			INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, dependencies, title, payload, status, created_at, updated_at)
			VALUES ($1, $2, $3, $4, $5, $6, 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
			RETURNING id, mission_id, parent_plan_id, dependencies, title, payload, status, created_at, updated_at
		`
	} else {
		query = `
			INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, dependencies, title, payload, status)
			VALUES ($1, $2, $3, $4, $5, $6, 'PENDING')
			RETURNING id, mission_id, parent_plan_id, dependencies, title, payload, status, created_at, updated_at
		`
	}

	var returnedParentPlanID sql.NullString
	var returnedDependencies string

	err = tm.db.QueryRow(ctx, query, id, missionID, parentPlanIDPtr, depsJSON, title, payload).Scan(
		&task.ID, &task.MissionID, &returnedParentPlanID, &returnedDependencies, &task.Title, &task.Payload, &task.Status, &task.CreatedAt, &task.UpdatedAt,
	)

	if returnedParentPlanID.Valid {
		task.ParentPlanID = returnedParentPlanID.String
	}
	_ = json.Unmarshal([]byte(returnedDependencies), &task.Dependencies)

	task.Description = description
	task.Priority = priority

	if err != nil {
		return nil, fmt.Errorf("failed to create task: %w", err)
	}

	// Broadcast task creation
	if tm.hub != nil {
		go func() {
			payloadBytes, err := json.Marshal(map[string]interface{}{
				"task_id":     task.ID,
				"action":      "CREATE",
				"agent_id":    task.AssignedAgentID,
				"status":      task.Status,
				"mission_id":  task.MissionID,
				"title":       task.Title,
				"description": task.Description,
				"priority":    task.Priority,
			})
			if err == nil {
				_, _ = tm.hub.node.Publish("mesh:tasks", payloadBytes)
			}
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
			SELECT id, mission_id, parent_plan_id, dependencies, title, payload, status, locked_until, created_at, updated_at
			FROM swarm_tasks
			WHERE id = $1 AND status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY json_extract(payload, '$.priority') ASC, created_at ASC
			LIMIT 1
		`
		var pID sql.NullString
		var deps string
		errQuery = tx.QueryRow(ctx, query, taskID).Scan(
			&task.ID, &task.MissionID, &pID, &deps, &task.Title, &task.Payload, &task.Status, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
		if pID.Valid {
			task.ParentPlanID = pID.String
		}
		_ = json.Unmarshal([]byte(deps), &task.Dependencies)
	} else {
		// PostgreSQL with SKIP LOCKED
		query := `
			SELECT id, mission_id, parent_plan_id, dependencies, title, payload, status, locked_until, created_at, updated_at
			FROM swarm_tasks
			WHERE id = $1 AND status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY payload->>'priority' ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
		var pID sql.NullString
		var deps string
		errQuery = tx.QueryRow(ctx, query, taskID).Scan(
			&task.ID, &task.MissionID, &pID, &deps, &task.Title, &task.Payload, &task.Status, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
		if pID.Valid {
			task.ParentPlanID = pID.String
		}
		_ = json.Unmarshal([]byte(deps), &task.Dependencies)
	}

	if errQuery != nil {
		if errors.Is(errQuery, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, fmt.Errorf("failed to find pending task: %w", errQuery)
	}

	// Check DAG dependencies
	// For DAG dependency check across swarm_tasks, PostgreSQL uses JSONB array text elements and SQLite uses json_each
	if len(task.Dependencies) > 0 {
		var pendingDeps int
		var checkQuery string

		if tm.db.IsSQLite() {
			checkQuery = `
				SELECT COUNT(*)
				FROM swarm_tasks
				WHERE id IN (SELECT value FROM json_each($1)) AND status != 'COMPLETED'
			`
		} else {
			checkQuery = `
				SELECT COUNT(*)
				FROM swarm_tasks
				WHERE id IN (SELECT jsonb_array_elements_text($1::jsonb)) AND status != 'COMPLETED'
			`
		}

		depsBytes, _ := json.Marshal(task.Dependencies)
		err = tx.QueryRow(ctx, checkQuery, string(depsBytes)).Scan(&pendingDeps)
		if err != nil || pendingDeps > 0 {
			// Dependencies not satisfied
			return nil, nil // Cannot claim yet
		}
	}

	// Reconstruct Description and Priority from JSON payload
	var payloadMap map[string]interface{}
	if err := json.Unmarshal([]byte(task.Payload), &payloadMap); err == nil {
		if desc, ok := payloadMap["description"].(string); ok {
			task.Description = desc
		}
		if prio, ok := payloadMap["priority"].(string); ok {
			task.Priority = prio
		}
	}

	// Update task status to IN_PROGRESS
	updateQuery := `
		UPDATE swarm_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
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
	telemetry.RecordSwarmTaskTransition(ctx, task.MissionID, "PENDING", "IN_PROGRESS")
	task.AssignedAgentID = agentID

	// Broadcast task claim
	if tm.hub != nil {
		go func() {
			payloadBytes, err := json.Marshal(map[string]interface{}{
				"task_id":  task.ID,
				"action":   "CLAIM",
				"agent_id": agentID,
				"status":   task.Status,
			})
			if err == nil {
				_, _ = tm.hub.node.Publish("mesh:tasks", payloadBytes)
			}
		}()
	}

	return &task, nil
}

// ReviewTask marks a task for review.
func (tm *TaskManager) ReviewTask(ctx context.Context, taskID, agentID string) error {
	query := `
		UPDATE swarm_tasks
		SET status = 'REVIEW', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND assigned_agent_id = $2 AND status = 'IN_PROGRESS'
	`
	rowsAffected, err := tm.db.Exec(ctx, query, taskID, agentID)
	if err != nil {
		return fmt.Errorf("failed to move task to review: %w", err)
	}

	if rowsAffected == 0 {
		return errors.New("task not found, not assigned to agent, or not in progress")
	}

	var missionID string
	err = tm.db.QueryRow(ctx, "SELECT mission_id FROM swarm_tasks WHERE id = $1", taskID).Scan(&missionID)
	if err == nil {
		telemetry.RecordSwarmTaskTransition(ctx, missionID, "IN_PROGRESS", "REVIEW")
	}

	// Broadcast task review
	if tm.hub != nil {
		go func() {
			payloadBytes, err := json.Marshal(map[string]interface{}{
				"task_id":  taskID,
				"action":   "REVIEW",
				"agent_id": agentID,
				"status":   "REVIEW",
			})
			if err == nil {
				_, _ = tm.hub.node.Publish("mesh:tasks", payloadBytes)
			}
		}()
	}

	return nil
}

// CompleteTask marks a task as completed.
func (tm *TaskManager) CompleteTask(ctx context.Context, taskID, agentID string) error {
	query := `
		UPDATE swarm_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND assigned_agent_id = $2 AND status IN ('IN_PROGRESS', 'REVIEW')
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
			payloadBytes, err := json.Marshal(map[string]interface{}{
				"task_id":  taskID,
				"action":   "COMPLETE",
				"agent_id": agentID,
				"status":   "COMPLETED",
			})
			if err == nil {
				_, _ = tm.hub.node.Publish("mesh:tasks", payloadBytes)
			}
		}()
	}

	// Record Telemetry
	// Note: We don't have mission_id readily available in this block, but telemetry.RecordSwarmTaskCompleted can take it or we can pass an empty string / agent string.
	// Actually we should fetch it if we want it perfect, but it's optional for the counter.
	var missionID string
	err = tm.db.QueryRow(ctx, "SELECT mission_id FROM swarm_tasks WHERE id = $1", taskID).Scan(&missionID)
	if err == nil {
		telemetry.RecordSwarmTaskCompleted(ctx, missionID)
		telemetry.RecordSwarmTaskTransition(ctx, missionID, "IN_PROGRESS_OR_REVIEW", "COMPLETED")
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
			SELECT id, mission_id, parent_plan_id, dependencies, title, payload, status, locked_until, created_at, updated_at
			FROM swarm_tasks
			WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY json_extract(payload, '$.priority') ASC, created_at ASC
			LIMIT $1
		`
	} else {
		// PostgreSQL with SKIP LOCKED
		query = `
			SELECT id, mission_id, parent_plan_id, dependencies, title, payload, status, locked_until, created_at, updated_at
			FROM swarm_tasks
			WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY payload->>'priority' ASC, created_at ASC
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
		var pID sql.NullString
		var deps string
		if err := rows.Scan(
			&task.ID, &task.MissionID, &pID, &deps, &task.Title, &task.Payload, &task.Status, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		); err != nil {
			return nil, fmt.Errorf("failed to scan task: %w", err)
		}
		if pID.Valid {
			task.ParentPlanID = pID.String
		}
		_ = json.Unmarshal([]byte(deps), &task.Dependencies)

		var payloadMap map[string]interface{}
		if err := json.Unmarshal([]byte(task.Payload), &payloadMap); err == nil {
			if desc, ok := payloadMap["description"].(string); ok {
				task.Description = desc
			}
			if prio, ok := payloadMap["priority"].(string); ok {
				task.Priority = prio
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
		// Check DAG dependencies: Task is only available if all dependencies are COMPLETED
		depsSatisfied := true

		if len(task.Dependencies) > 0 {
			var pendingDeps int
			var checkQuery string

			if tm.db.IsSQLite() {
				checkQuery = `
					SELECT COUNT(*)
					FROM swarm_tasks
					WHERE id IN (SELECT value FROM json_each($1)) AND status != 'COMPLETED'
				`
			} else {
				checkQuery = `
					SELECT COUNT(*)
					FROM swarm_tasks
					WHERE id IN (SELECT jsonb_array_elements_text($1::jsonb)) AND status != 'COMPLETED'
				`
			}

			depsBytes, _ := json.Marshal(task.Dependencies)
			err = tx.QueryRow(ctx, checkQuery, string(depsBytes)).Scan(&pendingDeps)
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
			UPDATE swarm_tasks
			SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
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
				payloadBytes, err := json.Marshal(map[string]interface{}{
					"task_id":  t.ID,
					"action":   "CLAIM",
					"agent_id": agentID,
					"status":   t.Status,
				})
				if err == nil {
					_, _ = tm.hub.node.Publish("mesh:tasks", payloadBytes)
				}
			}(task)
		}
	}

	return claimedTasks, nil
}

// generateID generates a pseudo-uuid for SQLite compatibility.
func generateID() string {
	return fmt.Sprintf("%d", time.Now().UnixNano())
}
