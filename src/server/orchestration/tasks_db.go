package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"sync"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/memory/autodream"
	"github.com/onehumancorp/mono/src/server/telemetry"
)

type SharedTaskDB struct {
	ID              string
	OrganizationID  string
	Title           string
	Description     *string
	Status          string
	AssignedAgentID *string
	Priority        string
	Payload         *string
	ParentPlanID    *string
	Dependencies    *string
	DeploymentMode  *string
	CreatedAt       string
	UpdatedAt       string
}

type SharedTaskOrchestrator struct {
	dbProvider db.Provider
	mu         sync.Mutex
	mesh       MeshTransport
	autodream  autodream.MemoryConsolidator
}

func NewSharedTaskOrchestrator(dbProvider db.Provider, mesh MeshTransport, ad autodream.MemoryConsolidator) *SharedTaskOrchestrator {
	return &SharedTaskOrchestrator{
		dbProvider: dbProvider,
		mesh:       mesh,
		autodream:  ad,
	}
}

func (to *SharedTaskOrchestrator) insertTransition(ctx context.Context, tx db.Tx, taskID, from, toState, agentID, reason string) error {
	id := uuid.New().String()
	_, err := tx.Exec(ctx, `
        INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
        VALUES ($1, $2, 'task', $3, $4, $5, $6, CURRENT_TIMESTAMP)
    `, id, taskID, from, toState, agentID, reason)
	return err
}

