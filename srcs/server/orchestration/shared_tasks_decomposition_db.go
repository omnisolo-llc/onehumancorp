package orchestration

import (
    "context"
    "errors"
    "fmt"
    "sync"
    "time"

    "github.com/onehumancorp/mono/srcs/server/auth"
    "github.com/onehumancorp/mono/srcs/server/db"
)


type SharedTasksDecompositionDB struct {
    ID              string
    ParentPlanID    *string
    OrganizationID  string
    Title           string
    Description     *string
    Status          string
    AgentID         *string
    CreatedAt       time.Time
    UpdatedAt       time.Time
}

type SharedTasksDecompositionRepository struct {
    dbProvider db.Provider
    mu         sync.Mutex
}

func NewSharedTasksDecompositionRepository(dbProvider db.Provider) *SharedTasksDecompositionRepository {
    return &SharedTasksDecompositionRepository{
        dbProvider: dbProvider,
    }
}

func (r *SharedTasksDecompositionRepository) ClaimTask(ctx context.Context, agentID string) (*SharedTasksDecompositionDB, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil {
        return nil, errors.New("unauthorized: missing claims")
    }

    if r.dbProvider.IsSQLite() {
        return r.claimTaskSQLite(ctx, claims.OrganizationID, agentID)
    }
    return r.claimTaskPostgres(ctx, claims.OrganizationID, agentID)
}

func (r *SharedTasksDecompositionRepository) claimTaskSQLite(ctx context.Context, orgID, agentID string) (*SharedTasksDecompositionDB, error) {
    r.mu.Lock()
    defer r.mu.Unlock()

    tx, err := r.dbProvider.Begin(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed to begin tx: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        SELECT std.id, std.organization_id, std.parent_plan_id, std.title, std.description, std.status, std.assigned_agent_id, std.created_at, std.updated_at
        FROM shared_tasks_decomposition std
        WHERE std.status = 'PENDING' AND std.organization_id = $1
        AND NOT EXISTS (
            SELECT 1 FROM json_each(std.dependencies) AS dep
            JOIN shared_tasks_decomposition d ON d.id = dep.value
            WHERE d.status != 'DONE'
        )
        LIMIT 1
    `
    row := tx.QueryRow(ctx, query, orgID)
    var task SharedTasksDecompositionDB
    if err := row.Scan(&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status, &task.AgentID, &task.CreatedAt, &task.UpdatedAt); err != nil {
        if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
            return nil, nil
        }
        return nil, err
    }

    if _, err = tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
        return nil, err
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, err
    }
    task.Status = "IN_PROGRESS"
    task.AgentID = &agentID
    return &task, nil
}

func (r *SharedTasksDecompositionRepository) claimTaskPostgres(ctx context.Context, orgID, agentID string) (*SharedTasksDecompositionDB, error) {
    tx, err := r.dbProvider.Begin(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed to begin tx: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        SELECT std.id, std.organization_id, std.parent_plan_id, std.title, std.description, std.status, std.assigned_agent_id, std.created_at, std.updated_at
        FROM shared_tasks_decomposition std
        WHERE std.status = 'PENDING' AND std.organization_id = $1
        AND NOT EXISTS (
            SELECT 1 FROM jsonb_array_elements_text(std.dependencies) AS dep
            JOIN shared_tasks_decomposition d ON d.id = dep::uuid
            WHERE d.status != 'DONE'
        )
        LIMIT 1
        FOR UPDATE SKIP LOCKED
    `
    row := tx.QueryRow(ctx, query, orgID)
    var task SharedTasksDecompositionDB
    if err := row.Scan(&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status, &task.AgentID, &task.CreatedAt, &task.UpdatedAt); err != nil {
        if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
            return nil, nil
        }
        return nil, err
    }

    if _, err = tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
        return nil, err
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, err
    }
    task.Status = "IN_PROGRESS"
    task.AgentID = &agentID
    return &task, nil
}
