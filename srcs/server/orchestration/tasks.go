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
	"strings"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/memory/autodream"
	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
	"github.com/onehumancorp/mono/srcs/server/orchestration/statemachine"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// SharedTask represents a shared task distributed across agents.
type SharedTask struct {
	ID              string     `json:"id"`
	OrganizationID  string     `json:"organization_id"`
	ParentPlanID    string     `json:"parent_plan_id"`
	Dependencies    []string   `json:"dependencies"`
	Title           string     `json:"title"`
	Description     string     `json:"description,omitempty"`
	AssignedAgentID string     `json:"assigned_agent_id,omitempty"`
	Status          string     `json:"status"` // PENDING, IN_PROGRESS, COMPLETED, FAILED, BLOCKED
	Priority        string     `json:"priority"`
	Payload         string     `json:"payload"`
	LockedUntil     *time.Time `json:"locked_until,omitempty"`
	UltraPlanPhase  string     `json:"ultraplan_phase,omitempty"`
	DeliberationLog string     `json:"deliberation_log,omitempty"`
	Depth           int        `json:"depth,omitempty"`
	CreatedAt       time.Time  `json:"created_at"`
	UpdatedAt       time.Time  `json:"updated_at"`
}

// TaskManager manages the shared tasks list
type TaskManager struct {
	db           db.Provider
	redisClient  rueidis.Client
	hub          *CentrifugeNode // For Teammate Mesh broadcast
	stopChan     chan struct{}
	stateMachine *statemachine.StateMachine
	taskQueue    queue.TaskQueue
	mu           sync.Mutex // For Standalone mode SQLite locking
	autodream    autodream.MemoryConsolidator
	mesh         MeshTransport
}

