package orchestration

import (
    "context"
    "errors"
    "fmt"
    "sync"

    "github.com/google/uuid"
    "github.com/onehumancorp/mono/srcs/server/auth"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type SharedTaskDB struct {
    ID              string
    ParentPlanID    *string
    OrganizationID  string
    Title           string
    Description     *string
    Status          string
    AgentID *string
    CreatedAt       string
    UpdatedAt       string
}

type SharedTaskOrchestrator struct {
    dbProvider db.Provider
    mu         sync.Mutex
}

func NewSharedTaskOrchestrator(dbProvider db.Provider) *SharedTaskOrchestrator {
    return &SharedTaskOrchestrator{
        dbProvider: dbProvider,
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

func (to *SharedTaskOrchestrator) ClaimTask(ctx context.Context, agentID string) (*SharedTaskDB, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil {
        return nil, errors.New("unauthorized: missing claims")
    }

    if to.dbProvider.IsSQLite() {
        return to.claimTaskSQLite(ctx, claims.OrganizationID, agentID)
    }
    return to.claimTaskPostgres(ctx, claims.OrganizationID, agentID)
}

func (to *SharedTaskOrchestrator) claimTaskSQLite(ctx context.Context, orgID, agentID string) (*SharedTaskDB, error) {
    to.mu.Lock()
    defer to.mu.Unlock()

    tx, err := to.dbProvider.Begin(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        SELECT st.id, st.organization_id, st.parent_plan_id, st.title, st.description, st.status, st.agent_id, st.created_at, st.updated_at
        FROM shared_tasks st
        WHERE st.status = 'PENDING' AND st.organization_id = $1
        AND NOT EXISTS (
            SELECT 1 FROM json_each(st.dependencies) AS dep
            JOIN shared_tasks d ON d.id = dep.value
            WHERE d.status != 'COMPLETED'
        )
        LIMIT 1
    `
    row := tx.QueryRow(ctx, query, orgID)

    var task SharedTaskDB
    if err := row.Scan(
        &task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title,
        &task.Description, &task.Status, &task.AgentID, &task.CreatedAt, &task.UpdatedAt,
    ); err != nil {
        if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
            return nil, nil
        }
        return nil, fmt.Errorf("failed to query pending task: %w", err)
    }

    if _, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
        return nil, fmt.Errorf("failed to update task status: %w", err)
    }

    if err := to.insertTransition(ctx, tx, task.ID, "PENDING", "IN_PROGRESS", agentID, "Task claimed by agent"); err != nil {
        return nil, fmt.Errorf("failed to insert transition: %w", err)
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, fmt.Errorf("failed to commit transaction: %w", err)
    }

    task.Status = "IN_PROGRESS"
    task.AgentID = &agentID
    return &task, nil
}

func (to *SharedTaskOrchestrator) claimTaskPostgres(ctx context.Context, orgID, agentID string) (*SharedTaskDB, error) {
    tx, err := to.dbProvider.Begin(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        SELECT st.id, st.organization_id, st.parent_plan_id, st.title, st.description, st.status, st.agent_id, st.created_at, st.updated_at
        FROM shared_tasks st
        WHERE st.status = 'PENDING' AND st.organization_id = $1
        AND NOT EXISTS (
            SELECT 1 FROM jsonb_array_elements_text(st.dependencies) AS dep
            JOIN shared_tasks d ON d.id = dep
            WHERE d.status != 'COMPLETED'
        )
        LIMIT 1
        FOR UPDATE SKIP LOCKED
    `
    row := tx.QueryRow(ctx, query, orgID)

    var task SharedTaskDB
    if err := row.Scan(
        &task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title,
        &task.Description, &task.Status, &task.AgentID, &task.CreatedAt, &task.UpdatedAt,
    ); err != nil {
        if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
            return nil, nil
        }
        return nil, fmt.Errorf("failed to query pending task: %w", err)
    }

    if _, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
        return nil, fmt.Errorf("failed to update task status: %w", err)
    }

    if err := to.insertTransition(ctx, tx, task.ID, "PENDING", "IN_PROGRESS", agentID, "Task claimed by agent"); err != nil {
        return nil, fmt.Errorf("failed to insert transition: %w", err)
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, fmt.Errorf("failed to commit transaction: %w", err)
    }

    task.Status = "IN_PROGRESS"
    task.AgentID = &agentID
    return &task, nil
}

func (to *SharedTaskOrchestrator) TransitionTask(ctx context.Context, taskID, agentID, fromState, toState, reason string) error {
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
