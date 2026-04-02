package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"os"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
	"github.com/redis/rueidis"
)

// TaskOrchestrator manages task queues and dependencies.
type TaskOrchestrator struct {
	db          db.Provider
	redisClient rueidis.Client
	hub         *CentrifugeNode
	mu          sync.Mutex // For standalone mode TOCTOU prevention
	localEvent  *sync.Cond
}

// NewTaskOrchestrator creates a new orchestrator.
func NewTaskOrchestrator(provider db.Provider, hub *CentrifugeNode) *TaskOrchestrator {
	to := &TaskOrchestrator{
		db:  provider,
		hub: hub,
	}
	to.localEvent = sync.NewCond(&to.mu)

	if os.Getenv("OHC_MULTITENANT") == "true" {
		redisURL := os.Getenv("REDIS_URL")
		if redisURL != "" {
			opts, err := rueidis.ParseURL(redisURL)
			if err == nil {
				c, err := rueidis.NewClient(opts)
				if err == nil {
					to.redisClient = c
				}
			}
		}
	}
	return to
}

// EnqueueTask adds a new task to the queue and evaluates its readiness.
func (to *TaskOrchestrator) EnqueueTask(ctx context.Context, task models.Task) error {
	to.mu.Lock()
	defer to.mu.Unlock()

	status := "PENDING"
	if task.Status != "" {
		status = task.Status
	}

	_, err := to.db.Exec(ctx, `
		INSERT INTO shared_tasks (id, mission_id, title, description, status, priority, payload, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`, task.ID, task.MissionID, task.Title, task.Description, status, task.Priority, task.Payload)

	if err != nil {
		return fmt.Errorf("failed to insert task: %w", err)
	}

	if to.redisClient == nil {
		to.localEvent.Broadcast()
	}

	return nil
}

