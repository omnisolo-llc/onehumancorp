package tasks

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/google/uuid"
)

type SharedTaskDecomposition struct {
	ID              string
	OrganizationID       string
	Priority        string
	ParentPlanID    *string
	Dependencies    json.RawMessage
	Title           string
	Status          string
	AssignedAgentID *string
	Payload         *json.RawMessage
	LockedUntil     *time.Time
	CreatedAt       time.Time
}

type StateMachineTransition struct {
	ID         string
	EntityID   string
	EntityType string
	FromState  string
	ToState    string
	AgentID    *string
	Reason     *string
	OccurredAt time.Time
}

type TaskDecompositionService interface {
	CreateTask(ctx context.Context, task *SharedTaskDecomposition) error
	GetTask(ctx context.Context, id string, organizationID string) (*SharedTaskDecomposition, error)
	ClaimTask(ctx context.Context, organizationID string, agentID string) (*SharedTaskDecomposition, error)
	UpdateTaskStatus(ctx context.Context, id string, newStatus string, agentID string, reason string) error
}

type DBTaskDecompositionService struct {
	db      *sql.DB
	isPgSQL bool
}

func NewDBTaskDecompositionService(db *sql.DB, isPgSQL bool) *DBTaskDecompositionService {
	return &DBTaskDecompositionService{
		db:      db,
		isPgSQL: isPgSQL,
	}
}

func (s *DBTaskDecompositionService) CreateTask(ctx context.Context, task *SharedTaskDecomposition) error {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}
	if task.Status == "" {
		task.Status = "PENDING"
	}
	if len(task.Dependencies) == 0 {
		task.Dependencies = json.RawMessage("[]")
	}
	if task.Priority == "" {
		task.Priority = "P2"
	}

	query := `
		INSERT INTO shared_tasks_decomposition (id, organization_id, parent_plan_id, dependencies, title, status, assigned_agent_id, payload, locked_until, priority, created_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, CURRENT_TIMESTAMP)
	`
	var parentPlanID sql.NullString
	if task.ParentPlanID != nil {
		parentPlanID = sql.NullString{String: *task.ParentPlanID, Valid: true}
	}
	var assignedAgentID sql.NullString
	if task.AssignedAgentID != nil {
		assignedAgentID = sql.NullString{String: *task.AssignedAgentID, Valid: true}
	}
	var payload interface{}
	if task.Payload != nil {
		payload = []byte(*task.Payload)
	}

	_, err := s.db.ExecContext(ctx, query,
		task.ID,
		task.OrganizationID,
		parentPlanID,
		[]byte(task.Dependencies),
		task.Title,
		task.Status,
		assignedAgentID,
		payload,
		task.LockedUntil,
		task.Priority,
	)
	return err
}

func (s *DBTaskDecompositionService) GetTask(ctx context.Context, id string, organizationID string) (*SharedTaskDecomposition, error) {
	query := `
		SELECT id, organization_id, parent_plan_id, dependencies, title, status, assigned_agent_id, payload, locked_until, priority, created_at
		FROM shared_tasks_decomposition
		WHERE id = $1
	`
	row := s.db.QueryRowContext(ctx, query, id)

	var task SharedTaskDecomposition
	var parentPlanID sql.NullString
	var assignedAgentID sql.NullString
	var payload []byte
	var deps []byte
	var lockedUntil sql.NullTime

	err := row.Scan(
		&task.ID,
		&task.OrganizationID,
		&parentPlanID,
		&deps,
		&task.Title,
		&task.Status,
		&assignedAgentID,
		&payload,
		&lockedUntil,
		&task.Priority,
		&task.CreatedAt,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil
		}
		return nil, err
	}

	if parentPlanID.Valid {
		task.ParentPlanID = &parentPlanID.String
	}
	if assignedAgentID.Valid {
		task.AssignedAgentID = &assignedAgentID.String
	}
	if payload != nil {
		rm := json.RawMessage(payload)
		task.Payload = &rm
	}
	if deps != nil {
		task.Dependencies = json.RawMessage(deps)
	}
	if lockedUntil.Valid {
		task.LockedUntil = &lockedUntil.Time
	}

	return &task, nil
}

