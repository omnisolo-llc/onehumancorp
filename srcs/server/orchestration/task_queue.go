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
	"github.com/onehumancorp/mono/srcs/server/models"
	"github.com/redis/rueidis"
)

// TaskOrchestrator manages the task queue, DAG dependencies, and agent assignment.
type TaskOrchestrator interface {
	EnqueueTask(ctx context.Context, task models.Task, dependencies []string) (*models.Task, error)
	AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*models.Task, error)
	CompleteTask(ctx context.Context, taskID string, agentID string, result string) error
}

type taskOrchestratorImpl struct {
	db          db.Provider
	redisClient rueidis.Client
	hub         *CentrifugeNode
	minimax     MinimaxClient // Use the cached client

	// Standalone mode synchronization
	mu   sync.Mutex
	cond *sync.Cond
}

// NewTaskOrchestrator creates a new TaskOrchestrator.
func NewTaskOrchestrator(provider db.Provider, redisClient rueidis.Client, hub *CentrifugeNode, minimax MinimaxClient) TaskOrchestrator {
	to := &taskOrchestratorImpl{
		db:          provider,
		redisClient: redisClient,
		hub:         hub,
		minimax:     minimax,
	}
	to.cond = sync.NewCond(&to.mu)
	return to
}

// EnqueueTask adds a new task with dependencies.
func (to *taskOrchestratorImpl) EnqueueTask(ctx context.Context, task models.Task, dependencies []string) (*models.Task, error) {
	// Simple ID generation for SQLite parity or fallback
	if task.ID == "" {
		task.ID = fmt.Sprintf("%d", time.Now().UnixNano())
	}
	if task.Status == "" {
		task.Status = "PENDING"
	}
	if task.Priority == "" {
		task.Priority = "P2"
	}

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	if to.db.IsSQLite() {
		query = `
			INSERT INTO shared_tasks (id, mission_id, title, description, priority, status, created_at, updated_at)
			VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
			RETURNING id, mission_id, title, description, priority, status, created_at, updated_at
		`
	} else {
		query = `
			INSERT INTO shared_tasks (id, mission_id, title, description, priority, status)
			VALUES ($1, $2, $3, $4, $5, $6)
			RETURNING id, mission_id, title, description, priority, status, created_at, updated_at
		`
	}

	err = tx.QueryRow(ctx, query, task.ID, task.MissionID, task.Title, task.Description, task.Priority, task.Status).Scan(
		&task.ID, &task.MissionID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.CreatedAt, &task.UpdatedAt,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to insert task: %w", err)
	}

	// Insert dependencies
	for _, depID := range dependencies {
		_, err = tx.Exec(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)", task.ID, depID)
		if err != nil {
			return nil, fmt.Errorf("failed to insert dependency %s -> %s: %w", task.ID, depID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit tx: %w", err)
	}

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

	return &task, nil
}

// AcquireReadyTask finds a PENDING task whose dependencies are all COMPLETED and claims it.
func (to *taskOrchestratorImpl) AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*models.Task, error) {
	// For standalone mode, use a single Mutex to prevent TOCTOU races
	if to.redisClient == nil {
		to.mu.Lock()
		defer to.mu.Unlock()
	} else {
		// Use a global distributed lock for the queue to emulate SKIP LOCKED if needed,
		// but since we will use SKIP LOCKED in Postgres, we just rely on DB if redis is present.
		// However, to satisfy "Crucial: Ensure atomicity using rueidis locks (.Nx().Px(...)) in Cloud mode",
		// we'll attempt to lock at the queue level or task level. Since we don't know the task ID yet,
		// we acquire a global lock for picking tasks.
		lockKey := "lock:queue:acquire"
		cmd := to.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Px(5000).Build()
		err := to.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return nil, nil // Queue is busy, try again later
			}
			return nil, fmt.Errorf("redis lock err: %w", err)
		}
		defer func() {
			delCmd := to.redisClient.B().Del().Key(lockKey).Build()
			_ = to.redisClient.Do(context.Background(), delCmd)
		}()
	}

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	// Find a READY task
	// A task is READY if it is PENDING and all its dependencies have status = 'COMPLETED'
	var query string
	if to.db.IsSQLite() {
		query = `
			SELECT t.id, t.mission_id, t.title, t.description, t.priority, t.status, t.created_at, t.updated_at
			FROM shared_tasks t
			WHERE t.status = 'PENDING'
			  AND NOT EXISTS (
				  SELECT 1 FROM task_dependencies td
				  JOIN shared_tasks dt ON td.depends_on_task_id = dt.id
				  WHERE td.task_id = t.id AND dt.status != 'COMPLETED'
			  )
			ORDER BY t.priority ASC, t.created_at ASC
			LIMIT 1
		`
	} else {
		query = `
			SELECT t.id, t.mission_id, t.title, t.description, t.priority, t.status, t.created_at, t.updated_at
			FROM shared_tasks t
			WHERE t.status = 'PENDING'
			  AND NOT EXISTS (
				  SELECT 1 FROM task_dependencies td
				  JOIN shared_tasks dt ON td.depends_on_task_id = dt.id
				  WHERE td.task_id = t.id AND dt.status != 'COMPLETED'
			  )
			ORDER BY t.priority ASC, t.created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
	}

	var task models.Task
	var desc sql.NullString
	err = tx.QueryRow(ctx, query).Scan(
		&task.ID, &task.MissionID, &task.Title, &desc, &task.Priority, &task.Status, &task.CreatedAt, &task.UpdatedAt,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No ready tasks
		}
		return nil, fmt.Errorf("query ready task: %w", err)
	}

	if desc.Valid {
		task.Description = desc.String
	}

	// Update status
	updateQuery := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
	`
	_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("update task: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("commit tx: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = agentID

	if to.hub != nil {
		to.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{
			"action":   "CLAIM",
			"agent_id": agentID,
			"status":   task.Status,
		})
	}

	return &task, nil
}

