package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// Task dependency model
type TaskDependency struct {
	TaskID          string `json:"task_id"`
	DependsOnTaskID string `json:"depends_on_task_id"`
}

// TaskOrchestrator interface matches the requested methods
type TaskOrchestrator interface {
	EnqueueTask(ctx context.Context, task *SharedTask) error
	AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*SharedTask, error)
	CompleteTask(ctx context.Context, taskID string, result string) error
}

type DefaultTaskOrchestrator struct {
	db          db.Provider
	redisClient rueidis.Client
	hub         *CentrifugeNode
	mesh        TeammateMesh
	autoDream   *AutoDreamWorker
	minimax     MinimaxClient

	// For standalone mode local state coordination
	mu    sync.Mutex
	locks map[string]time.Time
}

func NewTaskOrchestrator(provider db.Provider, redisClient rueidis.Client, hub *CentrifugeNode, mesh TeammateMesh, autoDream *AutoDreamWorker, minimax MinimaxClient) *DefaultTaskOrchestrator {
	return &DefaultTaskOrchestrator{
		db:          provider,
		redisClient: redisClient,
		hub:         hub,
		mesh:        mesh,
		autoDream:   autoDream,
		minimax:     minimax,
		locks:       make(map[string]time.Time),
	}
}

// EnqueueTask adds a task to the database, including any dependencies it might have.
func (to *DefaultTaskOrchestrator) EnqueueTask(ctx context.Context, task *SharedTask) error {
	tx, err := to.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// Since we are enqueueing, we should have an ID. If not, generate one.
	if task.ID == "" {
		task.ID = generateID()
	}
	if task.Status == "" {
		task.Status = "PENDING"
	}

	depsBytes, _ := json.Marshal(task.Dependencies)
	depsJSON := string(depsBytes)

	var parentPlanIDPtr *string
	if task.ParentPlanID != "" {
		parentPlanIDPtr = &task.ParentPlanID
	}

	var query string
	if to.db.IsSQLite() {
		query = `
			INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, dependencies, title, payload, status, created_at, updated_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		`
	} else {
		query = `
			INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, dependencies, title, payload, status)
			VALUES ($1, $2, $3, $4, $5, $6, $7)
		`
	}

	_, err = tx.Exec(ctx, query, task.ID, task.MissionID, parentPlanIDPtr, depsJSON, task.Title, task.Payload, task.Status)
	if err != nil {
		return fmt.Errorf("failed to insert task: %w", err)
	}

	// Insert dependencies if any
	for _, depID := range task.Dependencies {
		_, err = tx.Exec(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)", task.ID, depID)
		if err != nil {
			return fmt.Errorf("failed to insert dependency %s for task %s: %w", depID, task.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
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
	return nil
}

// AcquireReadyTask finds a PENDING task whose dependencies are all COMPLETED.
// It uses rueidis lock for Cloud mode and DB transaction for Standalone mode.
func (to *DefaultTaskOrchestrator) AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*SharedTask, error) {
	to.mu.Lock()
	defer to.mu.Unlock() // Single lock to prevent TOCTOU in local state

	// Clean up old local locks
	now := time.Now()
	for k, v := range to.locks {
		if now.After(v) {
			delete(to.locks, k) // Explicitly delete instead of nil
		}
	}

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var query string
	if to.db.IsSQLite() {
		// SQLite check
		query = `
			SELECT t.id, t.mission_id, t.parent_plan_id, t.dependencies, t.title, t.payload, t.status
			FROM swarm_tasks t
			WHERE t.status = 'PENDING'
			  AND NOT EXISTS (
				SELECT 1 FROM task_dependencies td
				JOIN swarm_tasks dep ON td.depends_on_task_id = dep.id
				WHERE td.task_id = t.id AND dep.status != 'COMPLETED'
			  )
			ORDER BY t.created_at ASC
			LIMIT 1
		`
	} else {
		// Postgres with SKIP LOCKED
		query = `
			SELECT t.id, t.mission_id, t.parent_plan_id, t.dependencies, t.title, t.payload, t.status
			FROM swarm_tasks t
			WHERE t.status = 'PENDING'
			  AND NOT EXISTS (
				SELECT 1 FROM task_dependencies td
				JOIN swarm_tasks dep ON td.depends_on_task_id = dep.id
				WHERE td.task_id = t.id AND dep.status != 'COMPLETED'
			  )
			ORDER BY t.created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
	}

	var task SharedTask
	var pID sql.NullString
	var deps string

	err = tx.QueryRow(ctx, query).Scan(
		&task.ID, &task.MissionID, &pID, &deps, &task.Title, &task.Payload, &task.Status,
	)

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No ready tasks
		}
		return nil, err
	}

	if pID.Valid {
		task.ParentPlanID = pID.String
	}
	_ = json.Unmarshal([]byte(deps), &task.Dependencies)

	// Try acquiring lock
	if to.redisClient != nil {
		lockKey := "lock:task:" + task.ID
		cmd := to.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Px(time.Minute).Build()
		err = to.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return nil, nil // Locked by someone else
			}
			return nil, fmt.Errorf("redis lock failed: %w", err)
		}
	} else {
		// Standalone mode lock
		if _, exists := to.locks[task.ID]; exists {
			return nil, nil // Locked locally
		}
		to.locks[task.ID] = time.Now().Add(time.Minute)
	}

	// Update status
	_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1 WHERE id = $2", agentID, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = agentID

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

// CompleteTask marks a task as COMPLETED, unlocks it, and triggers AutoDream.
func (to *DefaultTaskOrchestrator) CompleteTask(ctx context.Context, taskID string, result string) error {
	to.mu.Lock()
	if _, ok := to.locks[taskID]; ok {
		delete(to.locks, taskID) // Explicitly delete
	}
	to.mu.Unlock()

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var task SharedTask
	err = tx.QueryRow(ctx, "SELECT id, mission_id, title FROM swarm_tasks WHERE id = $1", taskID).Scan(
		&task.ID, &task.MissionID, &task.Title,
	)
	if err != nil {
		return fmt.Errorf("task not found: %w", err)
	}

	rowsAffected, err := tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1", taskID)
	if err != nil {
		return err
	}
	if rowsAffected == 0 {
		return errors.New("failed to complete task")
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	// Broadcast
	if to.mesh != nil {
		_ = to.mesh.BroadcastTask(ctx, Task{
			AgentID: "",
			Action:  "COMPLETE",
			Status:  "COMPLETED",
			TaskID:  taskID,
		})
	} else if to.hub != nil {
		to.hub.PublishTaskBroadcast(taskID, map[string]interface{}{
			"action": "COMPLETE",
			"status": "COMPLETED",
		})
	}

	if to.redisClient != nil {
		cmd := to.redisClient.B().Del().Key("lock:task:" + taskID).Build()
		_ = to.redisClient.Do(ctx, cmd)
	}

	// Trigger AutoDream ingestion
	if to.autoDream != nil && to.minimax != nil {
		go func() {
			bgCtx, cancel := context.WithTimeout(context.Background(), 2*time.Minute)
			defer cancel()

			summaryPrompt := fmt.Sprintf("Summarise task '%s' (Mission: %s) with result: %s", task.Title, task.MissionID, result)
			summary, err := to.minimax.Reason(bgCtx, summaryPrompt)
			if err != nil {
				slog.Error("TaskOrchestrator: failed to summarize task for AutoDream", "err", err)
				return
			}

			embedding, err := to.minimax.GenerateEmbedding(bgCtx, summary)
			if err != nil {
				slog.Error("TaskOrchestrator: failed to generate embedding for AutoDream", "err", err)
				return
			}

			embeddingBytes, _ := json.Marshal(embedding)
			embeddingStr := string(embeddingBytes)

			if to.db.IsSQLite() {
				// We don't have VECTOR in SQLite natively without extensions but schema might use text fallback.
				// Wait, the swarm_long_term_memory schema fallback to BLOB or TEXT.
				// We need to just inject it.
				// Our mission says: insert it into `autodream_memories` (or `swarm_long_term_memory`)
				query := "INSERT INTO swarm_long_term_memory (topic, summary, embedding) VALUES (?, ?, ?)"
				_, _ = to.db.Exec(bgCtx, query, "Task Completion: "+task.Title, summary, embeddingStr)
			} else {
				// Postgres pgvector
				query := "INSERT INTO swarm_long_term_memory (topic, summary, embedding) VALUES ($1, $2, $3::vector)"
				_, _ = to.db.Exec(bgCtx, query, "Task Completion: "+task.Title, summary, embeddingStr)
			}
			slog.Info("TaskOrchestrator: successfully recorded task memory to AutoDream", "task", taskID)
		}()
	}

	return nil
}