func (to *SharedTaskOrchestrator) ClaimTask(ctx context.Context, agentID string) (*Task, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}
	orgID := claims.OrganizationID

	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var id string

	if to.dbProvider.IsSQLite() {
		if !to.mu.TryLock() {
			telemetry.RecordPostgresLockContention(ctx, "claim_task")
			to.mu.Lock()
		}
		defer to.mu.Unlock()

		query := `
            SELECT t.id FROM shared_tasks t
            WHERE t.status = 'PENDING' AND t.organization_id = $1
            AND NOT EXISTS (
                SELECT 1 FROM json_each(t.dependencies) d
                JOIN shared_tasks dep ON dep.id = d.value
                WHERE dep.status != 'COMPLETED'
            )
            LIMIT 1
        `
		err = tx.QueryRow(ctx, query, orgID).Scan(&id)
	} else {
		query := `
            SELECT t.id FROM shared_tasks t
            WHERE t.status = 'PENDING' AND t.organization_id = $1
            AND NOT EXISTS (
                SELECT 1 FROM jsonb_array_elements_text(t.dependencies::jsonb) d
                JOIN shared_tasks dep ON dep.id::text = d
                WHERE dep.status != 'COMPLETED'
            )
            LIMIT 1
            FOR UPDATE SKIP LOCKED
        `
		err = tx.QueryRow(ctx, query, orgID).Scan(&id)
	}

	if err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, err
	}

	_, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND organization_id = $3", agentID, id, orgID)
	if err != nil {
		return nil, err
	}

	if err := to.insertTransition(ctx, tx, id, "PENDING", "IN_PROGRESS", agentID, "Task claimed by agent"); err != nil {
		return nil, fmt.Errorf("failed to insert transition: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	return &Task{TaskID: id, Status: "IN_PROGRESS", AgentID: agentID}, nil
}

func (to *SharedTaskOrchestrator) TransitionTask(ctx context.Context, taskID, agentID, fromState, toState, reason string) error {
	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var current string
	if err := tx.QueryRow(ctx, "SELECT status FROM shared_tasks_master WHERE id = $1", taskID).Scan(&current); err != nil {
		return fmt.Errorf("failed to fetch task %s: %w", taskID, err)
	}

	if current != fromState {
		return fmt.Errorf("task %s is in state %s, expected %s", taskID, current, fromState)
	}

	if _, err := tx.Exec(ctx, "UPDATE shared_tasks_master SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", toState, taskID); err != nil {
		return err
	}

	if err := to.insertTransition(ctx, tx, taskID, fromState, toState, agentID, reason); err != nil {
		return err
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if toState == "COMPLETED" {
		if to.autodream != nil {
			go func() {
				var payloadText, deliberationLog string
				err := to.dbProvider.QueryRow(context.Background(), "SELECT COALESCE(payload, '{}'), COALESCE(deliberation_log, '{}') FROM shared_tasks_master WHERE id = $1", taskID).Scan(&payloadText, &deliberationLog)
				if err != nil {
					// Log error here in a real scenario
					return
				}

				logs := []string{"Task " + taskID + " completed successfully.", "Payload: " + payloadText, "Deliberation Log: " + deliberationLog}
				_ = to.autodream.Consolidate(context.Background(), taskID, logs)
			}()
		}

		if to.mesh != nil {
			go func() {
				payload := map[string]interface{}{
					"task_id":  taskID,
					"action":   "COMPLETE",
					"agent_id": agentID,
					"status":   "COMPLETED",
				}
				payloadBytes, err := json.Marshal(payload)
				if err == nil {
					_ = to.mesh.BroadcastMeshEvent(context.Background(), "tasks", payloadBytes)
				}
			}()
		}
	}

	return nil
}

func (to *SharedTaskOrchestrator) ClaimPendingTask(ctx context.Context) (*Task, error) {
	if to.dbProvider.IsSQLite() {
		if !to.mu.TryLock() {
			telemetry.RecordPostgresLockContention(ctx, "claim_pending_task")
			to.mu.Lock()
		}
		defer to.mu.Unlock()
	}

	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
        SELECT id FROM shared_tasks_v2
        WHERE status = 'PENDING'
        LIMIT 1
        FOR UPDATE SKIP LOCKED
    `

	if to.dbProvider.IsSQLite() {
		query = `
            SELECT id FROM shared_tasks_v2
            WHERE status = 'PENDING'
            LIMIT 1
        `
	}

	var id string
	err = tx.QueryRow(ctx, query).Scan(&id)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) || err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, err
	}

	_, err = tx.Exec(ctx, "UPDATE shared_tasks_v2 SET status = 'IN_PROGRESS' WHERE id = $1", id)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	return &Task{TaskID: id, Status: "IN_PROGRESS"}, nil
}

func (to *SharedTaskOrchestrator) ClaimTaskV4(ctx context.Context, orgID, agentID string) (*SharedTaskDB, error) {
	if to.dbProvider.IsSQLite() {
		if !to.mu.TryLock() {
			telemetry.RecordPostgresLockContention(ctx, "claim_task_v4")
			to.mu.Lock()
		}
		defer to.mu.Unlock()
	}

	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	query := `
        SELECT id FROM shared_tasks_v4 t
        WHERE t.status = 'PENDING' AND t.organization_id = $1
        AND NOT EXISTS (
            SELECT 1
            FROM jsonb_array_elements_text(t.dependencies::jsonb) d
            JOIN shared_tasks_v4 dep ON dep.id = d
            WHERE dep.status != 'COMPLETED'
        )
        LIMIT 1
        FOR UPDATE SKIP LOCKED
    `

	if to.dbProvider.IsSQLite() {
		query = `
            SELECT id FROM shared_tasks_v4 t
            WHERE t.status = 'PENDING' AND t.organization_id = $1
            AND NOT EXISTS (
                SELECT 1
                FROM json_each(t.dependencies) d
                JOIN shared_tasks_v4 dep ON dep.id = d.value
                WHERE dep.status != 'COMPLETED'
            )
            LIMIT 1
        `
	}

	var id string
	err = tx.QueryRow(ctx, query, orgID).Scan(&id)
	if err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, err
	}

	_, err = tx.Exec(ctx, "UPDATE shared_tasks_v4 SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, id)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	return &SharedTaskDB{ID: id, Status: "IN_PROGRESS"}, nil
}

func (to *SharedTaskOrchestrator) CreateTaskV4(ctx context.Context, task *SharedTaskDB) error {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}

	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	depsStr := "[]"
	if task.Dependencies != nil && *task.Dependencies != "" {
		depsStr = *task.Dependencies
	}

	query := `
        INSERT INTO shared_tasks_v4 (id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
    `

	var priority string
	if task.Priority != "" {
		priority = task.Priority
	} else {
		priority = "P2"
	}

	var status string
	if task.Status != "" {
		status = task.Status
	} else {
		status = "PENDING"
	}

	_, err = tx.Exec(ctx, query,
		task.ID,
		task.OrganizationID,
		task.Title,
		task.Description,
		status,
		task.AssignedAgentID,
		priority,
		task.Payload,
		task.ParentPlanID,
		depsStr,
	)

	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}

type TasksDB struct {
	dbProvider db.Provider
	mu         sync.Mutex
}

func NewTasksDB(dbProvider db.Provider) *TasksDB {
	return &TasksDB{
		dbProvider: dbProvider,
	}
}

func (to *TasksDB) insertTransition(ctx context.Context, tx db.Tx, taskID, from, toState, agentID, reason string) error {
	id := uuid.New().String()
	_, err := tx.Exec(ctx, `
        INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
        VALUES ($1, $2, 'task', $3, $4, $5, $6, CURRENT_TIMESTAMP)
    `, id, taskID, from, toState, agentID, reason)
	return err
}

func (to *TasksDB) ClaimTask(ctx context.Context, agentID string) (*Task, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}
	orgID := claims.OrganizationID

	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var id string

	if to.dbProvider.IsSQLite() {
		if !to.mu.TryLock() {
			// Fallback to Lock
			to.mu.Lock()
		}
		defer to.mu.Unlock()

		query := `
            SELECT t.id FROM shared_tasks t
            WHERE t.status = 'PENDING' AND t.organization_id = $1
            AND NOT EXISTS (
                SELECT 1 FROM json_each(t.dependencies) d
                JOIN shared_tasks dep ON dep.id = d.value
                WHERE dep.status != 'DONE' AND dep.status != 'COMPLETED'
            )
            LIMIT 1
        `
		err = tx.QueryRow(ctx, query, orgID).Scan(&id)
	} else {
		query := `
            SELECT t.id FROM shared_tasks t
            WHERE t.status = 'PENDING' AND t.organization_id = $1
            AND NOT EXISTS (
                SELECT 1 FROM jsonb_array_elements_text(COALESCE(t.dependencies, '[]'::jsonb)) d
                JOIN shared_tasks dep ON dep.id = d
                WHERE dep.status != 'DONE' AND dep.status != 'COMPLETED'
            )
            LIMIT 1
            FOR UPDATE SKIP LOCKED
        `
		err = tx.QueryRow(ctx, query, orgID).Scan(&id)
	}

	if err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, err
	}

	_, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND organization_id = $3", agentID, id, orgID)
	if err != nil {
		return nil, err
	}

	if err := to.insertTransition(ctx, tx, id, "PENDING", "IN_PROGRESS", agentID, "Task claimed by agent"); err != nil {
		return nil, fmt.Errorf("failed to insert transition: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	return &Task{TaskID: id, Status: "IN_PROGRESS", AgentID: agentID}, nil
}

func (to *TasksDB) TransitionTask(ctx context.Context, taskID, agentID, fromState, toState, reason string) error {
	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var current string
	if err := tx.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", taskID).Scan(&current); err != nil {
		return fmt.Errorf("failed to fetch task %s: %w", taskID, err)
	}

	if current != fromState {
		return fmt.Errorf("task %s is in state %s, expected %s", taskID, current, fromState)
	}

	if _, err := tx.Exec(ctx, "UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", toState, taskID); err != nil {
		return err
	}

	if err := to.insertTransition(ctx, tx, taskID, fromState, toState, agentID, reason); err != nil {
		return err
	}

	return tx.Commit(ctx)
}

func (to *TasksDB) GetPendingApprovalTasks(ctx context.Context, orgID string) ([]Task, error) {
	query := `
		SELECT id, status, assigned_agent_id, action_risk, approval_status, proposed_content, created_at, updated_at
		FROM shared_tasks
		WHERE organization_id = $1 AND status = 'PENDING_APPROVAL'
	`
	rows, err := to.dbProvider.Query(ctx, query, orgID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []Task
	for rows.Next() {
		var t Task
		var risk, status, content *string
		if err := rows.Scan(&t.ID, &t.Status, &t.AssignedAgentID, &risk, &status, &content, &t.CreatedAt, &t.UpdatedAt); err != nil {
			return nil, err
		}
		if risk != nil {
			t.ActionRisk = *risk
		}
		if status != nil {
			t.ApprovalStatus = *status
		}
		if content != nil {
			t.ProposedContent = *content
		}
		tasks = append(tasks, t)
	}
	return tasks, nil
}

func (to *TasksDB) ApproveTask(ctx context.Context, taskID string, agentID string) error {
	return to.TransitionTask(ctx, taskID, agentID, "PENDING_APPROVAL", "COMPLETED", "Task approved by user")
}

func (to *TasksDB) RejectTask(ctx context.Context, taskID string, agentID string) error {
	return to.TransitionTask(ctx, taskID, agentID, "PENDING_APPROVAL", "IN_PROGRESS", "Task rejected by user")
}