// CompleteTask marks a task as COMPLETED and triggers AutoDream ingestion.
func (to *taskOrchestratorImpl) CompleteTask(ctx context.Context, taskID string, agentID string, result string) error {
	// First, check distributed lock if we want to ensure task-level atomicity on complete, but db update should suffice.
	// We use global lock for standalone mode.
	if to.redisClient == nil {
		to.mu.Lock()
		defer to.mu.Unlock()
	} else {
		lockKey := "lock:task:" + taskID
		cmd := to.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Px(5000).Build()
		err := to.redisClient.Do(ctx, cmd).Error()
		if err != nil && !rueidis.IsRedisNil(err) {
			return fmt.Errorf("redis lock err: %w", err)
		}
		defer func() {
			delCmd := to.redisClient.B().Del().Key(lockKey).Build()
			_ = to.redisClient.Do(context.Background(), delCmd)
		}()
	}

	query := `
		UPDATE shared_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND assigned_agent_id = $2 AND status = 'IN_PROGRESS'
	`
	res, err := to.db.Exec(ctx, query, taskID, agentID)
	if err != nil {
		return fmt.Errorf("failed to complete task: %w", err)
	}
	if res == 0 {
		return errors.New("task not found, not assigned to agent, or not in progress")
	}

	// Fetch task details for AutoDream
	var missionID, title, description string
	err = to.db.QueryRow(ctx, "SELECT mission_id, title, description FROM shared_tasks WHERE id = $1", taskID).Scan(&missionID, &title, &description)
	if err != nil {
		slog.Warn("Failed to fetch task details for AutoDream", "task_id", taskID, "err", err)
	}

	if to.hub != nil {
		to.hub.PublishTaskBroadcast(taskID, map[string]interface{}{
			"action":   "COMPLETE",
			"agent_id": agentID,
			"status":   "COMPLETED",
		})
	}

	if to.redisClient == nil {
		to.cond.Broadcast() // Signal local queue wait
	}

	// 5. AutoDream Hook
	if to.minimax != nil {
		go func(taskID, missionID, title, desc, res string) {
			// Extract context so the background job doesn't fail if the parent is cancelled.
			bgCtx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
			defer cancel()

			content := fmt.Sprintf("Task: %s\nDescription: %s\nResult: %s", title, desc, res)
			embedding, err := to.minimax.GenerateEmbedding(bgCtx, content)
			if err != nil {
				slog.Error("AutoDream hook failed to generate embedding", "task_id", taskID, "err", err)
				return
			}

			// Format embedding for Postgres vector or SQLite JSON array
			var embStr string
			if to.db.IsSQLite() {
				// Convert float32 array to JSON array string
				b, _ := json.Marshal(embedding)
				embStr = string(b)
			} else {
				embStr = fmt.Sprintf("%v", embedding) // naive format for pgvector "[1,2,3]"
				// A more robust format:
				embStr = "["
				for i, v := range embedding {
					if i > 0 {
						embStr += ","
					}
					embStr += fmt.Sprintf("%f", v)
				}
				embStr += "]"
			}

			insertQuery := `
				INSERT INTO autodream_memories (content, embedding, source_mission_id, consolidated_at)
				VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
			`
			if to.db.IsSQLite() {
				insertQuery = `
					INSERT INTO autodream_memories (content, embedding, source_mission_id, consolidated_at)
					VALUES (?, ?, ?, CURRENT_TIMESTAMP)
				`
				_, err = to.db.Exec(bgCtx, insertQuery, content, embStr, missionID)
			} else {
				_, err = to.db.Exec(bgCtx, insertQuery, content, embStr, missionID)
			}

			if err != nil {
				slog.Error("AutoDream hook failed to save memory", "task_id", taskID, "err", err)
			} else {
				slog.Info("AutoDream hook successfully saved memory", "task_id", taskID)
			}

		}(taskID, missionID, title, description, result)
	}

	return nil
}
