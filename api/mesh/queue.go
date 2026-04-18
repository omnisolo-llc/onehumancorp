package mesh

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"
)

// SubAgentTask represents a task in the ohc_tasks.sub_agent_queue
type SubAgentTask struct {
	ID           string          `json:"id"`
	ParentTaskID string          `json:"parent_task_id"`
	Payload      json.RawMessage `json:"payload"`
	Status       string          `json:"status"`
	WorkerID     *string         `json:"worker_id"`
	CreatedAt    time.Time       `json:"created_at"`
	UpdatedAt    time.Time       `json:"updated_at"`
}

// QueueOrchestrator manages distributed queuing via Redis and PostgreSQL
type QueueOrchestrator struct {
	db       *sql.DB
	mesh     *TeammateMesh
	isSQLite bool
}

// NewQueueOrchestrator creates a new queue orchestrator
func NewQueueOrchestrator(db *sql.DB, mesh *TeammateMesh, isSQLite bool) *QueueOrchestrator {
	return &QueueOrchestrator{
		db:       db,
		mesh:     mesh,
		isSQLite: isSQLite,
	}
}

// EnqueueSubTask adds a task to the sub-agent queue
func (q *QueueOrchestrator) EnqueueSubTask(ctx context.Context, parentTaskID string, payload json.RawMessage) (string, error) {
	if q.db == nil {
		return "", fmt.Errorf("db connection is nil")
	}

	query := `
		INSERT INTO ohc_tasks.sub_agent_queue (parent_task_id, payload, status)
		VALUES ($1, $2, 'QUEUED')
		RETURNING id
	`
	var taskID string
	err := q.db.QueryRowContext(ctx, query, parentTaskID, payload).Scan(&taskID)
	if err != nil {
		return "", fmt.Errorf("failed to enqueue sub-task: %w", err)
	}

	if q.mesh != nil {
		event := map[string]string{
			"type":    "new_sub_task",
			"task_id": taskID,
		}
		data, _ := json.Marshal(event)
		q.mesh.Publish(ctx, MeshMessage{
			Topic:   "kairos:sub_tasks",
			Payload: data,
		})
	}

	return taskID, nil
}

// ClaimSubTask claims a pending sub-task for a worker
func (q *QueueOrchestrator) ClaimSubTask(ctx context.Context, workerID string) (*SubAgentTask, error) {
	if q.db == nil {
		return nil, fmt.Errorf("db connection is nil")
	}

	var query string
	if q.isSQLite {
		query = `
			UPDATE ohc_tasks.sub_agent_queue
			SET status = 'IN_PROGRESS', worker_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id FROM ohc_tasks.sub_agent_queue WHERE status = 'QUEUED' LIMIT 1
			)
			RETURNING id, parent_task_id, payload, status, worker_id, created_at, updated_at
		`
	} else {
		query = `
			UPDATE ohc_tasks.sub_agent_queue
			SET status = 'IN_PROGRESS', worker_id = $1, updated_at = NOW()
			WHERE id = (
				SELECT id FROM ohc_tasks.sub_agent_queue WHERE status = 'QUEUED' FOR UPDATE SKIP LOCKED LIMIT 1
			)
			RETURNING id, parent_task_id, payload, status, worker_id, created_at, updated_at
		`
	}

	row := q.db.QueryRowContext(ctx, query, workerID)
	var task SubAgentTask
	err := row.Scan(
		&task.ID,
		&task.ParentTaskID,
		&task.Payload,
		&task.Status,
		&task.WorkerID,
		&task.CreatedAt,
		&task.UpdatedAt,
	)

	if err == sql.ErrNoRows {
		return nil, nil // No pending tasks
	}
	if err != nil {
		return nil, fmt.Errorf("failed to claim sub-task: %w", err)
	}

	return &task, nil
}

// CompleteSubTask marks a sub-task as completed
func (q *QueueOrchestrator) CompleteSubTask(ctx context.Context, taskID, workerID string) error {
	if q.db == nil {
		return fmt.Errorf("db connection is nil")
	}

	query := `
		UPDATE ohc_tasks.sub_agent_queue
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND worker_id = $2 AND status = 'IN_PROGRESS'
	`
	res, err := q.db.ExecContext(ctx, query, taskID, workerID)
	if err != nil {
		return fmt.Errorf("failed to complete sub-task: %w", err)
	}

	rows, err := res.RowsAffected()
	if err != nil {
		return fmt.Errorf("failed to check rows affected: %w", err)
	}
	if rows == 0 {
		return fmt.Errorf("sub-task not found, not in progress, or not assigned to worker")
	}

	return nil
}

