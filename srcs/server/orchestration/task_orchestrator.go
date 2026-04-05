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
	workerCtx   context.Context
	workerCancel context.CancelFunc
	workerWg    sync.WaitGroup
	spawner     SubAgentSpawner
}

func NewTaskOrchestrator(provider db.Provider, redisClient rueidis.Client, hub *CentrifugeNode, mesh TeammateMesh) TaskOrchestrator {
	ctx, cancel := context.WithCancel(context.Background())
	to := &DefaultTaskOrchestrator{
		db:          provider,
		redisClient: redisClient,
		hub:         hub,
		mesh:        mesh,
		workerCtx:   ctx,
		workerCancel: cancel,
	}
	// Note: We use nil hub for task manager to avoid circular dependency in DefaultSubAgentSpawner
	to.spawner = NewDefaultSubAgentSpawner(hub, NewTaskManager(provider, nil))
	to.StartBackgroundWorker()
	return to
}

// StartBackgroundWorker starts the background loop that queries the queue and dispatches jobs.
func (to *DefaultTaskOrchestrator) StartBackgroundWorker() {
	to.workerWg.Add(1)
	go func() {
		defer to.workerWg.Done()
		// Capacity-managed channel for throttling
		concurrencyLimit := 10
		sem := make(chan struct{}, concurrencyLimit)

		ticker := time.NewTicker(1 * time.Second)
		defer ticker.Stop()

		for {
			select {
			case <-to.workerCtx.Done():
				return
			case <-ticker.C:
				// Only poll DELEGATED tasks for Sub-Agent Spawner worker, otherwise leave it.
				// We do a manual peek to avoid acquiring non-delegated tasks.
				// For the sake of the KAIROS DAG orchestration, the orchestrator only consumes
				// DELEGATED tasks locally.

				// A simpler query just for sub agent polling
				var candidateID string
				query := `SELECT id FROM swarm_tasks WHERE status = 'READY' AND json_extract(payload, '$.priority') = 'DELEGATED' LIMIT 1`
				if !to.db.IsSQLite() {
					query = `SELECT id FROM swarm_tasks WHERE status = 'READY' AND payload->>'priority' = 'DELEGATED' LIMIT 1 FOR UPDATE SKIP LOCKED`
				}

				tx, err := to.db.Begin(to.workerCtx)
				if err != nil {
					continue
				}
				err = tx.QueryRow(to.workerCtx, query).Scan(&candidateID)

				if err != nil {
					_ = tx.Rollback(to.workerCtx)
					continue
				}

				// Re-acquire correctly with ID
				var task models.Task
				if to.db.IsSQLite() {
					updateQuery := `
						UPDATE swarm_tasks
						SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
						WHERE id = $2
						RETURNING id, mission_id, title, status, payload, created_at, updated_at
					`
					err = tx.QueryRow(to.workerCtx, updateQuery, "sub-agent-spawner", candidateID).Scan(
						&task.ID, &task.MissionID, &task.Title, &task.Status, &task.Payload, &task.CreatedAt, &task.UpdatedAt,
					)
				} else {
					updateQuery := `
						UPDATE swarm_tasks
						SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
						WHERE id = $2
						RETURNING id, mission_id, title, status, payload, created_at, updated_at
					`
					err = tx.QueryRow(to.workerCtx, updateQuery, "sub-agent-spawner", candidateID).Scan(
						&task.ID, &task.MissionID, &task.Title, &task.Status, &task.Payload, &task.CreatedAt, &task.UpdatedAt,
					)
				}

				_ = tx.Commit(to.workerCtx)

				if err == nil && task.ID != "" {
					if to.db.IsSQLite() {
						sem <- struct{}{}
					}

					go func(t *models.Task) {
						if to.db.IsSQLite() {
							defer func() { <-sem }()
						}

						sharedTask := &SharedTask{
							ID:             t.ID,
							OrganizationID: "system",
							ParentPlanID:   t.MissionID,
						}
						_ = to.spawner.Spawn(context.Background(), sharedTask)
					}(&task)
				}
			}
		}
	}()
}

func (to *DefaultTaskOrchestrator) Stop() {
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
			selectQuery := `
				SELECT id FROM swarm_tasks
				WHERE status = 'READY'
				ORDER BY json_extract(payload, '$.priority') ASC, created_at ASC
				LIMIT 1
			`
			var taskID string
			err = tx.QueryRow(ctx, selectQuery).Scan(&taskID)
			if err != nil {
				return nil, err // Could be sql.ErrNoRows if queue is empty
			}

			updateQuery := `
			UPDATE swarm_tasks
			SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
				WHERE id = $2
			RETURNING id, mission_id, title, status, payload, created_at, updated_at
		`
			err = tx.QueryRow(ctx, updateQuery, agentID, taskID).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Status, &task.Payload, &task.CreatedAt, &task.UpdatedAt,
		)
	} else {
		// In Postgres, use UPDATE RETURNING with a subquery that uses FOR UPDATE SKIP LOCKED.
		query = `
			UPDATE swarm_tasks
			SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id FROM swarm_tasks
				WHERE status = 'READY'
				ORDER BY payload->>'priority' ASC, created_at ASC
				LIMIT 1
				FOR UPDATE SKIP LOCKED
			)
			RETURNING id, mission_id, title, status, payload, created_at, updated_at
		`
		err = tx.QueryRow(ctx, query, agentID).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Status, &task.Payload, &task.CreatedAt, &task.UpdatedAt,
		)
	}

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

	// Update status to COMPLETED
	var returnedID string
	err = tx.QueryRow(ctx, "UPDATE swarm_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND assigned_agent_id = $2 AND status = 'IN_PROGRESS' RETURNING id", taskID, agentID).Scan(&returnedID)
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

	// Metrics
	telemetry.RecordSwarmTaskCompleted(ctx, taskID)

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
