package tasks

import (
	"context"
	"database/sql"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/google/uuid"
)

type TaskDecomposition struct {
	ID              string
	MissionID       string
	ParentPlanID    *string
	Dependencies    string
	Title           string
	Status          string
	AssignedAgentID *string
	Payload         *string
	LockedUntil     *time.Time
	CreatedAt       time.Time
}

type TaskDecompositionService struct {
	provider db.Provider
}

func NewTaskDecompositionService(provider db.Provider) *TaskDecompositionService {
	return &TaskDecompositionService{
		provider: provider,
	}
}

func (s *TaskDecompositionService) Create(ctx context.Context, task TaskDecomposition) (string, error) {
	if task.ID == "" {
		task.ID = uuid.NewString()
	}
	if task.Dependencies == "" {
		task.Dependencies = "[]"
	}
	if task.Status == "" {
		task.Status = "PENDING"
	}

	query := `INSERT INTO swarm_tasks (
		id, mission_id, parent_plan_id, dependencies, title, status, assigned_agent_id,
		payload, locked_until
	) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`

	_, err := s.provider.Exec(ctx, query,
		task.ID, task.MissionID, task.ParentPlanID, task.Dependencies, task.Title, task.Status,
		task.AssignedAgentID, task.Payload, task.LockedUntil,
	)

	if err != nil {
		return "", err
	}
	return task.ID, nil
}

func (s *TaskDecompositionService) Get(ctx context.Context, id string) (*TaskDecomposition, error) {
	query := `SELECT id, mission_id, parent_plan_id, dependencies, title, status, assigned_agent_id,
		payload, locked_until, created_at
	FROM swarm_tasks WHERE id = $1`

	var task TaskDecomposition
	err := s.provider.QueryRow(ctx, query, id).Scan(
		&task.ID, &task.MissionID, &task.ParentPlanID, &task.Dependencies, &task.Title, &task.Status,
		&task.AssignedAgentID, &task.Payload, &task.LockedUntil, &task.CreatedAt,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil
		}
		return nil, err
	}
	return &task, nil
}

func (s *TaskDecompositionService) UpdateState(ctx context.Context, id string, status string, agentID *string, reason *string) error {
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var currentStatus string
	err = tx.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1", id).Scan(&currentStatus)
	if err != nil {
		return err
	}

	query := `UPDATE swarm_tasks SET status = $1 WHERE id = $2`
	_, err = tx.Exec(ctx, query, status, id)
	if err != nil {
		return err
	}

	transitionID := uuid.NewString()
	transitionQuery := `INSERT INTO state_machine_transitions (
		id, entity_id, entity_type, from_state, to_state, agent_id, reason
	) VALUES ($1, $2, $3, $4, $5, $6, $7)`
	_, err = tx.Exec(ctx, transitionQuery, transitionID, id, "swarm_task", currentStatus, status, agentID, reason)
	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}

func (s *TaskDecompositionService) Claim(ctx context.Context, missionID string, agentID string) (*TaskDecomposition, error) {
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var task TaskDecomposition
	var query string

	if s.provider.IsSQLite() {
		// SQLite emulation for DAG dependencies
		query = `SELECT id, mission_id, parent_plan_id, dependencies, title, status, assigned_agent_id,
			payload, locked_until, created_at
		FROM swarm_tasks t
		WHERE status = 'PENDING' AND mission_id = $1
		AND (
			json_array_length(dependencies) = 0
			OR NOT EXISTS (
				SELECT 1 FROM json_each(t.dependencies) d
				JOIN swarm_tasks dep ON dep.id = d.value
				WHERE dep.status != 'COMPLETED'
			)
		)
		ORDER BY created_at ASC LIMIT 1`

		err = tx.QueryRow(ctx, query, missionID).Scan(
			&task.ID, &task.MissionID, &task.ParentPlanID, &task.Dependencies, &task.Title, &task.Status,
			&task.AssignedAgentID, &task.Payload, &task.LockedUntil, &task.CreatedAt,
		)
		if err != nil {
			if err == sql.ErrNoRows {
				return nil, nil
			}
			return nil, err
		}

		updateQuery := `UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1 WHERE id = $2`
		_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
		if err != nil {
			return nil, err
		}
	} else {
		// PostgreSQL with FOR UPDATE SKIP LOCKED
		query = `SELECT id, mission_id, parent_plan_id, dependencies, title, status, assigned_agent_id,
			payload, locked_until, created_at
		FROM swarm_tasks t
		WHERE status = 'PENDING' AND mission_id = $1
		AND (
			jsonb_array_length(dependencies) = 0
			OR NOT EXISTS (
				SELECT 1 FROM jsonb_array_elements_text(t.dependencies) AS d
				JOIN swarm_tasks dep ON dep.id = d::uuid
				WHERE dep.status != 'COMPLETED'
			)
		)
		ORDER BY created_at ASC FOR UPDATE SKIP LOCKED LIMIT 1`

		err = tx.QueryRow(ctx, query, missionID).Scan(
			&task.ID, &task.MissionID, &task.ParentPlanID, &task.Dependencies, &task.Title, &task.Status,
			&task.AssignedAgentID, &task.Payload, &task.LockedUntil, &task.CreatedAt,
		)
		if err != nil {
			if err == sql.ErrNoRows {
				return nil, nil
			}
			return nil, err
		}

		updateQuery := `UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1 WHERE id = $2`
		_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
		if err != nil {
			return nil, err
		}
	}

	// Record transition
	transitionID := uuid.NewString()
	transitionQuery := `INSERT INTO state_machine_transitions (
		id, entity_id, entity_type, from_state, to_state, agent_id
	) VALUES ($1, $2, $3, $4, $5, $6)`
	_, err = tx.Exec(ctx, transitionQuery, transitionID, task.ID, "swarm_task", "PENDING", "IN_PROGRESS", agentID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = &agentID

	return &task, nil
}
