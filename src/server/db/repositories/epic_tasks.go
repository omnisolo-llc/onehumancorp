package repositories

import (
	"context"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/db/models"
)

type EpicTaskRepository interface {
	CreateEpic(ctx context.Context, epic *models.Epic) error
	CreateTask(ctx context.Context, task *models.EpicTask) error
	GetTasksByEpicID(ctx context.Context, epicID string) ([]*models.EpicTask, error)
}

type epicTaskRepositoryImpl struct {
	dbProvider db.Provider
}

func NewEpicTaskRepository(dbProvider db.Provider) EpicTaskRepository {
	return &epicTaskRepositoryImpl{
		dbProvider: dbProvider,
	}
}

func (r *epicTaskRepositoryImpl) CreateEpic(ctx context.Context, epic *models.Epic) error {
	q := `INSERT INTO epics (id) VALUES ($1)`

	if epic.ID == "" {
		epic.ID = uuid.New().String()
	}

	_, err := r.dbProvider.Exec(ctx, q, epic.ID)
	if err != nil {
		return fmt.Errorf("failed to insert epic: %w", err)
	}

	return nil
}

func (r *epicTaskRepositoryImpl) CreateTask(ctx context.Context, task *models.EpicTask) error {
	q := `INSERT INTO tasks (id, epic_id, title, status, assigned_agent, created_at, updated_at)
		  VALUES ($1, $2, $3, $4, $5, $6, $7)`

	now := time.Now().UTC()
	if task.ID == "" {
		task.ID = uuid.New().String()
	}
	if task.CreatedAt.IsZero() {
		task.CreatedAt = now
	}
	if task.UpdatedAt.IsZero() {
		task.UpdatedAt = now
	}
	if task.Status == "" {
		task.Status = "PENDING"
	}

	_, err := r.dbProvider.Exec(ctx, q, task.ID, task.EpicID, task.Title, task.Status, task.AssignedAgent, task.CreatedAt, task.UpdatedAt)
	if err != nil {
		return fmt.Errorf("failed to insert epic task: %w", err)
	}

	return nil
}

func (r *epicTaskRepositoryImpl) GetTasksByEpicID(ctx context.Context, epicID string) ([]*models.EpicTask, error) {
	q := `SELECT id, epic_id, title, status, assigned_agent, created_at, updated_at
		  FROM tasks WHERE epic_id = $1 ORDER BY created_at ASC`

	rows, err := r.dbProvider.Query(ctx, q, epicID)
	if err != nil {
		return nil, fmt.Errorf("failed to query tasks by epic id: %w", err)
	}
	defer rows.Close()

	var tasks []*models.EpicTask
	for rows.Next() {
		task := &models.EpicTask{}
		err := rows.Scan(&task.ID, &task.EpicID, &task.Title, &task.Status, &task.AssignedAgent, &task.CreatedAt, &task.UpdatedAt)
		if err != nil {
			return nil, fmt.Errorf("failed to scan task: %w", err)
		}
		tasks = append(tasks, task)
	}
	if err = rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return tasks, nil
}