func (s *DBTaskDecompositionService) areDependenciesMet(ctx context.Context, tx *sql.Tx, deps json.RawMessage) (bool, error) {
	if len(deps) == 0 || string(deps) == "[]" || string(deps) == "null" {
		return true, nil
	}

	var depIDs []string
	if err := json.Unmarshal(deps, &depIDs); err != nil {
		return false, err
	}

	for _, depID := range depIDs {
		var status string
		err := tx.QueryRowContext(ctx, "SELECT status FROM shared_tasks_decomposition WHERE id = $1", depID).Scan(&status)
		if err != nil {
			if errors.Is(err, sql.ErrNoRows) {
				return false, nil // Missing dependency
			}
			return false, err
		}
		if status != "COMPLETED" {
			return false, nil
		}
	}
	return true, nil
}

func (s *DBTaskDecompositionService) ClaimTask(ctx context.Context, organizationID string, agentID string) (*SharedTaskDecomposition, error) {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	// 1. Find all PENDING tasks for the mission.
	query := "SELECT id, dependencies FROM shared_tasks_decomposition WHERE organization_id = $1 AND status = 'PENDING'"
	if s.isPgSQL {
		query += " FOR UPDATE SKIP LOCKED"
	}
	rows, err := tx.QueryContext(ctx, query, organizationID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var candidateID string

	type candidateTask struct {
		id   string
		deps []byte
	}
	var candidates []candidateTask

	for rows.Next() {
		var id string
		var deps []byte
		if err := rows.Scan(&id, &deps); err != nil {
			return nil, err
		}
		candidates = append(candidates, candidateTask{id: id, deps: deps})
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}

	// Wait to close rows to free up connection before further tx query
	rows.Close()

	for _, c := range candidates {
		met, err := s.areDependenciesMet(ctx, tx, json.RawMessage(c.deps))
		if err != nil {
			return nil, err
		}
		if met {
			candidateID = c.id
			break
		}
	}

	if candidateID == "" {
		return nil, nil // No task ready
	}

	// 2. Claim it
	updateQuery := "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1 WHERE id = $2"
	_, err = tx.ExecContext(ctx, updateQuery, agentID, candidateID)
	if err != nil {
		return nil, err
	}

	// 3. Record transition
	transID := uuid.New().String()
	transQuery := `
		INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
		VALUES ($1, $2, 'shared_tasks_decomposition', 'PENDING', 'IN_PROGRESS', $3, 'Claimed by agent', CURRENT_TIMESTAMP)
	`
	_, err = tx.ExecContext(ctx, transQuery, transID, candidateID, agentID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	return s.GetTask(ctx, candidateID, organizationID)
}

func (s *DBTaskDecompositionService) UpdateTaskStatus(ctx context.Context, id string, newStatus string, agentID string, reason string) error {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	var currentStatus string
	err = tx.QueryRowContext(ctx, "SELECT status FROM shared_tasks_decomposition WHERE id = $1", id).Scan(&currentStatus)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return fmt.Errorf("task not found")
		}
		return err
	}

	if currentStatus == newStatus {
		return nil
	}

	updateQuery := "UPDATE shared_tasks_decomposition SET status = $1 WHERE id = $2"
	_, err = tx.ExecContext(ctx, updateQuery, newStatus, id)
	if err != nil {
		return err
	}

	transID := uuid.New().String()
	transQuery := `
		INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
		VALUES ($1, $2, 'shared_tasks_decomposition', $3, $4, $5, $6, CURRENT_TIMESTAMP)
	`
	_, err = tx.ExecContext(ctx, transQuery, transID, id, currentStatus, newStatus, agentID, reason)
	if err != nil {
		return err
	}

	return tx.Commit()
}
