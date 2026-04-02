package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"log/slog"
	"os"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/go-redis/v9"
)

var (
	ErrTaskNotFound = errors.New("task not found")
	ErrTaskLocked   = errors.New("task is currently locked by another agent")
)

// SharedTask represents a collaborative task in the swarm.
type SharedTask struct {
	ID              string    `json:"id"`
	MissionID       string    `json:"mission_id"`
	Title           string    `json:"title"`
	Description     string    `json:"description"`
	AssignedAgentID string    `json:"assigned_agent_id"`
	Status          string    `json:"status"`
	Priority        string    `json:"priority"`
	CreatedAt       time.Time `json:"created_at"`
	UpdatedAt       time.Time `json:"updated_at"`
}

// TaskManager handles the shared task queue, supporting Redis locks in cloud mode
// and database row-level locking in standalone mode.
type TaskManager struct {
	dbClient db.Provider
	redisCli *redis.Client
}

// NewTaskManager creates a new task manager.
func NewTaskManager(dbClient db.Provider) *TaskManager {
	tm := &TaskManager{
		dbClient: dbClient,
	}

	redisURL := os.Getenv("REDIS_URL")
	if os.Getenv("OHC_MULTITENANT") == "true" && redisURL != "" {
		opts, err := redis.ParseURL(redisURL)
		if err == nil {
			tm.redisCli = redis.NewClient(opts)
			slog.Info("tasks: initialized with Redis distributed locking", "redis_url", redisURL)
		} else {
			slog.Error("tasks: failed to parse REDIS_URL for locking", "err", err)
		}
	} else {
		slog.Info("tasks: initialized with DB row-locking (standalone mode)")
	}

	return tm
}

// CreateTask adds a new task to the queue.
func (tm *TaskManager) CreateTask(ctx context.Context, missionID, title, description, priority string) (*SharedTask, error) {
	id := uuid.New().String()
	now := time.Now().UTC()

	if priority == "" {
		priority = "P2"
	}

	if tm.dbClient.IsSQLite() {
		query := `
			INSERT INTO shared_tasks (id, mission_id, title, description, status, priority, created_at, updated_at)
			VALUES (?, ?, ?, ?, ?, ?, ?, ?)
		`
		_, err := tm.dbClient.Exec(ctx, query, id, missionID, title, description, "PENDING", priority, now, now)
		if err != nil {
			return nil, err
		}
	} else {
		query := `
			INSERT INTO shared_tasks (id, mission_id, title, description, status, priority, created_at, updated_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
		`
		_, err := tm.dbClient.Exec(ctx, query, id, missionID, title, description, "PENDING", priority, now, now)
		if err != nil {
			return nil, err
		}
	}

	return &SharedTask{
		ID:          id,
		MissionID:   missionID,
		Title:       title,
		Description: description,
		Status:      "PENDING",
		Priority:    priority,
		CreatedAt:   now,
		UpdatedAt:   now,
	}, nil
}

// GetPendingTasks returns all tasks in the PENDING state.
func (tm *TaskManager) GetPendingTasks(ctx context.Context) ([]SharedTask, error) {
	query := `SELECT id, mission_id, title, description, assigned_agent_id, status, priority, created_at, updated_at FROM shared_tasks WHERE status = 'PENDING' ORDER BY priority ASC, created_at ASC`
	rows, err := tm.dbClient.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []SharedTask
	for rows.Next() {
		var t SharedTask
		var assigned sql.NullString
		var desc sql.NullString
		if err := rows.Scan(&t.ID, &t.MissionID, &t.Title, &desc, &assigned, &t.Status, &t.Priority, &t.CreatedAt, &t.UpdatedAt); err != nil {
			return nil, err
		}
		if assigned.Valid {
			t.AssignedAgentID = assigned.String
		}
		if desc.Valid {
			t.Description = desc.String
		}
		tasks = append(tasks, t)
	}
	return tasks, nil
}

// ClaimTask attempts to atomically claim a task for an agent.
func (tm *TaskManager) ClaimTask(ctx context.Context, taskID, agentID string) error {
	// 1. Try Redis Lock if in Cloud Mode
	if tm.redisCli != nil {
		lockKey := "task_lock:" + taskID
		acquired, err := tm.redisCli.SetNX(ctx, lockKey, agentID, 30*time.Second).Result()
		if err != nil {
			return err
		}
		if !acquired {
			return ErrTaskLocked
		}
		defer tm.redisCli.Del(ctx, lockKey)
	}

	// 2. Perform DB update inside a transaction to ensure no other agent claimed it
	tx, err := tm.dbClient.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// Check current status
	var status string

	var row db.Row
	if tm.dbClient.IsSQLite() {
		row = tx.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = ?", taskID)
	} else {
		row = tx.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", taskID)
	}
	err = row.Scan(&status)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return ErrTaskNotFound
		}
		return err
	}

	if status != "PENDING" {
		return ErrTaskLocked
	}

	now := time.Now().UTC()

	var res int64
	if tm.dbClient.IsSQLite() {
		query := `UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = ?, updated_at = ? WHERE id = ? AND status = 'PENDING'`
		res, err = tx.Exec(ctx, query, agentID, now, taskID)
	} else {
		query := `UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = $2 WHERE id = $3 AND status = 'PENDING'`
		res, err = tx.Exec(ctx, query, agentID, now, taskID)
	}

	if err != nil {
		return err
	}

	if res == 0 {
		return ErrTaskLocked
	}

	return tx.Commit(ctx)
}

// UpdateTaskStatus updates a task's status (e.g., COMPLETED, FAILED).
func (tm *TaskManager) UpdateTaskStatus(ctx context.Context, taskID, status string) error {
	now := time.Now().UTC()
	if tm.dbClient.IsSQLite() {
		query := `UPDATE shared_tasks SET status = ?, updated_at = ? WHERE id = ?`
		_, err := tm.dbClient.Exec(ctx, query, status, now, taskID)
		return err
	} else {
		query := `UPDATE shared_tasks SET status = $1, updated_at = $2 WHERE id = $3`
		_, err := tm.dbClient.Exec(ctx, query, status, now, taskID)
		return err
	}
}