// NewTaskManager creates a new TaskManager.
func NewTaskManager(provider db.Provider, hub *CentrifugeNode, ad autodream.MemoryConsolidator) *TaskManager {
	var broadcast func(string, map[string]interface{})
	if hub != nil {
		broadcast = hub.PublishTaskBroadcast
	}

	tm := &TaskManager{
		db:           provider,
		hub:          hub,
		stateMachine: statemachine.NewStateMachine(provider, broadcast),
		autodream:    ad,
	}

	if envBoolDefault("OHC_MULTITENANT", true) {
		redisURL := os.Getenv("REDIS_URL")
		if redisURL != "" {
			opts, err := rueidis.ParseURL(redisURL)
			if err == nil {
				c, err := rueidis.NewClient(opts)
				if err == nil {
					tm.redisClient = c
					tm.taskQueue = queue.NewRedisTaskQueue(c, "")
				}
			}
		}
	}

	// Fallback to SQLite queue if not using Redis
	if tm.taskQueue == nil {
		tm.taskQueue = queue.NewSQLiteTaskQueue(provider)
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

func (tm *TaskManager) SetMeshTransport(mt MeshTransport) {
	tm.mesh = mt
}

// CreateTask creates a new shared task.
// CreateTask creates a new pending task.
func (tm *TaskManager) CreateTask(ctx context.Context, organizationID, title, description, priority string) (*SharedTask, error) {
	return tm.CreateTaskWithPlan(ctx, organizationID, "", nil, title, description, priority)
}

// CreateTaskWithPlan creates a new pending task with plan association.
func (tm *TaskManager) CreateTaskWithPlan(ctx context.Context, organizationID string, parentPlanID string, dependencies []string, title, description, priority string) (*SharedTask, error) {
	// When creating a new task, we don't have its ID yet to check for a cycle
	// The real risk of circular dependencies is when updating dependencies.
	// But we can verify that the new task doesn't add a dependency on itself, which is trivially true here.

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

	if tm.db.IsSQLite() {
		tm.mu.Lock()
		defer tm.mu.Unlock()
	}

	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	if tm.db.IsSQLite() {
		query = `
			INSERT INTO shared_tasks (id, organization_id, parent_plan_id, title, description, payload, status, priority, ultraplan_phase, deliberation_log, depth)
			VALUES ($1, $2, NULLIF($7, ''), $3, $4, $5, 'PENDING', $6, 'PROPOSE', '[]', COALESCE((SELECT depth FROM shared_tasks WHERE id = $7), -1) + 1)
			RETURNING id, organization_id, COALESCE(parent_plan_id, ''), title, payload, status, priority, COALESCE(ultraplan_phase, ''), COALESCE(deliberation_log, ''), COALESCE(depth, 0), created_at, updated_at
		`
	} else {
		query = `
			INSERT INTO shared_tasks (id, organization_id, parent_plan_id, title, description, payload, status, priority, ultraplan_phase, deliberation_log, depth)
			VALUES ($1, $2, NULLIF($7, ''), $3, $4, $5, 'PENDING', $6, 'PROPOSE', '[]', COALESCE((SELECT depth FROM shared_tasks WHERE id = $7), -1) + 1)
			RETURNING id, organization_id, COALESCE(parent_plan_id, ''), title, payload, status, priority, COALESCE(ultraplan_phase, ''), COALESCE(deliberation_log, ''), COALESCE(depth, 0), created_at, updated_at
		`
	}

	err = tx.QueryRow(ctx, query, id, organizationID, title, description, payload, priority, parentPlanID).Scan(
		&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.UltraPlanPhase, &task.DeliberationLog, &task.Depth, &task.CreatedAt, &task.UpdatedAt,
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

	if tm.db.IsSQLite() {
		tm.mu.Lock()
		defer tm.mu.Unlock()
	}

	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var task SharedTask
	var errQuery error

	var fetchedTaskID string
	var queryErr error
	if tm.db.IsSQLite() {
		// SQLite doesn't support UPDATE ... RETURNING with a LIMIT, so we use explicit two-step select-then-update within the transaction.
		selectQuery := `
			SELECT st.id
			FROM shared_tasks st
			WHERE st.id = $1 AND st.organization_id = $2 AND st.status = 'PENDING' AND (st.ultraplan_phase IS NULL OR st.ultraplan_phase = '' OR st.ultraplan_phase = 'APPROVED') AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
			AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
			ORDER BY st.priority ASC, st.created_at ASC
			LIMIT 1
		`
		queryErr = tx.QueryRow(ctx, selectQuery, taskID, claims.OrganizationID).Scan(&fetchedTaskID)
	} else {
		// PostgreSQL with FOR UPDATE SKIP LOCKED
		selectQuery := `
			SELECT st.id
			FROM shared_tasks st
			WHERE st.id = $1 AND st.organization_id = $2 AND st.status = 'PENDING' AND (st.ultraplan_phase IS NULL OR st.ultraplan_phase = '' OR st.ultraplan_phase = 'APPROVED') AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
			AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
			ORDER BY st.priority ASC, st.created_at ASC
			LIMIT 1 FOR UPDATE SKIP LOCKED
		`
		queryErr = tx.QueryRow(ctx, selectQuery, taskID, claims.OrganizationID).Scan(&fetchedTaskID)
	}

	if queryErr != nil {
		if errors.Is(queryErr, sql.ErrNoRows) {
			return nil, nil // No task available or blocked
		}
		if strings.Contains(queryErr.Error(), "database is locked") || strings.Contains(queryErr.Error(), "SQLITE_BUSY") {
			return nil, fmt.Errorf("database is locked: %w", queryErr)
		}
		return nil, fmt.Errorf("failed to check pending task: %w", queryErr)
	}

	broadcastFunc, err := tm.stateMachine.TransitionWithTx(ctx, tx, fetchedTaskID, "SHARED_TASK", statemachine.StateInProgress, agentID, "Claimed task")
	if err != nil {
		return nil, fmt.Errorf("failed to transition state: %w", err)
	}

	// Fetch updated task data
	readQuery := `
		SELECT id, organization_id, COALESCE(parent_plan_id, ''), title, payload, status, priority, locked_until, COALESCE(ultraplan_phase, ''), COALESCE(deliberation_log, ''), COALESCE(depth, 0), created_at, updated_at
		FROM shared_tasks
		WHERE id = $1
	`
	errQuery = tx.QueryRow(ctx, readQuery, fetchedTaskID).Scan(
		&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.LockedUntil, &task.UltraPlanPhase, &task.DeliberationLog, &task.Depth, &task.CreatedAt, &task.UpdatedAt,
	)

	if errQuery != nil {
		return nil, fmt.Errorf("failed to read claimed task: %w", errQuery)
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

	if broadcastFunc != nil {
		broadcastFunc()
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

	if tm.db.IsSQLite() {
		tm.mu.Lock()
		defer tm.mu.Unlock()
	}

	// Verify ownership first
	var currentStatus string
	err := tm.db.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1 AND agent_id = $2 AND organization_id = $3", taskID, agentID, claims.OrganizationID).Scan(&currentStatus)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return errors.New("task not found or not assigned to agent")
		}
		return fmt.Errorf("failed to verify task ownership: %w", err)
	}

	err = tm.stateMachine.Transition(ctx, taskID, "SHARED_TASK", statemachine.StateReview, agentID, "Agent requested review")
	if err != nil {
		if strings.Contains(err.Error(), "database is locked") || strings.Contains(err.Error(), "SQLITE_BUSY") {
			return fmt.Errorf("database is locked: %w", err)
		}
		return fmt.Errorf("failed to move task to review: %w", err)
	}

	telemetry.RecordSwarmTaskTransition(ctx, claims.OrganizationID, currentStatus, "REVIEW")

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
	return tm.CompleteTaskWithResult(ctx, taskID, agentID, "")
}

// CompleteTaskWithResult marks a task as completed and persists the completion result.
func (tm *TaskManager) CompleteTaskWithResult(ctx context.Context, taskID, agentID, result string) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return errors.New("unauthorized: missing claims")
	}

	if tm.db.IsSQLite() {
		tm.mu.Lock()
		defer tm.mu.Unlock()
	}

	var createdAt time.Time
	var currentStatus string
	err := tm.db.QueryRow(ctx, "SELECT created_at, status FROM shared_tasks WHERE id = $1 AND agent_id = $2 AND organization_id = $3", taskID, agentID, claims.OrganizationID).Scan(&createdAt, &currentStatus)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return errors.New("task not found or not assigned to agent")
		}
		return fmt.Errorf("failed to verify task ownership: %w", err)
	}

	latencyMS := float64(time.Since(createdAt).Milliseconds())
	telemetry.RecordSwarmTaskProcessingLatency(ctx, latencyMS)

	if currentStatus == statemachine.StateCompleted {
		return errors.New("task already completed")
	}

	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	broadcast, err := tm.stateMachine.TransitionWithTx(ctx, tx, taskID, "SHARED_TASK", statemachine.StateCompleted, agentID, "Task completed successfully")
	if err != nil {
		if strings.Contains(err.Error(), "database is locked") || strings.Contains(err.Error(), "SQLITE_BUSY") {
			return fmt.Errorf("database is locked: %w", err)
		}
		return fmt.Errorf("failed to complete task: %w", err)
	}

	if err := tm.persistSharedTaskCompletion(ctx, tx, taskID, result); err != nil {
		return err
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit completed task: %w", err)
	}
	if broadcast != nil {
		broadcast()
	}

	telemetry.RecordSwarmTaskTransition(ctx, claims.OrganizationID, currentStatus, "COMPLETED")

	if tm.autodream != nil {
		go func() {
			logLine := strings.TrimSpace(result)
			if logLine == "" {
				logLine = "Task " + taskID + " completed successfully."
			}
			logs := []string{logLine}
			_ = tm.autodream.Consolidate(context.Background(), taskID, logs)
		}()
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

	// Broadcast mesh events
	if tm.mesh != nil {
		go func() {
			payload := map[string]interface{}{
				"task_id":  taskID,
				"action":   "COMPLETE",
				"agent_id": agentID,
				"status":   "COMPLETED",
			}
			payloadBytes, _ := json.Marshal(payload)
			_ = tm.mesh.BroadcastMeshEvent(context.Background(), "tasks", payloadBytes)
		}()
	}

	return nil
}

