package queue

import (
	"context"
	"database/sql"
	"errors"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/db"
)

type SharedTask struct {
	ID             string    `json:"id"`
	OrganizationID string    `json:"organization_id"`
	ParentID       *string   `json:"parent_id"`
	EpicID         *string   `json:"epic_id"`
	Title          string    `json:"title"`
	Status         string    `json:"status"`
	AssignedAgent  *string   `json:"assigned_agent"`
	Payload        []byte    `json:"payload"`
	CreatedAt      time.Time `json:"created_at"`
	UpdatedAt      time.Time `json:"updated_at"`
}

type TaskQueueService struct {
	db db.Provider
}

func NewTaskQueueService(dbProvider db.Provider) *TaskQueueService {
	return &TaskQueueService{
		db: dbProvider,
	}
}

func (s *TaskQueueService) PushTask(ctx context.Context, task *SharedTask) error {
	if task.ID == "" {
		task.ID = uuid.NewString()
	}
	if task.OrganizationID == "" {
		return errors.New("organization_id is required")
	}

	query := `INSERT INTO shared_tasks (id, parent_id, epic_id, title, status, assigned_agent, payload, organization_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`

	var parentID sql.NullString
	if task.ParentID != nil {
		parentID.String = *task.ParentID
		parentID.Valid = true
	}
	var epicID sql.NullString
	if task.EpicID != nil {
		epicID.String = *task.EpicID
		epicID.Valid = true
	}
	var assignedAgent sql.NullString
	if task.AssignedAgent != nil {
		assignedAgent.String = *task.AssignedAgent
		assignedAgent.Valid = true
	}

	payloadStr := "{}"
	if task.Payload != nil {
		payloadStr = string(task.Payload)
	}

	if s.db.IsSQLite() {
		query = `INSERT INTO shared_tasks (id, parent_id, epic_id, title, status, assigned_agent, payload, organization_id)
			VALUES (?, ?, ?, ?, ?, ?, ?, ?)`
		_, err := s.db.Exec(ctx, query, task.ID, parentID, epicID, task.Title, "PENDING", assignedAgent, payloadStr, task.OrganizationID)
		return err
	}

	_, err := s.db.Exec(ctx, query, task.ID, parentID, epicID, task.Title, "PENDING", assignedAgent, payloadStr, task.OrganizationID)
	return err
}

func (s *TaskQueueService) ClaimTask(ctx context.Context, agentID string) (*SharedTask, error) {
	if s.db.IsSQLite() {
		return s.claimTaskSQLite(ctx, agentID)
	}
	return s.claimTaskPG(ctx, agentID)
}

func (s *TaskQueueService) claimTaskPG(ctx context.Context, agentID string) (*SharedTask, error) {
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assigned_agent = $1
		WHERE id = (
			SELECT id FROM shared_tasks
			WHERE status = 'PENDING' AND (assigned_agent IS NULL OR assigned_agent = $1)
			ORDER BY created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		)
		RETURNING id, organization_id, parent_id, epic_id, title, status, assigned_agent, payload, created_at, updated_at
	`

	row := tx.QueryRow(ctx, query, agentID)

	var task SharedTask
	var parentID, epicID, assigned sql.NullString
	var payloadStr string
	var createdAt, updatedAt sql.NullTime

	err = row.Scan(&task.ID, &task.OrganizationID, &parentID, &epicID, &task.Title, &task.Status, &assigned, &payloadStr, &createdAt, &updatedAt)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No tasks available
		}
		return nil, err
	}

	if parentID.Valid {
		task.ParentID = &parentID.String
	}
	if epicID.Valid {
		task.EpicID = &epicID.String
	}
	if assigned.Valid {
		task.AssignedAgent = &assigned.String
	}
	if payloadStr != "" {
		task.Payload = []byte(payloadStr)
	}
	if createdAt.Valid {
		task.CreatedAt = createdAt.Time
	}
	if updatedAt.Valid {
		task.UpdatedAt = updatedAt.Time
	}

	return &task, tx.Commit(ctx)
}

func (s *TaskQueueService) claimTaskSQLite(ctx context.Context, agentID string) (*SharedTask, error) {
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	// SQLite doesn't support SKIP LOCKED, but RETURNING is supported in modern versions
	query := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assigned_agent = ?
		WHERE id = (
			SELECT id FROM shared_tasks
			WHERE status = 'PENDING' AND (assigned_agent IS NULL OR assigned_agent = ?)
			ORDER BY created_at ASC
			LIMIT 1
		)
		RETURNING id, organization_id, parent_id, epic_id, title, status, assigned_agent, payload, created_at, updated_at
	`

	row := tx.QueryRow(ctx, query, agentID, agentID)

	var task SharedTask
	var parentID, epicID, assigned sql.NullString
	var payloadStr string
	var createdAt, updatedAt sql.NullTime

	err = row.Scan(&task.ID, &task.OrganizationID, &parentID, &epicID, &task.Title, &task.Status, &assigned, &payloadStr, &createdAt, &updatedAt)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No tasks available
		}
		return nil, err
	}

	if parentID.Valid {
		task.ParentID = &parentID.String
	}
	if epicID.Valid {
		task.EpicID = &epicID.String
	}
	if assigned.Valid {
		task.AssignedAgent = &assigned.String
	}
	if payloadStr != "" {
		task.Payload = []byte(payloadStr)
	}
	if createdAt.Valid {
		task.CreatedAt = createdAt.Time
	}
	if updatedAt.Valid {
		task.UpdatedAt = updatedAt.Time
	}

	return &task, tx.Commit(ctx)
}

func (s *TaskQueueService) CompleteTask(ctx context.Context, taskID string) error {
	query := `UPDATE shared_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1`
	if s.db.IsSQLite() {
		query = `UPDATE shared_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?`
	}

	_, err := s.db.Exec(ctx, query, taskID)
	return err
}

func (s *TaskQueueService) GetCompletedTasks(ctx context.Context, limit int) ([]SharedTask, error) {
	query := `SELECT id, organization_id, parent_id, epic_id, title, status, assigned_agent, payload, created_at, updated_at
		FROM shared_tasks WHERE status = 'COMPLETED' LIMIT $1`
	if s.db.IsSQLite() {
		query = `SELECT id, organization_id, parent_id, epic_id, title, status, assigned_agent, payload, created_at, updated_at
		FROM shared_tasks WHERE status = 'COMPLETED' LIMIT ?`
	}

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []SharedTask
	for rows.Next() {
		var task SharedTask
		var parentID, epicID, assigned sql.NullString
		var payloadStr string
		var createdAt, updatedAt sql.NullTime

		if err := rows.Scan(&task.ID, &task.OrganizationID, &parentID, &epicID, &task.Title, &task.Status, &assigned, &payloadStr, &createdAt, &updatedAt); err != nil {
			return nil, err
		}

		if parentID.Valid {
			task.ParentID = &parentID.String
		}
		if epicID.Valid {
			task.EpicID = &epicID.String
		}
		if assigned.Valid {
			task.AssignedAgent = &assigned.String
		}
		if payloadStr != "" {
			task.Payload = []byte(payloadStr)
		}
		if createdAt.Valid {
			task.CreatedAt = createdAt.Time
		}
		if updatedAt.Valid {
			task.UpdatedAt = updatedAt.Time
		}

		tasks = append(tasks, task)
	}

	return tasks, nil
}