// EnqueueMission adds a new mission to the ohc_tasks.mission_queue
func (q *QueueOrchestrator) EnqueueMission(ctx context.Context, title, priority string, payload json.RawMessage) (string, error) {
	if q.db == nil {
		return "", fmt.Errorf("db connection is nil")
	}

	query := `
		INSERT INTO ohc_tasks.mission_queue (title, priority, payload)
		VALUES ($1, $2, $3)
		RETURNING mission_id
	`
	var missionID string
	err := q.db.QueryRowContext(ctx, query, title, priority, payload).Scan(&missionID)
	if err != nil {
		return "", fmt.Errorf("failed to enqueue mission: %w", err)
	}

	if q.mesh != nil {
		event := map[string]string{
			"type":       "new_mission",
			"mission_id": missionID,
		}
		data, _ := json.Marshal(event)
		q.mesh.Publish(ctx, MeshMessage{
			Topic:   "kairos:missions",
			Payload: data,
		})
	}

	return missionID, nil
}

// CompleteMission marks an IN_PROGRESS mission as DONE
func (q *QueueOrchestrator) CompleteMission(ctx context.Context, missionID, agentID string) error {
	if q.db == nil {
		return fmt.Errorf("db connection is nil")
	}

	var query string
	if q.isSQLite {
		query = `
			UPDATE ohc_tasks.mission_queue
			SET status = 'DONE',
			    updated_at = CURRENT_TIMESTAMP
			WHERE mission_id = $1 AND assigned_agent = $2 AND status = 'IN_PROGRESS'
		`
	} else {
		query = `
			UPDATE ohc_tasks.mission_queue
			SET status = 'DONE',
			    updated_at = NOW()
			WHERE mission_id = $1 AND assigned_agent = $2 AND status = 'IN_PROGRESS'
		`
	}

	res, err := q.db.ExecContext(ctx, query, missionID, agentID)
	if err != nil {
		return fmt.Errorf("failed to complete mission: %w", err)
	}

	rows, err := res.RowsAffected()
	if err != nil {
		return fmt.Errorf("failed to check rows affected: %w", err)
	}
	if rows == 0 {
		return fmt.Errorf("mission not found, not in progress, or not assigned to agent")
	}

	return nil
}

// Mission represents a task in the ohc_tasks.mission_queue
type Mission struct {
	MissionID     string          `json:"mission_id"`
	Title         string          `json:"title"`
	Status        string          `json:"status"`
	AssignedAgent *string         `json:"assigned_agent"`
	Priority      string          `json:"priority"`
	Payload       json.RawMessage `json:"payload"`
	CreatedAt     time.Time       `json:"created_at"`
	UpdatedAt     time.Time       `json:"updated_at"`
}

// ClaimMission attempts to claim a queued mission for the given agent
func (q *QueueOrchestrator) ClaimMission(ctx context.Context, agentID string) (*Mission, error) {
	if q.db == nil {
		return nil, fmt.Errorf("db connection is nil")
	}

	tx, err := q.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()

	var query string
	if q.isSQLite {
		query = `
			UPDATE ohc_tasks.mission_queue
			SET status = 'IN_PROGRESS',
			    assigned_agent = $1,
			    updated_at = CURRENT_TIMESTAMP
			WHERE mission_id = (
			    SELECT mission_id
			    FROM ohc_tasks.mission_queue
			    WHERE status = 'QUEUED'
			    LIMIT 1
			)
			RETURNING mission_id, title, status, assigned_agent, priority, payload, created_at, updated_at
		`
	} else {
		query = `
			UPDATE ohc_tasks.mission_queue
			SET status = 'IN_PROGRESS',
			    assigned_agent = $1,
			    updated_at = NOW()
			WHERE mission_id = (
			    SELECT mission_id
			    FROM ohc_tasks.mission_queue
			    WHERE status = 'QUEUED'
			    FOR UPDATE SKIP LOCKED
			    LIMIT 1
			)
			RETURNING mission_id, title, status, assigned_agent, priority, payload, created_at, updated_at
		`
	}

	row := tx.QueryRowContext(ctx, query, agentID)

	var m Mission
	if err := row.Scan(
		&m.MissionID,
		&m.Title,
		&m.Status,
		&m.AssignedAgent,
		&m.Priority,
		&m.Payload,
		&m.CreatedAt,
		&m.UpdatedAt,
	); err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No task found
		}
		return nil, fmt.Errorf("failed to scan claimed mission: %w", err)
	}

	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return &m, nil
}
