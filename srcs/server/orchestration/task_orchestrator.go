package orchestration

import (
	"context"
	"crypto/rand"
	"database/sql"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// TaskOrchestrator abstracts the state machine and dependency tracking for the Teammate Mesh


type SharedTaskDecompositionDB struct {
	ID              string          `json:"id" db:"id"`
	OrganizationID  string          `json:"organization_id" db:"organization_id"`
	Title           string          `json:"title" db:"title"`
	Description     *string         `json:"description" db:"description"`
	Status          string          `json:"status" db:"status"`
	AssignedAgentID *string         `json:"assigned_agent_id" db:"assigned_agent_id"`
	Priority        string          `json:"priority" db:"priority"`
	Payload         json.RawMessage `json:"payload" db:"payload"`
	ParentPlanID    *string         `json:"parent_plan_id" db:"parent_plan_id"`
	Dependencies    json.RawMessage `json:"dependencies" db:"dependencies"`
	LockedUntil     *time.Time      `json:"locked_until" db:"locked_until"`
	CreatedAt       time.Time       `json:"created_at" db:"created_at"`
	UpdatedAt       time.Time       `json:"updated_at" db:"updated_at"`
}


type TaskOrchestrator interface {
	ReceiveHighLevelRequest(ctx context.Context, orgID, title string) (string, error)
	EnqueueTask(ctx context.Context, task *models.Task, dependsOn []string) (*models.Task, error)
	AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*models.Task, error)
	CompleteTask(ctx context.Context, taskID string, agentID string, result string) error
}

type DefaultTaskOrchestrator struct {
	db           db.Provider
	redisClient  rueidis.Client
	hub          *CentrifugeNode
	mesh         MeshTransport
	spawner      SubAgentSpawner
	mu           sync.Mutex // For standalone mode coordination
	workerCtx    context.Context
	workerCancel context.CancelFunc
	workerWg     sync.WaitGroup
	taskQueue    queue.TaskQueue
	subWorker    *SubAgentWorker
}

func NewTaskOrchestrator(provider db.Provider, redisClient rueidis.Client, hub *CentrifugeNode, mesh MeshTransport) TaskOrchestrator {
	ctx, cancel := context.WithCancel(context.Background())

	spawner := NewDefaultSubAgentSpawner(provider, nil, hub, 10)

	var tq queue.TaskQueue
	if redisClient == nil {
		tq = queue.NewSQLiteTaskQueue(provider)
	} else {
		tq = queue.NewPostgresTaskQueue(provider)
	}

	subWorker := NewSubAgentWorker(tq, spawner)

	to := &DefaultTaskOrchestrator{
		db:           provider,
		redisClient:  redisClient,
		hub:          hub,
		mesh:         mesh,
		spawner:      spawner,
		workerCtx:    ctx,
		workerCancel: cancel,
		taskQueue:    tq,
		subWorker:    subWorker,
	}
	to.StartBackgroundWorker()
	subWorker.Start(ctx)
	return to
}

// StartBackgroundWorker starts the background loop that queries the queue and dispatches jobs.
func (to *DefaultTaskOrchestrator) StartBackgroundWorker() {
	to.workerWg.Add(1)

	// Start the SubAgentSpawner monitor
	go func() {
		_ = to.spawner.Monitor(to.workerCtx)
	}()

	go func() {
		defer to.workerWg.Done()

		ticker := time.NewTicker(2 * time.Second)
		defer ticker.Stop()

		for {
			select {
			case <-to.workerCtx.Done():
				return
			case <-ticker.C:
				to.pollAndDelegateTasks()
			}
		}
	}()
}

