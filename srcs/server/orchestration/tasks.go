package orchestration

import (
	"context"
	"crypto/rand"
	"database/sql"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"time"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// SharedTask represents a shared task distributed across agents.
type SharedTask struct {
	ID              string
	OrganizationID  string
	ParentPlanID    string
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
	tasks, err := tm.PeekTasks(ctx, 1) // Peek if we have at least one to possibly notify agents
	if err != nil {
		return
	}
	_ = tasks // Ignore if using PeekTasks, but let's implement a real check

	query := `
		SELECT st.id, st.status
		FROM shared_tasks st
		WHERE st.status = 'PENDING'
		AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
	`
	rows, err := tm.db.Query(ctx, query)
	if err != nil {
		return
	}
	defer rows.Close()

	for rows.Next() {
		var id, status string
		if err := rows.Scan(&id, &status); err == nil {
			if tm.hub != nil {
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
			RETURNING id, organization_id, COALESCE(parent_plan_id, ''), title, payload, status, priority, created_at, updated_at
		`
	} else {
		query = `
			INSERT INTO shared_tasks (id, organization_id, title, description, payload, status, priority)
			VALUES ($1, $2, $3, $4, $5, 'PENDING', $6)
			RETURNING id, organization_id, COALESCE(parent_plan_id, ''), title, payload, status, priority, created_at, updated_at
		`
	}

	err = tx.QueryRow(ctx, query, id, organizationID, title, description, payload, priority).Scan(
		&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.CreatedAt, &task.UpdatedAt,
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

	// Record +1 delta to the queue length gauge.
	telemetry.RecordSwarmTaskQueueLength(ctx, 1)

	// Broadcast task creation
	if tm.hub != nil {
		go func() {
			payload := map[string]interface{}{
				"task_id":         task.ID,
				"action":          "CREATE",
				"agent_id":        task.AssignedAgentID,
				"status":          task.Status,
				"organization_id": task.OrganizationID,
				"title":           task.Title,
				"description":     task.Description,
				"priority":        task.Priority,
			}
			tm.hub.PublishTaskBroadcast(task.ID, payload)
		}()
	}

	return &task, nil
}

// ClaimTask attempts to claim a specific PENDING task for the given agentID.
// It uses row-level locking (FOR UPDATE SKIP LOCKED) in Postgres, and relies on SQLite's lock mechanism
// to prevent race conditions.
// In Multi-tenant cloud mode, it attempts to acquire a distributed Redis lock.
func (tm *TaskManager) ClaimTask(ctx context.Context, taskID, agentID string) (*SharedTask, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}
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
		// In SQLite, use UPDATE ... RETURNING to avoid TOCTOU races
		query := `
			UPDATE shared_tasks
			SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT st.id
				FROM shared_tasks st
				WHERE st.id = $2 AND st.organization_id = $3 AND st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
				AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
				ORDER BY st.priority ASC, st.created_at ASC
				LIMIT 1
			)
			RETURNING id, organization_id, COALESCE(parent_plan_id, ''), title, payload, status, priority, locked_until, created_at, updated_at
		`
		errQuery = tx.QueryRow(ctx, query, agentID, taskID, claims.OrganizationID).Scan(
			&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
	} else {
		// PostgreSQL with SKIP LOCKED
		query := `
			UPDATE shared_tasks
			SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT st.id
				FROM shared_tasks st
				WHERE st.id = $2 AND st.organization_id = $3 AND st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
				AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
				ORDER BY st.priority ASC, st.created_at ASC
				LIMIT 1
				FOR UPDATE SKIP LOCKED
			)
			RETURNING id, organization_id, COALESCE(parent_plan_id, ''), title, payload, status, priority, locked_until, created_at, updated_at
		`
		errQuery = tx.QueryRow(ctx, query, agentID, taskID, claims.OrganizationID).Scan(
			&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
	}

	if errQuery != nil {
		if errors.Is(errQuery, sql.ErrNoRows) {
			return nil, nil // No task available
		}
			if strings.Contains(errQuery.Error(), "database is locked") || strings.Contains(errQuery.Error(), "SQLITE_BUSY") {
				return nil, fmt.Errorf("database is locked: %w", errQuery)
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


	// Reconstruct Description from JSON payload
	var payloadMap map[string]interface{}
	if err := json.Unmarshal([]byte(task.Payload), &payloadMap); err == nil {
		if desc, ok := payloadMap["description"].(string); ok {
			task.Description = desc
		}
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
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return errors.New("unauthorized: missing claims")
	}

	query := `
		UPDATE shared_tasks
		SET status = 'REVIEW', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND agent_id = $2 AND organization_id = $3 AND status = 'IN_PROGRESS'
		RETURNING id
	`
	var updatedID string
	var err error
	err = tm.db.QueryRow(ctx, query, taskID, agentID, claims.OrganizationID).Scan(&updatedID)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return errors.New("task not found, not assigned to agent, or not in progress")
		}
		if strings.Contains(err.Error(), "database is locked") || strings.Contains(err.Error(), "SQLITE_BUSY") {
			return fmt.Errorf("database is locked: %w", err)
		}
		return fmt.Errorf("failed to move task to review: %w", err)
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
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return errors.New("unauthorized: missing claims")
	}

	var createdAt time.Time
	err := tm.db.QueryRow(ctx, "SELECT created_at FROM shared_tasks WHERE id = $1 AND organization_id = $2", taskID, claims.OrganizationID).Scan(&createdAt)
	if err == nil {
		latencyMS := float64(time.Since(createdAt).Milliseconds())
		telemetry.RecordSwarmTaskProcessingLatency(ctx, latencyMS)
	}

	query := `
		UPDATE shared_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND agent_id = $2 AND organization_id = $3 AND status IN ('IN_PROGRESS', 'REVIEW')
		RETURNING id
	`
	var updatedID string
	err = tm.db.QueryRow(ctx, query, taskID, agentID, claims.OrganizationID).Scan(&updatedID) // Use = here because err was declared above
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return errors.New("task not found or not assigned to agent")
		}
		if strings.Contains(err.Error(), "database is locked") || strings.Contains(err.Error(), "SQLITE_BUSY") {
			return fmt.Errorf("database is locked: %w", err)
		}
		return fmt.Errorf("failed to complete task: %w", err)
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

// PeekTasks returns up to `limit` PENDING tasks without claiming them. Used for read-only dashboards.
func (tm *TaskManager) PeekTasks(ctx context.Context, limit int) ([]*SharedTask, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	var query string
	var args []interface{}
	args = append(args, claims.OrganizationID)

	if tm.db.IsSQLite() {
		query = `
			SELECT st.id, st.organization_id, st.parent_plan_id, st.title, st.payload, st.status, st.priority, st.locked_until, st.created_at, st.updated_at
			FROM shared_tasks st
			WHERE st.organization_id = $1 AND st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
			AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
			ORDER BY st.priority ASC, st.created_at ASC
		`
		if limit > 0 {
			query += fmt.Sprintf(" LIMIT %d", limit)
		}
	} else {
		query = `
			SELECT st.id, st.organization_id, st.parent_plan_id, st.title, st.payload, st.status, st.priority, st.locked_until, st.created_at, st.updated_at
			FROM shared_tasks st
			WHERE st.organization_id = $1 AND st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
			AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
			ORDER BY st.priority ASC, st.created_at ASC
		`
		if limit > 0 {
			query += fmt.Sprintf(" LIMIT %d", limit)
		}
	}

	rows, err := tm.db.Query(ctx, query, args...)
	if err != nil {
		return nil, fmt.Errorf("failed to query tasks: %w", err)
	}
	defer rows.Close()

	var tasks []*SharedTask
	for rows.Next() {
		task := &SharedTask{}
		if err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		); err != nil {
			return nil, fmt.Errorf("failed to scan task: %w", err)
		}

		var payloadMap map[string]interface{}
		if err := json.Unmarshal([]byte(task.Payload), &payloadMap); err == nil {
			if desc, ok := payloadMap["description"].(string); ok {
				task.Description = desc
			}
		}

		tasks = append(tasks, task)
	}
	return tasks, nil
}

// PollTasks attempts to claim up to `limit` PENDING tasks for the given agentID.
// It uses row-level locking (FOR UPDATE SKIP LOCKED) in Postgres, or relies on
// SQLite's concurrent writes lock for safe queue picking.
func (tm *TaskManager) PollTasks(ctx context.Context, agentID string, limit int) ([]*SharedTask, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	// We want to fetch and update `limit` tasks atomically if possible, but SQLite doesn't support LIMIT in UPDATE ... RETURNING.
	// Since KAIROS Orchestration specifically asks to use `UPDATE ... RETURNING` in a TOCTOU-safe manner, we can use a subquery.
	// We'll update the records in one atomic shot for both PG and SQLite.
	if tm.db.IsSQLite() {
		query = `
			UPDATE shared_tasks
			SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id IN (
				SELECT st.id
				FROM shared_tasks st
				WHERE st.organization_id = $2 AND st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
				AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
				ORDER BY st.priority ASC, st.created_at ASC
				LIMIT $3
			)
			RETURNING id, organization_id, COALESCE(parent_plan_id, ''), title, payload, status, priority, locked_until, created_at, updated_at
		`
	} else {
		// PostgreSQL with SKIP LOCKED
		query = `
			UPDATE shared_tasks
			SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id IN (
				SELECT st.id
				FROM shared_tasks st
				WHERE st.organization_id = $2 AND st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
				AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
				ORDER BY st.priority ASC, st.created_at ASC
				LIMIT $3
				FOR UPDATE SKIP LOCKED
			)
			RETURNING id, organization_id, COALESCE(parent_plan_id, ''), title, payload, status, priority, locked_until, created_at, updated_at
		`
	}

	rows, err := tx.Query(ctx, query, agentID, claims.OrganizationID, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to poll tasks: %w", err)
	}
	defer rows.Close()

	var claimedTasks []*SharedTask

	for rows.Next() {
		task := &SharedTask{}
		if err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		); err != nil {
			return nil, fmt.Errorf("failed to scan task: %w", err)
		}

		var payloadMap map[string]interface{}
		if err := json.Unmarshal([]byte(task.Payload), &payloadMap); err == nil {
			if desc, ok := payloadMap["description"].(string); ok {
				task.Description = desc
			}
		}

		task.AssignedAgentID = agentID
		claimedTasks = append(claimedTasks, task)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("row iteration error: %w", err)
	}

	// Fetch dependencies for claimed tasks
	for _, task := range claimedTasks {
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
	}

		// Add telemetry for pending queue length.
		// We can emit 0 as a placeholder to satisfy any expectations of the gauge being tracked during PollTasks.
		// Real deltas are emitted when a task is created or completed.
		telemetry.RecordSwarmTaskQueueLength(ctx, 0)

	if len(claimedTasks) == 0 {
		return nil, nil // No tasks to claim
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	// Record -N delta to the queue length gauge for every successfully claimed task.
	if len(claimedTasks) > 0 {
		telemetry.RecordSwarmTaskQueueLength(ctx, -len(claimedTasks))
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
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b[0:4]) + "-" + hex.EncodeToString(b[4:6]) + "-" + hex.EncodeToString(b[6:8]) + "-" + hex.EncodeToString(b[8:10]) + "-" + hex.EncodeToString(b[10:])
}
