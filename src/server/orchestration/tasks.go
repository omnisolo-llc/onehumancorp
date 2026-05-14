package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"time"

	"github.com/go-redis/redis/v8"
)

type Task struct {
	ID            string    `json:"id"`
	MissionID     string    `json:"mission_id"`
	ParentPlanID  string    `json:"parent_plan_id"`
	Dependencies  string    `json:"dependencies"`
	Title         string    `json:"title"`
	Payload       string    `json:"payload"`
	Status        string    `json:"status"`
	LockedUntil   time.Time `json:"locked_until"`
	CreatedAt     time.Time `json:"created_at"`
	UpdatedAt     time.Time `json:"updated_at"`
}

type TaskOrchestrator struct {
	db          *sql.DB
	redisClient *redis.Client
}

func NewTaskOrchestrator(db *sql.DB, redisClient *redis.Client) *TaskOrchestrator {
	return &TaskOrchestrator{
		db:          db,
		redisClient: redisClient,
	}
}

func (o *TaskOrchestrator) DelegateTask(ctx context.Context, task Task) error {
	query := `
		INSERT INTO swarm_tasks (
			id, mission_id, parent_plan_id, dependencies, title, payload, status, locked_until, created_at, updated_at
		) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)`
	_, err := o.db.ExecContext(ctx, query, task.ID, task.MissionID, task.ParentPlanID, task.Dependencies, task.Title, task.Payload, "PENDING", task.LockedUntil, task.CreatedAt, task.UpdatedAt)
	return err
}

func (o *TaskOrchestrator) AcquireTask(ctx context.Context, workerID string) (*Task, error) {
	// Try distributed lock first via Redis to avoid collision
	lockKey := "lock:swarm_tasks"
	lockAcquired, err := o.redisClient.SetNX(ctx, lockKey, workerID, 5*time.Second).Result()
	if err != nil {
		return nil, err
	}
	if !lockAcquired {
		return nil, errors.New("could not acquire distributed lock for task assignment")
	}

	// Release lock later
	defer o.redisClient.Del(ctx, lockKey)

	query := `
		SELECT id, mission_id, parent_plan_id, dependencies, title, payload, status, locked_until, created_at, updated_at
		FROM swarm_tasks
		WHERE status = 'PENDING'
		LIMIT 1`

	row := o.db.QueryRowContext(ctx, query)
	var task Task
	err = row.Scan(&task.ID, &task.MissionID, &task.ParentPlanID, &task.Dependencies, &task.Title, &task.Payload, &task.Status, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No task found
		}
		return nil, err
	}

	// Mark IN_PROGRESS
	updateQuery := `UPDATE swarm_tasks SET status = 'IN_PROGRESS', updated_at = $1 WHERE id = $2`
	_, err = o.db.ExecContext(ctx, updateQuery, time.Now(), task.ID)
	if err != nil {
		return nil, err
	}

	task.Status = "IN_PROGRESS"
	return &task, nil
}

func (o *TaskOrchestrator) CompleteTask(ctx context.Context, taskID string) error {
	query := `UPDATE swarm_tasks SET status = 'COMPLETED', updated_at = $1 WHERE id = $2`
	_, err := o.db.ExecContext(ctx, query, time.Now(), taskID)
	return err
}