// pollAndDelegateTasks queries for DELEGATED priority tasks and spawns sub-agents.
func (to *DefaultTaskOrchestrator) pollAndDelegateTasks() {
	if to.redisClient == nil {
		to.mu.Lock()
		defer to.mu.Unlock()
	}

	tx, err := to.db.Begin(to.workerCtx)
	if err != nil {
		return
	}
	defer tx.Rollback(to.workerCtx)

	var taskID string
	var orgID string
	var query string

	if to.db.IsSQLite() {
		selectQuery := `
			SELECT id, organization_id FROM shared_tasks
				WHERE status = 'PENDING' AND (priority = 'DELEGATED' OR json_extract(payload, '$.sub_agent_type') IS NOT NULL)
			ORDER BY created_at ASC
			LIMIT 1
		`
		err = tx.QueryRow(to.workerCtx, selectQuery).Scan(&taskID, &orgID)
		if err != nil {
			return // typically sql.ErrNoRows
		}

		updateQuery := `
			UPDATE shared_tasks
			SET status = 'IN_PROGRESS', agent_id = 'sub-agent-spawner', updated_at = CURRENT_TIMESTAMP
			WHERE id = $1 AND status = 'PENDING'
		`
		_, err = tx.Exec(to.workerCtx, updateQuery, taskID)
		if err != nil {
			return
		}

	} else {
		// Postgres mode: use FOR UPDATE SKIP LOCKED
		query = `
			UPDATE shared_tasks
			SET status = 'IN_PROGRESS', agent_id = 'sub-agent-spawner', updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id FROM shared_tasks
					WHERE status = 'PENDING' AND (priority = 'DELEGATED' OR payload->>'sub_agent_type' IS NOT NULL)
				ORDER BY created_at ASC
				LIMIT 1
				FOR UPDATE SKIP LOCKED
			)
			RETURNING id, organization_id
		`
		err = tx.QueryRow(to.workerCtx, query).Scan(&taskID, &orgID)
		if err != nil {
			return // sql.ErrNoRows or locking issue
		}
	}

	if err := tx.Commit(to.workerCtx); err != nil {
		return
	}

	// Spawn sub-agent by enqueuing to sub_agent_queue
	jobPayload := map[string]interface{}{
		"task_id":         taskID,
		"organization_id": orgID,
	}
	payloadBytes, _ := json.Marshal(jobPayload)

	job := &queue.Job{
		ID:           generateID(),
		ParentTaskID: taskID,
		AgentRole:    "sub-agent-spawner",
		Payload:      string(payloadBytes),
		MaxAttempts:  3,
	}

	_ = to.taskQueue.Enqueue(to.workerCtx, job)
}

