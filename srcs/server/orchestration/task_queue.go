package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"sync"
	"time"
)

// TaskOrchestrator interface represents the required orchestration methods.
type TaskOrchestrator interface {
	EnqueueTask(ctx context.Context, task *SharedTask, dependencies []string) error
	AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*SharedTask, error)
	CompleteReadyTask(ctx context.Context, taskID string, result string) error
}

func (tm *TaskManager) EnqueueTask(ctx context.Context, task *SharedTask, dependencies []string) error {
	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	task.Status = "PENDING"

	// Insert task
	if tm.db.IsSQLite() {
		query := `
			INSERT INTO shared_tasks (id, mission_id, title, description, priority, status, created_at, updated_at)
			VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		`
		_, err = tx.Exec(ctx, query, task.ID, task.MissionID, task.Title, task.Description, task.Priority, task.Status)
	} else {
		query := `
			INSERT INTO shared_tasks (id, mission_id, title, description, priority, status)
			VALUES ($1, $2, $3, $4, $5, $6)
		`
		_, err = tx.Exec(ctx, query, task.ID, task.MissionID, task.Title, task.Description, task.Priority, task.Status)
	}
	if err != nil {
		return fmt.Errorf("failed to insert task: %w", err)
	}

	for _, depID := range dependencies {
		_, err = tx.Exec(ctx, `INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)`, task.ID, depID)
		if err != nil {
			return fmt.Errorf("failed to insert dependency: %w", err)
		}
	}

	// Verify if dependencies are already completed to avoid deadlock
	var pendingDeps int
	if len(dependencies) > 0 {
		err = tx.QueryRow(ctx, `
			SELECT count(*) FROM task_dependencies td
			JOIN shared_tasks st ON td.depends_on_task_id = st.id
			WHERE td.task_id = $1 AND st.status != 'COMPLETED'
		`, task.ID).Scan(&pendingDeps)
		if err != nil {
			return fmt.Errorf("failed to count pending dependencies: %w", err)
		}
	}

	if pendingDeps == 0 {
		_, err = tx.Exec(ctx, `UPDATE shared_tasks SET status = 'READY', updated_at = CURRENT_TIMESTAMP WHERE id = $1`, task.ID)
		if err != nil {
			return fmt.Errorf("failed to update task to READY: %w", err)
		}
		task.Status = "READY"
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	return nil
}

func (tm *TaskManager) AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*SharedTask, error) {
	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var task SharedTask
	var query string

	// Ensure we only pick tasks matching our capabilities. If none provided, any task is valid.
	capabilityFilter := ""
	var args []interface{}
	if len(capabilities) > 0 {
		capabilityFilter = ` AND (`
		for i, cap := range capabilities {
			if i > 0 {
				capabilityFilter += " OR "
			}
			args = append(args, "%"+cap+"%")
			capabilityFilter += fmt.Sprintf("description LIKE $%d", len(args))
		}
		capabilityFilter += `)`
	}

	if tm.db.IsSQLite() {
		query = `
			SELECT id, mission_id, title, description, priority, status, created_at, updated_at
			FROM shared_tasks
			WHERE status = 'READY' ` + capabilityFilter + `
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
		`
	} else {
		query = `
			SELECT id, mission_id, title, description, priority, status, created_at, updated_at
			FROM shared_tasks
			WHERE status = 'READY' ` + capabilityFilter + `
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
	}

	var desc sql.NullString
	err = tx.QueryRow(ctx, query, args...).Scan(
		&task.ID, &task.MissionID, &task.Title, &desc, &task.Priority, &task.Status, &task.CreatedAt, &task.UpdatedAt,
	)

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No ready tasks
		}
		return nil, err
	}
	if desc.Valid {
		task.Description = desc.String
	}

	// Update status securely checking for 'READY'
	res, err := tx.Exec(ctx, `UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND status = 'READY'`, agentID, task.ID)
	if err != nil {
		return nil, err
	}
	rowsAffected, _ := res.RowsAffected()
	if rowsAffected == 0 {
		return nil, nil // Concurrently claimed by another worker
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = agentID

	return &task, nil
}

// CompleteReadyTask acts like CompleteTask but with dependency logic
// as required by the TaskOrchestrator interface (to avoid conflict with existing CompleteTask).
// localEventChannels tracks local signals for Standalone mode Mesh Hand-offs.
var localEventChannels = make(map[string]chan string)
var localEventMu sync.Mutex

func GetLocalMeshChannel(topic string) chan string {
	localEventMu.Lock()
	defer localEventMu.Unlock()
	if ch, exists := localEventChannels[topic]; exists {
		return ch
	}
	ch := make(chan string, 100)
	localEventChannels[topic] = ch
	return ch
}

func (tm *TaskManager) CompleteReadyTask(ctx context.Context, taskID string, result string) error {
	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// Update task status
	res, err := tx.Exec(ctx, `UPDATE shared_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND status = 'IN_PROGRESS'`, taskID)
	if err != nil {
		return err
	}
	rowsAffected, _ := res.RowsAffected()
	if rowsAffected == 0 {
		return errors.New("task not found or not in progress")
	}

	// Update downstream dependent tasks
	rows, err := tx.Query(ctx, `SELECT task_id FROM task_dependencies WHERE depends_on_task_id = $1`, taskID)
	if err != nil {
		return err
	}
	defer rows.Close()

	var dependentTaskIDs []string
	for rows.Next() {
		var id string
		if err := rows.Scan(&id); err == nil {
			dependentTaskIDs = append(dependentTaskIDs, id)
		}
	}
	if err := rows.Err(); err != nil {
		return err
	}
	rows.Close()

	for _, depTaskID := range dependentTaskIDs {
		// Lock the dependent task to prevent race conditions when concurrent dependencies complete
		if !tm.db.IsSQLite() {
			_, err = tx.Exec(ctx, `SELECT id FROM shared_tasks WHERE id = $1 FOR NO KEY UPDATE`, depTaskID)
			if err != nil {
				continue // Skip if locked/deleted, it will fail gracefully or retry
			}
		}

		var pendingDeps int
		err := tx.QueryRow(ctx, `
			SELECT count(*) FROM task_dependencies td
			JOIN shared_tasks st ON td.depends_on_task_id = st.id
			WHERE td.task_id = $1 AND st.status != 'COMPLETED'
		`, depTaskID).Scan(&pendingDeps)

		if err == nil && pendingDeps == 0 {
			// All dependencies completed, mark as READY
			tx.Exec(ctx, `UPDATE shared_tasks SET status = 'READY', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND status = 'PENDING'`, depTaskID)
		}
	}

	var task SharedTask
	var desc sql.NullString
	err = tx.QueryRow(ctx, `SELECT id, mission_id, title, description FROM shared_tasks WHERE id = $1`, taskID).Scan(
		&task.ID, &task.MissionID, &task.Title, &desc,
	)
	if err != nil {
		return err
	}
	if desc.Valid {
		task.Description = desc.String
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	// Mesh Integration
	eventMap := map[string]string{
		"action":  "TASK_COMPLETED",
		"task_id": taskID,
		"result":  result,
	}
	eventBytes, err := json.Marshal(eventMap)
	if err == nil {
		eventPayload := string(eventBytes)
		if tm.redisClient != nil {
			// Cloud mode
			cmd := tm.redisClient.B().Publish().Channel("mesh:tasks").Message(eventPayload).Build()
			tm.redisClient.Do(ctx, cmd)
		} else {
			// Standalone mode local signaling
			ch := GetLocalMeshChannel("mesh:tasks")
			select {
			case ch <- eventPayload:
			default:
				// Non-blocking, drop if full
			}
		}
	}

	// AutoDream Hook
	if tm.minimax != nil {
		go func() {
			bgCtx, cancel := context.WithTimeout(context.Background(), 2*time.Minute)
			defer cancel()

			summary := fmt.Sprintf("Task: %s\nDesc: %s\nResult: %s", task.Title, task.Description, result)
			emb, err := tm.minimax.GenerateEmbedding(bgCtx, summary)
			if err == nil && len(emb) > 0 {
				var query string
				if tm.db.IsSQLite() {
					query = `INSERT INTO autodream_memories (content, source_mission_id, consolidated_at) VALUES ($1, $2, CURRENT_TIMESTAMP)`
					tm.db.Exec(bgCtx, query, summary, task.MissionID) // SQLite vector omit
				} else {
					query = `INSERT INTO autodream_memories (content, embedding, source_mission_id) VALUES ($1, $2, $3)`
					// Serialize embedding for pgvector
					embStr := "["
					for i, v := range emb {
						if i > 0 {
							embStr += ","
						}
						embStr += fmt.Sprintf("%g", v)
					}
					embStr += "]"
					tm.db.Exec(bgCtx, query, summary, embStr, task.MissionID)
				}
			}
		}()
	}

	return nil
}