func (tm *TaskManager) persistSharedTaskCompletion(ctx context.Context, tx db.Tx, taskID, result string) error {
	var payloadText string
	var deliberationLog string
	err := tx.QueryRow(ctx, "SELECT COALESCE(payload, '{}'), COALESCE(deliberation_log, '{}') FROM shared_tasks WHERE id = $1", taskID).Scan(&payloadText, &deliberationLog)
	if err != nil {
		return fmt.Errorf("failed to load task payload: %w", err)
	}

	payloadBytes, err := mergeTaskResultPayload(payloadText, result)
	if err != nil {
		return fmt.Errorf("failed to encode task payload: %w", err)
	}

	updatedLog, err := appendDeliberationResult(deliberationLog, result)
	if err != nil {
		return fmt.Errorf("failed to encode deliberation log: %w", err)
	}

	_, err = tx.Exec(ctx, `
		UPDATE shared_tasks
		SET payload = $2, deliberation_log = $3, updated_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`, taskID, string(payloadBytes), updatedLog)
	if err != nil {
		return fmt.Errorf("failed to persist task result: %w", err)
	}

	return nil
}

func mergeTaskResultPayload(payloadText, result string) ([]byte, error) {
	payloadMap := map[string]interface{}{}
	if strings.TrimSpace(payloadText) != "" {
		if err := json.Unmarshal([]byte(payloadText), &payloadMap); err != nil {
			return nil, err
		}
	}
	if strings.TrimSpace(result) != "" {
		payloadMap["result"] = result
		payloadMap["completed_at"] = time.Now().UTC().Format(time.RFC3339Nano)
	}
	return json.Marshal(payloadMap)
}

