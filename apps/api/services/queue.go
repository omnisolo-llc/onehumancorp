package services

import (
	"database/sql"
	"fmt"
	"time"
)

type Task struct {
	ID            string
	ParentID      *string
	EpicID        *string
	Title         string
	Status        string
	AssignedAgent *string
	Payload       *string
	CreatedAt     time.Time
	UpdatedAt     time.Time
}

type TaskQueueService struct {
	db       *sql.DB
	isSQLite bool
}

func NewTaskQueueService(db *sql.DB, isSQLite bool) *TaskQueueService {
	return &TaskQueueService{db: db, isSQLite: isSQLite}
}

func (s *TaskQueueService) PushTask(id, title string, payload *string) error {
	query := `INSERT INTO shared_tasks (id, title, payload) VALUES ($1, $2, $3)`
	_, err := s.db.Exec(query, id, title, payload)
	return err
}

func (s *TaskQueueService) ClaimTask(agentID string) (*Task, error) {
	var task Task
	var query string

	if s.isSQLite {
		query = `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assigned_agent = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT id FROM shared_tasks WHERE status = 'PENDING' LIMIT 1
		)
		RETURNING id, parent_id, epic_id, title, status, assigned_agent, payload, created_at, updated_at`
	} else {
		query = `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assigned_agent = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT id FROM shared_tasks WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED LIMIT 1
		)
		RETURNING id, parent_id, epic_id, title, status, assigned_agent, payload, created_at, updated_at`
	}

	row := s.db.QueryRow(query, agentID)
	err := row.Scan(&task.ID, &task.ParentID, &task.EpicID, &task.Title, &task.Status, &task.AssignedAgent, &task.Payload, &task.CreatedAt, &task.UpdatedAt)
	if err == sql.ErrNoRows {
		return nil, nil // No pending tasks
	}
	if err != nil {
		return nil, fmt.Errorf("failed to claim task: %w", err)
	}

	return &task, nil
}

func (s *TaskQueueService) CompleteTask(id string) error {
	query := `UPDATE shared_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1`
	_, err := s.db.Exec(query, id)
	return err
}