// EnqueueTaskWithDependencies inserts a task and its dependencies.
func (to *TaskOrchestrator) EnqueueTaskWithDependencies(ctx context.Context, task models.Task, deps []string) error {
	to.mu.Lock()
	defer to.mu.Unlock()

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	status := "PENDING"
	if len(deps) == 0 {
		status = "READY"
	}

	_, err = tx.Exec(ctx, `
		INSERT INTO shared_tasks (id, mission_id, title, description, status, priority, payload, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`, task.ID, task.MissionID, task.Title, task.Description, status, task.Priority, task.Payload)

	if err != nil {
		return fmt.Errorf("failed to insert task: %w", err)
	}

	for _, depID := range deps {
		_, err = tx.Exec(ctx, `
			INSERT INTO task_dependencies (task_id, depends_on_task_id)
			VALUES ($1, $2)
		`, task.ID, depID)
		if err != nil {
			return fmt.Errorf("failed to insert task dependency: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if to.redisClient == nil {
		to.localEvent.Broadcast()
	}

	return nil
}

// AcquireReadyTask finds a READY task and locks it for the agent.
func (to *TaskOrchestrator) AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*models.Task, error) {
	if to.redisClient != nil {
		return to.acquireTaskCloud(ctx, agentID, capabilities)
	}
	return to.acquireTaskStandalone(ctx, agentID, capabilities)
}

func (to *TaskOrchestrator) acquireTaskCloud(ctx context.Context, agentID string, capabilities []string) (*models.Task, error) {
	// Find a ready task
	rows, err := to.db.Query(ctx, `
		SELECT id, mission_id, title, description, status, priority, payload
		FROM shared_tasks
		WHERE status = 'READY'
		ORDER BY priority ASC, created_at ASC
		LIMIT 10
	`)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var candidateIDs []string
	tasks := make(map[string]*models.Task)
	for rows.Next() {
		var t models.Task
		var payload sql.NullString
		if err := rows.Scan(&t.ID, &t.MissionID, &t.Title, &t.Description, &t.Status, &t.Priority, &payload); err != nil {
			continue
		}
		if payload.Valid {
			t.Payload = payload.String
		}
		candidateIDs = append(candidateIDs, t.ID)
		tasks[t.ID] = &t
	}

	for _, id := range candidateIDs {
		// Attempt to acquire distributed lock
		lockKey := fmt.Sprintf("task_lock:%s", id)
		cmd := to.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Px(10 * time.Minute).Build()
		err := to.redisClient.Do(ctx, cmd).Error()
		if err == nil { // Lock acquired
			// Update DB state
			_, err = to.db.Exec(ctx, `
				UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
				WHERE id = $2 AND status = 'READY'
			`, agentID, id)
			if err == nil {
				t := tasks[id]
				t.Status = "IN_PROGRESS"
				t.AssignedAgentID = agentID
				return t, nil
			}
			// If DB update failed, release lock
			delCmd := to.redisClient.B().Del().Key(lockKey).Build()
			to.redisClient.Do(ctx, delCmd)
		}
	}

	return nil, sql.ErrNoRows
}

func (to *TaskOrchestrator) acquireTaskStandalone(ctx context.Context, agentID string, capabilities []string) (*models.Task, error) {
	to.mu.Lock()
	defer to.mu.Unlock()

	// Find a ready task
	row := to.db.QueryRow(ctx, `
		SELECT id, mission_id, title, description, status, priority, payload
		FROM shared_tasks
		WHERE status = 'READY'
		ORDER BY priority ASC, created_at ASC
		LIMIT 1
	`)

	var t models.Task
	var payload sql.NullString
	err := row.Scan(&t.ID, &t.MissionID, &t.Title, &t.Description, &t.Status, &t.Priority, &payload)
	if err != nil {
		return nil, err
	}
	if payload.Valid {
		t.Payload = payload.String
	}

	// Update to IN_PROGRESS
	_, err = to.db.Exec(ctx, `
		UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
	`, agentID, t.ID)
	if err != nil {
		return nil, err
	}

	t.Status = "IN_PROGRESS"
	t.AssignedAgentID = agentID
	return &t, nil
}

// CompleteTask marks a task as COMPLETED and evaluates dependencies.
func (to *TaskOrchestrator) CompleteTask(ctx context.Context, taskID string, result string) error {
	to.mu.Lock()
	defer to.mu.Unlock()

	tx, err := to.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// Update task status
	_, err = tx.Exec(ctx, `
		UPDATE shared_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`, taskID)
	if err != nil {
		return err
	}

	// Find tasks that were dependent on this task and check if they are now READY
	// In SQLite we use standard IN clause or fallback iteration, but here we can just do a join or subquery
	// to see if ALL dependencies for a given task are COMPLETED.

	// SQLite/Postgres compatible way to check dependencies
	rows, err := tx.Query(ctx, `
		SELECT t.id
		FROM shared_tasks t
		JOIN task_dependencies td ON t.id = td.task_id
		WHERE td.depends_on_task_id = $1 AND t.status = 'PENDING'
	`, taskID)
	if err != nil {
		return err
	}
	defer rows.Close()

	var dependentTasks []string
	for rows.Next() {
		var id string
		if err := rows.Scan(&id); err == nil {
			dependentTasks = append(dependentTasks, id)
		}
	}

	for _, depTaskID := range dependentTasks {
		// Check if all dependencies are COMPLETED
		var count int
		err := tx.QueryRow(ctx, `
			SELECT COUNT(*)
			FROM task_dependencies td
			JOIN shared_tasks st ON td.depends_on_task_id = st.id
			WHERE td.task_id = $1 AND st.status != 'COMPLETED'
		`, depTaskID).Scan(&count)

		if err == nil && count == 0 {
			// All dependencies completed, mark as READY
			_, err = tx.Exec(ctx, `
				UPDATE shared_tasks SET status = 'READY', updated_at = CURRENT_TIMESTAMP
				WHERE id = $1
			`, depTaskID)
			if err != nil {
				return err
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	// Mesh Integration
	event := map[string]interface{}{
		"action":   "TASK_COMPLETED",
		"agent_id": "orchestrator", // We don't have agentID here, but we can set it
		"status":   "COMPLETED",
		"task_id":  taskID,
	}
	eventData, _ := json.Marshal(event)

	if to.redisClient != nil && to.hub != nil {
		cmd := to.redisClient.B().Publish().Channel("mesh:tasks").Message(string(eventData)).Build()
		go to.redisClient.Do(context.Background(), cmd)
	} else {
		// Standalone mode: signal local channel
		to.localEvent.Broadcast()
	}

	// AutoDream Hook
	go to.triggerAutoDreamHook(taskID, result)

	return nil
}

func (to *TaskOrchestrator) triggerAutoDreamHook(taskID string, result string) {
	// Trigger background goroutine to generate summary embedding via CachedMinimaxClient
	// Since we don't have CachedMinimaxClient initialized directly here, we could instantiate it or look up the DB
	// We'll query task context and insert embedding into autodream_memories.
	ctx := context.Background()

	var missionID, title, description string
	err := to.db.QueryRow(ctx, `SELECT mission_id, title, description FROM shared_tasks WHERE id = $1`, taskID).Scan(&missionID, &title, &description)
	if err != nil {
		return // log error
	}

	// We instantiate CachedMinimaxClient. For simplicity, we just use the default initialized one or a new one.
	// The Minimax API URL and Key are fetched from env vars if needed, or we construct a new client.
	baseClient := NewMinimaxClient(os.Getenv("MINIMAX_API_KEY"))
	client := NewCachedMinimaxClient(baseClient, to.db, to.redisClient)

	contentToEmbed := fmt.Sprintf("Task: %s\nDescription: %s\nResult: %s", title, description, result)
	embedding, err := client.GenerateEmbedding(ctx, contentToEmbed)
	if err != nil {
		return // log error
	}

	// Insert into autodream_memories (fallback for SQLite: convert VECTOR to BLOB or store JSON string)
	// We handle BLOB fallback inside db wrapper if needed, or simply pass the []float32.
	// Since SQLite doesn't have pgvector natively, we often store it as JSON or BLOB.
	embeddingBytes, _ := json.Marshal(embedding)
	_, _ = to.db.Exec(ctx, `
		INSERT INTO autodream_memories (content, embedding, source_mission_id, consolidated_at)
		VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
	`, contentToEmbed, string(embeddingBytes), missionID)
}

// FailTask marks a task as FAILED.
func (to *TaskOrchestrator) FailTask(ctx context.Context, taskID string, reason string) error {
	to.mu.Lock()
	defer to.mu.Unlock()

	_, err := to.db.Exec(ctx, `
		UPDATE shared_tasks SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`, taskID)
	return err
}

// BlockTask marks a task as BLOCKED.
func (to *TaskOrchestrator) BlockTask(ctx context.Context, taskID string, reason string) error {
	to.mu.Lock()
	defer to.mu.Unlock()

	_, err := to.db.Exec(ctx, `
		UPDATE shared_tasks SET status = 'BLOCKED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`, taskID)
	return err
}
