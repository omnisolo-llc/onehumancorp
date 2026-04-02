package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// TaskOrchestrator manages the shared task list state machine and sub-agent queues.
type TaskOrchestrator interface {
	EnqueueTask(ctx context.Context, task *SharedTask, dependencies []string) error
	AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*SharedTask, error)
	CompleteTask(ctx context.Context, taskID string, agentID string, result string) error
}

// taskOrchestrator is the default implementation of TaskOrchestrator.
type taskOrchestrator struct {
	db          db.Provider
	redisClient rueidis.Client
	hub         *CentrifugeNode
	mu          sync.Mutex
	llmClient   MinimaxClient
}

// NewTaskOrchestrator creates a new TaskOrchestrator.
func NewTaskOrchestrator(db db.Provider, redisClient rueidis.Client, hub *CentrifugeNode, llmClient MinimaxClient) TaskOrchestrator {
	return &taskOrchestrator{
		db:          db,
		redisClient: redisClient,
		hub:         hub,
		llmClient:   llmClient,
	}
}

func (o *taskOrchestrator) EnqueueTask(ctx context.Context, task *SharedTask, dependencies []string) error {
	o.mu.Lock()
	defer o.mu.Unlock()

	tx, err := o.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	// Determine initial status based on dependencies
	status := "READY"
	if len(dependencies) > 0 {
		status = "PENDING"
	}

	// payloadBytes, _ := json.Marshal(payloadMap) // removed unused
	var query string
	if o.db.IsSQLite() {
		query = `
			INSERT INTO shared_tasks (id, mission_id, title, description, status, priority, created_at, updated_at)
			VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		`
	} else {
		query = `
			INSERT INTO shared_tasks (id, mission_id, title, description, status, priority)
			VALUES ($1, $2, $3, $4, $5, $6)
		`
	}

	_, err = tx.Exec(ctx, query, task.ID, task.MissionID, task.Title, task.Description, status, task.Priority)
	if err != nil {
		return fmt.Errorf("insert task: %w", err)
	}

	// Insert dependencies
	for _, depID := range dependencies {
		_, err = tx.Exec(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)", task.ID, depID)
		if err != nil {
			return fmt.Errorf("insert dependency %s for %s: %w", depID, task.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("commit tx: %w", err)
	}

	task.Status = status
	if o.hub != nil {
		o.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{
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

func (o *taskOrchestrator) AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*SharedTask, error) {
	o.mu.Lock()
	defer o.mu.Unlock()

	tx, err := o.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	// In SQLite we can't use SKIP LOCKED natively across multiple processes,
	// but since we are in Standalone we use o.mu.Lock() across goroutines.
	// In Cloud we might use rueidis distributed lock, but we must also ensure we use
	// a proper lock mechanism. Wait, if we use a DB lock inside the transaction we can fetch a READY task.
	var task SharedTask
	var query string
	if o.db.IsSQLite() {
		query = `
			SELECT id, mission_id, title, description, status, priority, created_at, updated_at
			FROM shared_tasks
			WHERE status = 'READY'
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
		`
	} else {
		query = `
			SELECT id, mission_id, title, description, status, priority, created_at, updated_at
			FROM shared_tasks
			WHERE status = 'READY'
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
	}

	err = tx.QueryRow(ctx, query).Scan(
		&task.ID, &task.MissionID, &task.Title, &task.Description, &task.Status, &task.Priority, &task.CreatedAt, &task.UpdatedAt,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, fmt.Errorf("query ready task: %w", err)
	}

	if o.redisClient != nil {
		// Acquire distributed lock for safety in multi-tenant environment.
		// Use TaskID as the lock key
		lockKey := "lock:shared_task:" + task.ID
		cmd := o.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Px(time.Minute * 5).Build()
		err = o.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return nil, nil // Locked by someone else
			}
			return nil, fmt.Errorf("redis set nx: %w", err)
		}
	}

	updateQuery := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND status = 'READY'
	`
	res, err := tx.Exec(ctx, updateQuery, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("update task to IN_PROGRESS: %w", err)
	}

	if res == 0 {
		return nil, nil // Already picked up
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("commit tx: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = agentID

	if o.hub != nil {
		o.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{
			"action":    "CLAIM",
			"agent_id":  agentID,
			"status":    task.Status,
		})
	}

	return &task, nil
}

func (o *taskOrchestrator) CompleteTask(ctx context.Context, taskID string, agentID string, result string) error {
	o.mu.Lock()
	defer o.mu.Unlock()

	tx, err := o.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	// Update task status to COMPLETED
	updateQuery := `
		UPDATE shared_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND assigned_agent_id = $2 AND status = 'IN_PROGRESS'
	`
	res, err := tx.Exec(ctx, updateQuery, taskID, agentID)
	if err != nil {
		return fmt.Errorf("complete task %s: %w", taskID, err)
	}
	if res == 0 {
		return fmt.Errorf("task %s not found or not in progress by agent %s", taskID, agentID)
	}

	// Fetch task details for AutoDream
	var missionID, title, description string
	err = tx.QueryRow(ctx, "SELECT mission_id, title, description FROM shared_tasks WHERE id = $1", taskID).Scan(&missionID, &title, &description)
	if err != nil && !errors.Is(err, sql.ErrNoRows) {
		slog.Error("failed to get task details for autodream", "task", taskID, "error", err)
	}

	// Resolve dependencies
	// For any task that depends on this completed task, check if all its dependencies are now COMPLETED
	// If so, update its status to READY.
	depsQuery := `
		SELECT t.id
		FROM shared_tasks t
		JOIN task_dependencies td ON t.id = td.task_id
		WHERE td.depends_on_task_id = $1 AND t.status = 'PENDING'
	`
	rows, err := tx.Query(ctx, depsQuery, taskID)
	if err != nil {
		return fmt.Errorf("query dependents of %s: %w", taskID, err)
	}

	var dependentTaskIDs []string
	for rows.Next() {
		var dTaskID string
		if err := rows.Scan(&dTaskID); err == nil {
			dependentTaskIDs = append(dependentTaskIDs, dTaskID)
		}
	}
	rows.Close()

	for _, dTaskID := range dependentTaskIDs {
		// Check if ALL dependencies are completed
		allCompletedQuery := `
			SELECT COUNT(*)
			FROM task_dependencies td
			JOIN shared_tasks st ON td.depends_on_task_id = st.id
			WHERE td.task_id = $1 AND st.status != 'COMPLETED'
		`
		var pendingDeps int
		if err := tx.QueryRow(ctx, allCompletedQuery, dTaskID).Scan(&pendingDeps); err != nil {
			continue
		}
		if pendingDeps == 0 {
			// Mark as READY
			_, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = 'READY', updated_at = CURRENT_TIMESTAMP WHERE id = $1", dTaskID)
			if err != nil {
				slog.Error("failed to mark dependent task as ready", "task", dTaskID, "error", err)
			} else {
				// Broadcast state change to READY
				if o.hub != nil {
					o.hub.PublishTaskBroadcast(dTaskID, map[string]interface{}{
						"action":   "READY",
						"agent_id": "",
						"status":   "READY",
					})
				}
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("commit tx: %w", err)
	}

	// Unlock redis distributed lock
	if o.redisClient != nil {
		lockKey := "lock:shared_task:" + taskID
		o.redisClient.Do(ctx, o.redisClient.B().Del().Key(lockKey).Build())
	}

	if o.hub != nil {
		o.hub.PublishTaskBroadcast(taskID, map[string]interface{}{
			"action":   "COMPLETE",
			"agent_id": agentID,
			"status":   "COMPLETED",
		})
	}

	// Trigger AutoDream Hook asynchronously
	if o.llmClient != nil && missionID != "" {
		go func() {
			bgCtx, cancel := context.WithTimeout(context.Background(), 2*time.Minute)
			defer cancel()

			content := fmt.Sprintf("Task: %s\nDescription: %s\nResult: %s", title, description, result)
			emb, err := o.llmClient.GenerateEmbedding(bgCtx, content)
			if err != nil {
				slog.Error("autodream hook embedding generation failed", "task", taskID, "error", err)
				return
			}

			// If not zero vector
			if len(emb) > 0 {
				if o.db.IsSQLite() {
					// Encode as JSON for SQLite text fallback
					embJSON, _ := json.Marshal(emb)
					_, err = o.db.Exec(bgCtx, `
						INSERT INTO autodream_memories (id, content, embedding, source_mission_id, consolidated_at)
						VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
					`, taskID+"-mem", content, string(embJSON), missionID)
				} else {
					_, err = o.db.Exec(bgCtx, `
						INSERT INTO autodream_memories (id, content, embedding, source_mission_id)
						VALUES ($1, $2, $3, $4)
					`, taskID+"-mem", content, fmt.Sprintf("%v", emb), missionID)
				}

				if err != nil {
					slog.Error("autodream hook db insert failed", "task", taskID, "error", err)
				}
			}
		}()
	}

	return nil
}