func appendDeliberationResult(deliberationLog, result string) (string, error) {
	trimmed := strings.TrimSpace(result)
	if trimmed == "" {
		return deliberationLog, nil
	}

	entries := []string{}
	trimmedLog := strings.TrimSpace(deliberationLog)
	if trimmedLog != "" && trimmedLog != "{}" && trimmedLog != "null" {
		if err := json.Unmarshal([]byte(deliberationLog), &entries); err != nil {
			return "", err
		}
	}
	entries = append(entries, trimmed)
	updated, err := json.Marshal(entries)
	if err != nil {
		return "", err
	}
	return string(updated), nil
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
			SELECT st.id, st.organization_id, COALESCE(st.parent_plan_id, ''), st.title, st.payload, st.status, st.priority, st.locked_until, st.created_at, st.updated_at
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
			SELECT st.id, st.organization_id, COALESCE(st.parent_plan_id, ''), st.title, st.payload, st.status, st.priority, st.locked_until, st.created_at, st.updated_at
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

	if tm.db.IsSQLite() {
		tm.mu.Lock()
		defer tm.mu.Unlock()
	}

	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var claimedTasks []*SharedTask
	var broadcastFuncs []func()

	if tm.db.IsSQLite() {
		// SQLite: explicit select-then-update to bypass limit/returning issues
		selectQuery := `
			SELECT st.id
			FROM shared_tasks st
			WHERE st.organization_id = $1 AND st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
			AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
			ORDER BY st.priority ASC, st.created_at ASC
			LIMIT $2
		`
		rows, err := tx.Query(ctx, selectQuery, claims.OrganizationID, limit)
		if err != nil {
			return nil, fmt.Errorf("failed to find tasks: %w", err)
		}

		var taskIDs []string
		for rows.Next() {
			var id string
			if err := rows.Scan(&id); err == nil {
				taskIDs = append(taskIDs, id)
			}
		}

		if err := rows.Err(); err != nil {
			rows.Close()
			return nil, fmt.Errorf("row iteration error: %w", err)
		}
		rows.Close()

		if len(taskIDs) == 0 {
			return nil, nil // No tasks found
		}

		for _, id := range taskIDs {
			broadcastFunc, err := tm.stateMachine.TransitionWithTx(ctx, tx, id, "SHARED_TASK", statemachine.StateInProgress, agentID, "Polled task")
			if err != nil {
				return nil, fmt.Errorf("failed to transition state: %w", err)
			}
			broadcastFuncs = append(broadcastFuncs, broadcastFunc)

			// Fetch updated task data
			readQuery := `
				SELECT id, organization_id, COALESCE(parent_plan_id, ''), title, payload, status, priority, locked_until, created_at, updated_at
				FROM shared_tasks
				WHERE id = $1
			`
			task := &SharedTask{}
			errQuery := tx.QueryRow(ctx, readQuery, id).Scan(
				&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
			)

			if errQuery != nil {
				return nil, fmt.Errorf("failed to read claimed task: %w", errQuery)
			}

			task.AssignedAgentID = agentID

			var payloadMap map[string]interface{}
			if err := json.Unmarshal([]byte(task.Payload), &payloadMap); err == nil {
				if desc, ok := payloadMap["description"].(string); ok {
					task.Description = desc
				}
			}

			claimedTasks = append(claimedTasks, task)
		}

	} else {
		// PostgreSQL with SKIP LOCKED
		selectQuery := `
				SELECT st.id
				FROM shared_tasks st
				WHERE st.organization_id = $1 AND st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
				AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
				ORDER BY st.priority ASC, st.created_at ASC
				LIMIT $2 FOR UPDATE SKIP LOCKED
		`

		rows, err := tx.Query(ctx, selectQuery, claims.OrganizationID, limit)
		if err != nil {
			return nil, fmt.Errorf("failed to find tasks: %w", err)
		}

		var taskIDs []string
		for rows.Next() {
			var id string
			if err := rows.Scan(&id); err == nil {
				taskIDs = append(taskIDs, id)
			}
		}

		if err := rows.Err(); err != nil {
			rows.Close()
			return nil, fmt.Errorf("row iteration error: %w", err)
		}
		rows.Close()

		for _, id := range taskIDs {
			broadcastFunc, err := tm.stateMachine.TransitionWithTx(ctx, tx, id, "SHARED_TASK", statemachine.StateInProgress, agentID, "Polled task")
			if err != nil {
				return nil, fmt.Errorf("failed to transition state: %w", err)
			}
			broadcastFuncs = append(broadcastFuncs, broadcastFunc)

			// Fetch updated task data
			readQuery := `
				SELECT id, organization_id, COALESCE(parent_plan_id, ''), title, payload, status, priority, locked_until, created_at, updated_at
				FROM shared_tasks
				WHERE id = $1
			`
			task := &SharedTask{}
			errQuery := tx.QueryRow(ctx, readQuery, id).Scan(
				&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
			)

			if errQuery != nil {
				return nil, fmt.Errorf("failed to read claimed task: %w", errQuery)
			}

			task.AssignedAgentID = agentID

			var payloadMap map[string]interface{}
			if err := json.Unmarshal([]byte(task.Payload), &payloadMap); err == nil {
				if desc, ok := payloadMap["description"].(string); ok {
					task.Description = desc
				}
			}

			claimedTasks = append(claimedTasks, task)
		}
	}

	for _, task := range claimedTasks {
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
	}

	if len(claimedTasks) == 0 {
		return nil, nil // No tasks to claim
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	for _, b := range broadcastFuncs {
		if b != nil {
			b()
		}
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

// DelegateSubTask queues a task to an isolated sub-agent worker
func (tm *TaskManager) DelegateSubTask(ctx context.Context, parentTaskID, agentRole string, payloadMap map[string]interface{}) error {
	payloadBytes, err := json.Marshal(payloadMap)
	if err != nil {
		return err
	}
	job := &queue.Job{
		ID:           generateID(),
		ParentTaskID: parentTaskID,
		AgentRole:    agentRole,
		Payload:      string(payloadBytes),
		MaxAttempts:  3,
	}
	return tm.taskQueue.Enqueue(ctx, job)
}

// generateID generates a pseudo-uuid for SQLite compatibility.
func generateID() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b[0:4]) + "-" + hex.EncodeToString(b[4:6]) + "-" + hex.EncodeToString(b[6:8]) + "-" + hex.EncodeToString(b[8:10]) + "-" + hex.EncodeToString(b[10:])
}

// CheckCircularDependency ensures that the dependencies do not create a cycle.
func (tm *TaskManager) CheckCircularDependency(ctx context.Context, taskID string, dependencies []string) error {
	if len(dependencies) == 0 {
		return nil
	}

	visited := make(map[string]bool)
	queue := append([]string{}, dependencies...)

	for len(queue) > 0 {
		currID := queue[0]
		queue = queue[1:]

		if currID == taskID {
			return fmt.Errorf("circular dependency detected for task %s", taskID)
		}

		if visited[currID] {
			continue
		}
		visited[currID] = true

		depQuery := `SELECT depends_on_task_id FROM task_dependencies WHERE task_id = $1`
		rows, err := tm.db.Query(ctx, depQuery, currID)
		if err != nil {
			return fmt.Errorf("failed to check dependencies: %w", err)
		}

		for rows.Next() {
			var childDep string
			if err := rows.Scan(&childDep); err == nil {
				queue = append(queue, childDep)
			}
		}
		rows.Close()
	}

	return nil
}