func (to *DefaultTaskOrchestrator) Stop() {
	if to.subWorker != nil {
		to.subWorker.Stop()
	}
	if to.workerCancel != nil {
		to.workerCancel()
	}
	to.workerWg.Wait()
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
		INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, title, payload, status, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`
	_, err = tx.Exec(ctx, query, task.ID, task.MissionID, task.ParentPlanID, task.Title, payload, task.Status)
	if err != nil {
		return nil, fmt.Errorf("failed to insert task: %w", err)
	}

	// Insert dependencies
	if len(dependsOn) > 0 {
		for _, depID := range dependsOn {
			_, err = tx.Exec(ctx, "INSERT INTO swarm_task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)", task.ID, depID)
			if err != nil {
				return nil, fmt.Errorf("failed to insert dependency: %w", err)
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit: %w", err)
	}

	// Metrics
	telemetry.RecordTaskEnqueued(ctx, task.ID)

	// Broadcast
	if to.mesh != nil {
		_ = to.mesh.BroadcastTask(ctx, Task{
			AgentID: "",
			Action:  "CREATE",
			Status:  task.Status,
			TaskID:  task.ID,
		})
	} else if to.hub != nil {
		payload := map[string]interface{}{
			"task_id":    task.ID,
			"action":     "CREATE",
			"mission_id": task.MissionID,
			"title":      task.Title,
			"status":     task.Status,
		}
		to.hub.PublishTaskBroadcast(task.ID, payload)
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
		// In SQLite, use UPDATE RETURNING if supported, or SELECT then UPDATE.
		// However, UPDATE ... RETURNING with LIMIT is not supported in SQLite.
		// Instead, we use a two-step SELECT then UPDATE approach.
		// We should only select tasks where their dependencies are completed,
		// or they have no dependencies. Wait, if it's READY, doesn't it mean dependencies are completed?
		// In `EnqueueTask`, if `dependsOn` is empty, it sets status to 'READY'.
		// If `dependsOn` is not empty, it sets status to 'PENDING'.
		// In `CompleteTask`, it checks if all dependencies are completed and if so sets status to 'READY'.
		// Actually, to be safe, we can enforce DAG dependencies here as well just in case.
		// But let's check `EnqueueTask`.
		selectQuery := `
			SELECT st.id FROM swarm_tasks st
			WHERE st.status = 'READY'
			AND (SELECT COUNT(*) FROM swarm_task_dependencies td INNER JOIN swarm_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
			ORDER BY json_extract(st.payload, '$.priority') ASC, st.created_at ASC
			LIMIT 1
		`
		var taskID string
		err = tx.QueryRow(ctx, selectQuery).Scan(&taskID)
		if err != nil {
			tx.Rollback(ctx)
			if errors.Is(err, sql.ErrNoRows) {
				return nil, nil // Queue is empty
			}
			return nil, fmt.Errorf("failed to scan task ID in SQLite: %w", err)
		}

		updateQuery := `
			UPDATE swarm_tasks
			SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = $2 AND status = 'READY'
			RETURNING id, mission_id, COALESCE(parent_plan_id, ''), title, status, payload, created_at, updated_at
		`
		err = tx.QueryRow(ctx, updateQuery, agentID, taskID).Scan(
			&task.ID, &task.MissionID, &task.ParentPlanID, &task.Title, &task.Status, &task.Payload, &task.CreatedAt, &task.UpdatedAt,
		)
	} else {
		// In Postgres, use UPDATE RETURNING with a subquery that uses FOR UPDATE SKIP LOCKED.
		query = `
			UPDATE swarm_tasks
			SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT st.id FROM swarm_tasks st
				WHERE st.status = 'READY'
				AND (SELECT COUNT(*) FROM swarm_task_dependencies td INNER JOIN swarm_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
				ORDER BY st.payload->>'priority' ASC, st.created_at ASC
				LIMIT 1
				FOR UPDATE SKIP LOCKED
			)
			RETURNING id, mission_id, COALESCE(parent_plan_id, ''), title, status, payload, created_at, updated_at
		`
		err = tx.QueryRow(ctx, query, agentID).Scan(
			&task.ID, &task.MissionID, &task.ParentPlanID, &task.Title, &task.Status, &task.Payload, &task.CreatedAt, &task.UpdatedAt,
		)
	}

	if err != nil {
		tx.Rollback(ctx)
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

	task.AssignedAgentID = agentID

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
		payload := map[string]interface{}{
			"task_id":  task.ID,
			"action":   "CLAIM",
			"agent_id": agentID,
			"status":   task.Status,
		}
		to.hub.PublishTaskBroadcast(task.ID, payload)
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

	var taskPayload string
	err = tx.QueryRow(ctx, "SELECT COALESCE(payload, '{}') FROM swarm_tasks WHERE id = $1 AND assigned_agent_id = $2 AND status = 'IN_PROGRESS'", taskID, agentID).Scan(&taskPayload)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) || err.Error() == "no rows in result set" {
			return fmt.Errorf("task not found or not assigned to agent")
		}
		return fmt.Errorf("failed to load task for completion: %w", err)
	}

	updatedPayload, err := mergeTaskResultPayload(taskPayload, result)
	if err != nil {
		return fmt.Errorf("failed to encode swarm task result: %w", err)
	}

	// Update status to COMPLETED
	var returnedID string
	err = tx.QueryRow(ctx, "UPDATE swarm_tasks SET status = 'COMPLETED', payload = $3, updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND assigned_agent_id = $2 AND status = 'IN_PROGRESS' RETURNING id", taskID, agentID, string(updatedPayload)).Scan(&returnedID)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) || err.Error() == "no rows in result set" {
			return fmt.Errorf("task not found or not assigned to agent")
		}
		return fmt.Errorf("failed to complete task: %w", err)
	}

	// Find dependent tasks that might now be READY
	// A task is READY if all its dependencies are COMPLETED.
	// We do this by checking all tasks that depend on the completed one,
	// and for each, checking if any unresolved dependencies remain.
	query := `
		SELECT t.id
		FROM swarm_tasks t
		JOIN swarm_task_dependencies td ON t.id = td.task_id
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
			FROM swarm_task_dependencies td
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

	// Metrics
	telemetry.RecordSwarmTaskCompleted(ctx, taskID)
	taskPayload = string(updatedPayload)

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
		payload := map[string]interface{}{
			"task_id":  taskID,
			"action":   "COMPLETE",
			"agent_id": agentID,
			"status":   "COMPLETED",
		}
		to.hub.PublishTaskBroadcast(taskID, payload)

		for _, rid := range newReadyTasks {
			readyPayload := map[string]interface{}{
				"task_id": rid,
				"action":  "READY",
				"status":  "READY",
			}
			to.hub.PublishTaskBroadcast(rid, readyPayload)
		}
	}

	// Trigger AutoDream embedding in background
	go func() {
		bgCtx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
		defer cancel()

		worker := NewAutoDreamWorker(to.db)

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

func (to *DefaultTaskOrchestrator) ReceiveHighLevelRequest(ctx context.Context, orgID, title string) (string, error) {
	tx, err := to.db.Begin(ctx)
	if err != nil {
		return "", err
	}
	defer tx.Rollback(ctx)

	b := make([]byte, 16)
	_, _ = rand.Read(b)
	taskID := hex.EncodeToString(b)
	_, err = tx.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ($1, $2, $3, $4)", taskID, orgID, title, "PENDING")
	if err != nil {
		return "", err
	}
	_, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = $1 WHERE id = $2", "DECOMPOSING", taskID)
	if err != nil {
		return "", err
	}

	tx.Commit(ctx)

	sm := NewTaskStateMachine(to.db, to.redisClient)
	err = sm.ProcessEvent(ctx, taskID, EventDecompositionComplete)
	if err != nil {
		return "", err
	}

	return taskID, nil
}



func (to *DefaultTaskOrchestrator) ClaimDecompositionTask(ctx context.Context, agentID string) (*SharedTaskDecompositionDB, error) {
	if to.db.IsSQLite() {
		return to.claimDecompositionTaskSQLite(ctx, agentID)
	}
	return to.claimDecompositionTaskPostgres(ctx, agentID)
}

func (to *DefaultTaskOrchestrator) claimDecompositionTaskSQLite(ctx context.Context, agentID string) (*SharedTaskDecompositionDB, error) {
	to.mu.Lock()
	defer to.mu.Unlock()

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
		FROM shared_tasks_decomposition
		WHERE status = 'PENDING'
		LIMIT 1
	`
	row := tx.QueryRow(ctx, query)

	var task SharedTaskDecompositionDB
	var payloadStr, dependenciesStr string
    var createdAtStr, updatedAtStr string
    var lockedUntil *time.Time
	if err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description,
		&task.Status, &task.AssignedAgentID, &task.Priority, &payloadStr, &task.ParentPlanID,
		&dependenciesStr, &lockedUntil, &createdAtStr, &updatedAtStr,
	); err != nil {
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}

	task.Payload = []byte(payloadStr)
	task.Dependencies = []byte(dependenciesStr)
    task.LockedUntil = lockedUntil
    task.CreatedAt, _ = time.Parse("2006-01-02 15:04:05", createdAtStr)
    task.UpdatedAt, _ = time.Parse("2006-01-02 15:04:05", updatedAtStr)

	if _, err = tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = &agentID
	return &task, nil
}

func (to *DefaultTaskOrchestrator) claimDecompositionTaskPostgres(ctx context.Context, agentID string) (*SharedTaskDecompositionDB, error) {
	tx, err := to.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
		FROM shared_tasks_decomposition
		WHERE status = 'PENDING'
		LIMIT 1
		FOR UPDATE SKIP LOCKED
	`
	row := tx.QueryRow(ctx, query)

	var task SharedTaskDecompositionDB
	if err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description,
		&task.Status, &task.AssignedAgentID, &task.Priority, &task.Payload, &task.ParentPlanID,
		&task.Dependencies, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
	); err != nil {
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}

	if _, err = tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = &agentID
	return &task, nil
}
